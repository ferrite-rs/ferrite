use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use crate::internal::{
  base::*,
  functional::*,
};

pub struct ExternalChoice<Row>(PhantomData<Row>);

impl<Row> SealedProtocol for ExternalChoice<Row> {}

impl<Row> Protocol for ExternalChoice<Row>
where
  Row: Send + 'static,
  Row: ToRow,
{
  type ClientEndpoint =
    SenderOnce<AppSum<'static, Row::Row, ProviderEndpointF>>;
  type ProviderEndpoint =
    ReceiverOnce<AppSum<'static, Row::Row, ProviderEndpointF>>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    let (sender, receiver) = once_channel();

    (receiver, sender)
  }

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let payload = provider_end.recv().await.unwrap();
      client_end.send(payload).unwrap();
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
