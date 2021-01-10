
use crate::base::*;

use ipc_channel::ipc;

pub struct End ();

impl Protocol for End { }

impl < A >
  RecApp < A >
  for End
{
  type Applied = End;
}

impl ForwardChannel
  for End
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
    End()
  }
}
