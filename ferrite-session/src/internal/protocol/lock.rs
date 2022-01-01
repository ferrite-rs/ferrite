use core::{
  future::Future,
  pin::Pin,
};

use super::{
  linear_to_shared::LinearToShared,
  shared_to_linear::SharedToLinear,
};
use crate::internal::base::*;

pub struct Lock<F>
where
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
{
  pub(crate) unlock: Receiver<(SenderOnce<()>, SenderOnce<LinearToShared<F>>)>,
}

impl<F> Protocol for Lock<F>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  type ConsumerEndpoint = ReceiverOnce<Lock<F>>;
  type ProviderEndpoint = SenderOnce<Lock<F>>;

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
