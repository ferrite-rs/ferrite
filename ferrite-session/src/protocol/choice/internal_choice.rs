use super::utils::*;

use crate::base::*;
use crate::functional::row::*;

use ipc_channel::ipc;

pub struct InternalChoice < Row >
where
  Row : RowCon,
{ pub (crate) field :
    AppliedSum < Row, ReceiverF >
}

impl < Row >
  Protocol for
  InternalChoice < Row >
where
  Row : Send + 'static,
  Row : RowCon,
{ }

impl < Row, A >
  RecApp < A > for
  InternalChoice < Row >
where
  Row : RowCon,
  Row : RecApp < A >,
  Row::Applied : RowCon,
{
  type Applied =
    InternalChoice <
      Row::Applied
    >;
}

impl < Row >
  ForwardChannel
  for InternalChoice < Row >
where
  Row: RowCon,
  AppliedSum < Row, ReceiverF >: ForwardChannel,
{
  fn forward_to(self,
    sender: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
  )
  {
    self.field.forward_to(sender, receiver)
  }

  fn forward_from(
    sender: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    InternalChoice {
      field: <
        AppliedSum < Row, ReceiverF >
      >::forward_from(sender, receiver)
    }
  }
}
