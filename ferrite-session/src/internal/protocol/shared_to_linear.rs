use std::marker::PhantomData;

use super::linear_to_shared::LinearToShared;
use crate::internal::base::*;

pub struct SharedToLinear<F>
{
  pub(crate) unlock: SenderOnce<()>,
  pub(crate) phantom: PhantomData<F>,
}

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
}

impl<F> ForwardChannel for SharedToLinear<F>
where
  F: Send + 'static,
{
  fn forward_to(
    self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    self.unlock.forward_to(sender, receiver);
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    let unlock = <SenderOnce<()>>::forward_from(sender, receiver);

    SharedToLinear {
      unlock,
      phantom: PhantomData,
    }
  }
}
