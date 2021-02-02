use std::marker::PhantomData;

use crate::base::*;

pub struct SharedToLinear < F >
( pub (crate) PhantomData < F > );

impl < F > Protocol
  for SharedToLinear < F >
where
  F : Protocol
{ }


impl <F> ForwardChannel
  for SharedToLinear < F >
where
  F: Send + 'static,
{
  fn forward_to(self,
    sender: OpaqueSender,
    _: OpaqueReceiver,
  )
  {
    sender.send(())
  }

  fn forward_from(
    _: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    let () = receiver.recv().unwrap();
    SharedToLinear(PhantomData)
  }
}
