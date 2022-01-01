use core::{
  future::Future,
  pin::Pin,
};

use crate::internal::{
  base::{
    channel::{
      once_channel,
      ReceiverOnce,
      SenderOnce,
    },
    rec::{
      RecApp,
      RecX,
      Release,
    },
  },
  functional::{
    nat::*,
    row::*,
    type_app::{
      App,
      TyCon,
      TypeApp,
    },
  },
};

pub trait Protocol: Send + 'static
{
  type ProviderEndpoint: Send + 'static;
  type ConsumerEndpoint: Send + 'static;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint);

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

pub trait SharedProtocol: Send + 'static {}

pub trait ProtocolRow: RowCon
{
  fn create_row_endpoints() -> (
    AppSum<Self, ProviderEndpointF>,
    AppSum<Self, ConsumerEndpointF>,
  );
}

pub struct ProviderEndpointF;

pub struct ConsumerEndpointF;

pub type ProviderEndpoint<A> = App<ProviderEndpointF, A>;

pub type ConsumerEndpoint<A> = App<ConsumerEndpointF, A>;

impl TyCon for ProviderEndpointF {}

impl TyCon for ConsumerEndpointF {}

impl<A: Protocol> TypeApp<A> for ProviderEndpointF
{
  type Applied = A::ProviderEndpoint;
}

impl<A: Protocol> TypeApp<A> for ConsumerEndpointF
{
  type Applied = A::ConsumerEndpoint;
}

pub trait HasRecEndpoint<F, C>: Send + 'static
{
  fn get_applied(
    self: Box<Self>
  ) -> Box<<F::Applied as Protocol>::ConsumerEndpoint>
  where
    F: RecApp<C>,
    F::Applied: Protocol;
}

impl<F, C, E> HasRecEndpoint<F, C> for E
where
  E: Send + 'static,
  F: RecApp<C>,
  F::Applied: Protocol<ConsumerEndpoint = E>,
{
  fn get_applied(self: Box<Self>) -> Box<Self>
  {
    self
  }
}

pub struct RecEndpoint<F, C>
{
  pub applied: Box<dyn HasRecEndpoint<F, C>>,
}

impl<C, F> Protocol for RecX<C, F>
where
  C: Send + 'static,
  F: Protocol,
{
  type ConsumerEndpoint = ReceiverOnce<RecEndpoint<F, (RecX<C, F>, C)>>;
  type ProviderEndpoint = SenderOnce<RecEndpoint<F, (RecX<C, F>, C)>>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    once_channel()
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let endpoint = consumer_end.recv().await.unwrap();
      provider_end.send(endpoint).unwrap();
    })
  }
}

impl Protocol for Release
{
  type ConsumerEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    ((), ())
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {})
  }
}

impl Protocol for Z
{
  type ConsumerEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    ((), ())
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {})
  }
}

impl<N> Protocol for S<N>
where
  N: Protocol,
{
  type ConsumerEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    ((), ())
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {})
  }
}
