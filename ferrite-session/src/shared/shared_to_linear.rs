use ipc_channel::ipc;
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
    sender: ipc::OpaqueIpcSender,
    _: ipc::OpaqueIpcReceiver,
  )
  {
    sender.to().send(()).unwrap()
  }

  fn forward_from(
    _: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    let () = receiver.to().recv().unwrap();
    SharedToLinear(PhantomData)
  }
}
