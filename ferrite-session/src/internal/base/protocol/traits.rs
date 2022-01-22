use core::{
  future::Future,
  pin::Pin,
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
