
use crate::base::*;

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
    End()
  }
}
