use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use crate::internal::{
  base::*,
  functional::*,
};

pub struct InternalChoice<Row>(PhantomData<Row>);

impl<Row1, Row2> Protocol for InternalChoice<Row1>
where
  Row1: Send + 'static,
  Row2: Send + 'static,
  Row1: ToRow<Row = Row2>,
{
  type ClientEndpoint = ReceiverOnce<AppSum<'static, Row2, ClientEndpointF>>;
  type ProviderEndpoint = SenderOnce<AppSum<'static, Row2, ClientEndpointF>>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    once_channel()
  }

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let endpoint = client_end.recv().await.unwrap();
      provider_end.send(endpoint).unwrap();
    })
  }
}

impl<Row1, Row2, Row3, A> RecApp<A> for InternalChoice<Row1>
where
  A: Send + 'static,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  Row2: RecApp<A, Applied = Row3>,
  Row3: RowCon,
{
  type Applied = InternalChoice<RecRow<A, Row1>>;
}

impl<Row1, Row2, Row3, A> SharedRecApp<A> for InternalChoice<Row1>
where
  A: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  Row2: SharedRecApp<A, Applied = Row3>,
  Row3: RowCon,
{
  type Applied = InternalChoice<SharedRecRow<A, Row1>>;
}
