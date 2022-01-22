use core::{
  future::Future,
  pin::Pin,
};

use super::{
  traits::*,
  types::*,
};
use crate::internal::functional::{
  nat::*,
  type_app::{
    TyCon,
    TypeApp,
  },
};

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
