use core::{
  future::Future,
  pin::Pin,
};

use crate::internal::functional::{
  nat::*,
  type_app::{
    App,
    TyCon,
    TypeApp,
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
