use serde;

use crate::internal::{
  base::*,
  functional::*,
};

pub struct ExternalChoice<Row>
where
  Row: ToRow,
{
  pub(crate) sender: SenderOnce<(
    Value<AppSum<Row::Row, ()>>,
    SenderOnce<AppSum<Row::Row, ReceiverF>>,
  )>,
}

impl<Row> Protocol for ExternalChoice<Row> where Row: ToRow {}

impl<R, Row1, Row2> RecApp<R> for ExternalChoice<Row1>
where
  R: Send + 'static,
  Row2: RowCon,
  Row1: ToRow<Row = Row2>,
  Row2: RecApp<R>,
  Row2::Applied: RowCon,
{
  type Applied = ExternalChoice<LinearRecRow<R, Row2>>;
}

impl<R, Row1, Row2> SharedRecApp<R> for ExternalChoice<Row1>
where
  R: Send + 'static,
  Row2: RowCon,
  Row1: ToRow<Row = Row2>,
  Row2: SharedRecApp<R>,
  Row2::Applied: RowCon,
{
  type Applied = ExternalChoice<SharedRecRow<R, Row2>>;
}

impl<Row1, Row2> ForwardChannel for ExternalChoice<Row1>
where
  Row2: RowCon,
  Row1: ToRow<Row = Row2>,
  AppSum<Row2, ReceiverF>: ForwardChannel,
  AppSum<Row2, ()>:
    Send + 'static + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn forward_to(
    self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    self.sender.forward_to(sender, receiver)
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    ExternalChoice {
      sender: <SenderOnce<(
        Value<AppSum<Row2, ()>>,
        SenderOnce<AppSum<Row2, ReceiverF>>,
      )>>::forward_from(sender, receiver),
    }
  }
}
