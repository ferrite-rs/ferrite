use std::marker::PhantomData;

use crate::internal::base::*;

pub struct SharedToLinear<F>
{
  pub(crate) unlock: SenderOnce<()>,
  pub(crate) phantom: PhantomData<F>,
}

impl<F> Protocol for SharedToLinear<F> where F: Send + 'static {}

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
