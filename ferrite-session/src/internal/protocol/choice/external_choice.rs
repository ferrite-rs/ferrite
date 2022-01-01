use core::{
  future::Future,
  pin::Pin,
};

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

impl<Row> Protocol for ExternalChoice<Row>
where
  Row: Send + 'static,
  Row: ToRow,
{
  type ConsumerEndpoint = (
    SenderOnce<Value<AppSum<Row::Row, ()>>>,
    ReceiverOnce<AppSum<Row::Row, ConsumerEndpointF>>,
  );
  type ProviderEndpoint = (
    ReceiverOnce<Value<AppSum<Row::Row, ()>>>,
    SenderOnce<AppSum<Row::Row, ConsumerEndpointF>>,
  );

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    let (choice_sender, choice_receiver) = once_channel();
    let (endpoint_sender, endpoint_receiver) = once_channel();

    (
      (choice_receiver, endpoint_sender),
      (choice_sender, endpoint_receiver),
    )
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    let (choice_sender, endpoint_receiver) = consumer_end;
    let (choice_receiver, endpoint_sender) = provider_end;

    Box::pin(async {
      let choice = choice_receiver.recv().await.unwrap();
      choice_sender.send(choice).unwrap();

      let endpoint = endpoint_receiver.recv().await.unwrap();
      endpoint_sender.send(endpoint).unwrap();
    })
  }
}

impl<R, Row1, Row2, Row3> RecApp<R> for ExternalChoice<Row1>
where
  R: Send + 'static,
  Row2: RowCon,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RecApp<R, Applied = Row3>,
  Row3: RowCon,
{
  type Applied = ExternalChoice<RecRow<R, Row1>>;
}

impl<R, Row1, Row2, Row3> SharedRecApp<R> for ExternalChoice<Row1>
where
  R: Send + 'static,
  Row2: RowCon,
  Row1: ToRow<Row = Row2>,
  Row2: SharedRecApp<R, Applied = Row3>,
  Row3: RowCon,
{
  type Applied = ExternalChoice<SharedRecRow<R, Row1>>;
}

impl<Row1, Row2> ForwardChannel for ExternalChoice<Row1>
where
  Row2: RowCon,
  Row1: Send + 'static,
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
