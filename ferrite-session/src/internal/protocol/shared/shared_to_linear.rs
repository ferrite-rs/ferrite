use core::{
  future::Future,
  pin::Pin,
};
use std::marker::PhantomData;

use super::linear_to_shared::LinearToShared;
use crate::internal::base::*;

pub struct SharedToLinear<F>(PhantomData<F>);

impl<F> SealedProtocol for SharedToLinear<LinearToShared<F>> {}

impl<F> Protocol for SharedToLinear<LinearToShared<F>>
where
  F: Send + 'static,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
{
  type ClientEndpoint = SenderOnce<()>;
  type ProviderEndpoint = ReceiverOnce<()>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    let (sender, receiver) = once_channel();
    (receiver, sender)
  }

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let payload = provider_end.recv().await.unwrap();
      client_end.send(payload).unwrap();
    })
  }
}
