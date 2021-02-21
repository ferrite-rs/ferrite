use std::marker::PhantomData;

use crate::base::*;

pub struct SharedToLinear < F >
{
  // pub unlock: SenderOnce<()>,
  pub phantom: PhantomData<F>,
}

impl < F > Protocol
  for SharedToLinear < F >
where
  F : Protocol
{}


impl <F> ForwardChannel
  for SharedToLinear < F >
where
  F: Send + 'static,
{
  fn forward_to(self,
    _sender: OpaqueSender,
    _receiver: OpaqueReceiver,
  )
  {
    // self.unlock.forward_to(sender, receiver);
  }

  fn forward_from(
    _sender: OpaqueSender,
    _receiver: OpaqueReceiver,
  ) -> Self
  {
    // let unlock = <SenderOnce<()>>::forward_from(sender, receiver);

    SharedToLinear {
      // unlock,
      phantom: PhantomData
    }
  }
}
