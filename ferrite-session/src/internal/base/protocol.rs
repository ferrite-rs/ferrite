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
  type ClientEndpoint: Send + 'static;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint);

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

pub trait SharedProtocol: Send + 'static {}

pub trait ProtocolRow: RowCon
{
  fn create_row_endpoints() -> (
    AppSum<'static, Self, ProviderEndpointF>,
    AppSum<'static, Self, ClientEndpointF>,
  );
}

pub struct ProviderEndpointF;

pub struct ClientEndpointF;

pub type ProviderEndpoint<A> = App<'static, ProviderEndpointF, A>;

pub type ClientEndpoint<A> = App<'static, ClientEndpointF, A>;

impl TyCon for ProviderEndpointF {}

impl TyCon for ClientEndpointF {}

impl<'a, A: Protocol> TypeApp<'a, A> for ProviderEndpointF
{
  type Applied = A::ProviderEndpoint;
}

impl<'a, A: Protocol> TypeApp<'a, A> for ClientEndpointF
{
  type Applied = A::ClientEndpoint;
}

pub trait HasRecEndpoint<F, C>: Send + 'static
{
  fn get_applied(
    self: Box<Self>
  ) -> Box<<F::Applied as Protocol>::ClientEndpoint>
  where
    F: RecApp<C>,
    F::Applied: Protocol;
}

impl<F, C, E> HasRecEndpoint<F, C> for E
where
  E: Send + 'static,
  F: RecApp<C>,
  F::Applied: Protocol<ClientEndpoint = E>,
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
  F: RecApp<(RecX<C, F>, C)>,
{
  type ClientEndpoint = ReceiverOnce<RecEndpoint<F, (RecX<C, F>, C)>>;
  type ProviderEndpoint = SenderOnce<RecEndpoint<F, (RecX<C, F>, C)>>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    once_channel()
  }

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let endpoint = client_end.recv().await.unwrap();
      provider_end.send(endpoint).unwrap();
    })
  }
}

impl Protocol for Release
{
  type ClientEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    ((), ())
  }

  fn forward(
    _client_end: Self::ClientEndpoint,
    _provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {})
  }
}

impl Protocol for Z
{
  type ClientEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    ((), ())
  }

  fn forward(
    _client_end: Self::ClientEndpoint,
    _provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {})
  }
}

impl<N> Protocol for S<N>
where
  N: Protocol,
{
  type ClientEndpoint = ();
  type ProviderEndpoint = ();

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    ((), ())
  }

  fn forward(
    _client_end: Self::ClientEndpoint,
    _provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {})
  }
}
