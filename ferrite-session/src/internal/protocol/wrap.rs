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
  fn unwrap(self: Box<Self>) -> <T::Unwrap as Protocol>::ConsumerEndpoint
  where
    T: Wrapper,
    T::Unwrap: Protocol;
}

impl<T, W> HasWrapped<T> for W::ConsumerEndpoint
where
  T: Wrapper<Unwrap = W>,
  W: Protocol,
{
  fn unwrap(self: Box<Self>) -> <T::Unwrap as Protocol>::ConsumerEndpoint
  {
    *self
  }
}

impl<T> Protocol for Wrap<T>
where
  T: Wrapper,
  T: Send + 'static,
{
  type ConsumerEndpoint = ReceiverOnce<Wrap<T>>;
  type ProviderEndpoint = SenderOnce<Wrap<T>>;

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
      let payload = consumer_end.recv().await.unwrap();
      provider_end.send(payload).unwrap();
    })
  }
}
