use core::{
  future::Future,
  pin::Pin,
};
use std::marker::PhantomData;

use super::linear_to_shared::LinearToShared;
use crate::internal::base::*;

pub struct SharedToLinear<F>(PhantomData<F>);

impl<F> Protocol for SharedToLinear<LinearToShared<F>>
where
  F: Send + 'static,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
{
  type ConsumerEndpoint = SenderOnce<()>;
  type ProviderEndpoint = ReceiverOnce<()>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    let (sender, receiver) = once_channel();
    (receiver, sender)
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let payload = provider_end.recv().await.unwrap();
      consumer_end.send(payload).unwrap();
    })
  }
}
