use core::{
  future::Future,
  pin::Pin,
};

use crate::internal::base::*;

pub trait Wrapper
{
  type Unwrap: Protocol;
}

pub struct Wrap<T>
where
  T: Wrapper,
{
  pub(crate) unwrap: Box<dyn HasWrapped<T>>,
}

pub trait HasWrapped<T>: Send + 'static
{
  fn unwrap(self: Box<Self>) -> <T::Unwrap as Protocol>::ClientEndpoint
  where
    T: Wrapper,
    T::Unwrap: Protocol;
}

impl<T, W> HasWrapped<T> for W::ClientEndpoint
where
  T: Wrapper<Unwrap = W>,
  W: Protocol,
{
  fn unwrap(self: Box<Self>) -> <T::Unwrap as Protocol>::ClientEndpoint
  {
    *self
  }
}

impl<T> Protocol for Wrap<T>
where
  T: Wrapper,
  T: Send + 'static,
{
  type ClientEndpoint = ReceiverOnce<Wrap<T>>;
  type ProviderEndpoint = SenderOnce<Wrap<T>>;

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
      let payload = client_end.recv().await.unwrap();
      provider_end.send(payload).unwrap();
    })
  }
}
