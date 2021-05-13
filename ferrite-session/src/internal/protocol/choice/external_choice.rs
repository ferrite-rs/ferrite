use serde;

use crate::internal::{
  base::*,
  functional::*,
};

pub struct ExternalChoiceX<R, Row>
where
  Row: RowCon,
{
  pub(crate) payload: Box<dyn HasExternalChoice<R, Row>>,
}

pub struct ExternalChoice<Row>
where
  Row: RowCon,
{
  pub(crate) sender:
    SenderOnce<(Value<AppSum<Row, ()>>, SenderOnce<AppSum<Row, ReceiverF>>)>,
}

pub trait HasExternalChoice<R, Row>: Send + 'static
{
  fn get_payload(self: Box<Self>) -> ExternalChoice<Row::Applied>
  where
    Row: SharedRecApp<R>,
    Row::Applied: RowCon;
}

impl<R, Row> HasExternalChoice<R, Row> for ExternalChoice<Row::Applied>
where
  Row: SharedRecApp<R>,
  Row::Applied: RowCon,
{
  fn get_payload(self: Box<Self>) -> ExternalChoice<Row::Applied>
  {
    *self
  }
}

impl<Row> Protocol for ExternalChoice<Row> where Row: RowCon {}

impl<R, Row> Protocol for ExternalChoiceX<R, Row>
where
  R: Send + 'static,
  Row: RowCon,
{
}

impl<R, Row> Fixed for ExternalChoiceX<R, Row>
where
  R: 'static,
  Row: RowCon,
  Row: SharedRecApp<R>,
  Row::Applied: RowCon,
{
  type Unfixed = ExternalChoice<Row::Applied>;

  fn unfix(self) -> Self::Unfixed
  {
    self.payload.get_payload()
  }

  fn fix(unfixed: Self::Unfixed) -> Self
  {
    ExternalChoiceX {
      payload: Box::new(unfixed),
    }
  }
}

impl<Row> Fixed for ExternalChoice<Row>
where
  Row: RowCon,
{
  type Unfixed = Self;

  fn unfix(self) -> Self
  {
    self
  }

  fn fix(unfixed: Self) -> Self
  {
    unfixed
  }
}

impl<Row, R> RecApp<R> for ExternalChoice<Row>
where
  R: 'static,
  Row: RecApp<R>,
  Row: RowCon,
  Row::Applied: RowCon,
{
  type Applied = ExternalChoice<Row::Applied>;
}

impl<Row, A> SharedRecApp<A> for ExternalChoice<Row>
where
  Row: RowCon,
  Row: SharedRecApp<A>,
  Row::Applied: RowCon,
{
  type Applied = ExternalChoiceX<A, Row>;
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

impl<R, Row1, Row2> ForwardChannel for ExternalChoiceX<R, Row1>
where
  R: 'static,
  Row1: RowCon,
  Row1: SharedRecApp<R, Applied = Row2>,
  Row2: RowCon,
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
    self.payload.get_payload().forward_to(sender, receiver)
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    let payload = <ExternalChoice<Row2>>::forward_from(sender, receiver);
    ExternalChoiceX {
      payload: Box::new(payload),
    }
  }
}
