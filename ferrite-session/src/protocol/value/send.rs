use serde;

use crate::base::*;

pub struct SendValue < T, A >
( pub (crate) ( Value<T>, ReceiverOnce < A > )
);

impl < T, P > Protocol for SendValue < T, P >
where
  T : Send + 'static,
  P : Protocol
{ }

impl < X, T, A >
  RecApp < X > for
  SendValue < T, A >
where
  T : Send + 'static,
  A : RecApp < X >,
{
  type Applied =
    SendValue <
      T,
      A :: Applied
    >;
}

impl < T, A >
  ForwardChannel
  for SendValue < T, A >
where
  A: ForwardChannel,
  T: Send + 'static,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn forward_to(self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    self.0.forward_to(sender, receiver)
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    SendValue(
      < ( Value<T>, ReceiverOnce < A > )
      > :: forward_from(sender, receiver)
    )
  }
}
