use super::types::{
  OpaqueReceiver,
  OpaqueSender,
};

pub trait ForwardChannel: Send + 'static
{
  fn forward_to(
    self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  );

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self;
}
