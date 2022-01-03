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
  type ConsumerEndpoint =
    ReceiverOnce<AppSum<'static, Row2, ConsumerEndpointF>>;
  type ProviderEndpoint = SenderOnce<AppSum<'static, Row2, ConsumerEndpointF>>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    once_channel()
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let endpoint = consumer_end.recv().await.unwrap();
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
