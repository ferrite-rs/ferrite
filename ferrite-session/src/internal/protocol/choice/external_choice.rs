use serde;

use crate::internal::{
  base::*,
  functional::*,
};

pub struct ExternalChoice<Row>
where
  Row: RowCon,
{
  pub(crate) sender:
    SenderOnce<(Value<AppSum<Row, ()>>, SenderOnce<AppSum<Row, ReceiverF>>)>,
}

impl<Row> Protocol for ExternalChoice<Row>
where
  Row: Send + 'static,
  Row: RowCon,
{
}

impl<Row, A> RecApp<A> for ExternalChoice<Row>
where
  Row: RecApp<A>,
  Row: RowCon,
  Row::Applied: RowCon,
{
  type Applied = ExternalChoice<Row::Applied>;
}

impl<Row, A> SharedRecApp<A> for ExternalChoice<Row>
where
  Row: SharedRecApp<A>,
  Row: SumApp<()>,
  Row: SumApp<ReceiverF>,
  <Row as SharedRecApp<A>>::Applied: SumApp<()>,
  <Row as SharedRecApp<A>>::Applied: SumApp<ReceiverF>,
{
  type Applied = ExternalChoice<<Row as SharedRecApp<A>>::Applied>;
}

impl<Row> ForwardChannel for ExternalChoice<Row>
where
  Row: RowCon,
  AppSum<Row, ReceiverF>: ForwardChannel,
  AppSum<Row, ()>:
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
        Value<AppSum<Row, ()>>,
        SenderOnce<AppSum<Row, ReceiverF>>,
      )>>::forward_from(sender, receiver),
    }
  }
}
