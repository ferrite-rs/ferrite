use serde;

use super::utils::*;
use crate::{base::*, functional::row::*};

pub struct ExternalChoice<Row>
where
  Row : RowCon,
{
  pub(crate) sender : SenderOnce<(
    Value<AppliedSum<Row, ()>>,
    SenderOnce<AppliedSum<Row, ReceiverF>>,
  )>,
}

impl<Row> Protocol for ExternalChoice<Row>
where
  Row : Send + 'static,
  Row : RowCon,
{
}

impl<Row, A> RecApp<A> for ExternalChoice<Row>
where
  Row : RecApp<A>,
  Row : RowCon,
  Row::Applied : RowCon,
{
  type Applied = ExternalChoice<Row::Applied>;
}

impl<Row> ForwardChannel for ExternalChoice<Row>
where
  Row : RowCon,
  AppliedSum<Row, ReceiverF> : ForwardChannel,
  AppliedSum<Row, ()> :
    Send + 'static + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn forward_to(
    self,
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  )
  {

    self.sender.forward_to(sender, receiver)
  }

  fn forward_from(
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  ) -> Self
  {

    ExternalChoice {
      sender : <SenderOnce<(
        Value<AppliedSum<Row, ()>>,
        SenderOnce<AppliedSum<Row, ReceiverF>>,
      )>>::forward_from(sender, receiver),
    }
  }
}
