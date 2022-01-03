use std::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use super::super::internal_session::*;
use crate::internal::{
  base::*,
  functional::*,
  protocol::*,
};

pub async fn run_case_cont<N, C, D, B, Row1, Row2>(
  ctx: D::Endpoints,
  provider_end_b: ProviderEndpoint<B>,
  cont1: AppSum<
    'static,
    Row2,
    Merge<ConsumerEndpointF, InternalSessionF<N, C, B, Row1, D>>,
  >,
) where
  C: Context,
  D: Context,
  B: Protocol,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: ElimSum,
  N: ContextLens<C, InternalChoice<Row1>, Empty, Deleted = D>,
{
  let cont2 = ContRunner1::<N, C, B, Row1, D> {
    ctx,
    provider_end_b,
    phantom: PhantomData,
  };

  Row2::elim_sum(cont2, cont1).await;
}

struct ContRunner1<N, C, B, Row, D>
where
  B: Protocol,
  C: Context,
  D: Context,
  Row: ToRow,
  Row: Send + 'static,
  Row::Row: Send + 'static,
  N: ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  ctx: D::Endpoints,
  provider_end_b: ProviderEndpoint<B>,
  phantom: PhantomData<(N, C, Row)>,
}

struct ContRunner2<N, C, A, B, Row, D>
where
  B: Protocol,
  C: Context,
  D: Context,
  Row: ToRow,
  Row: Send + 'static,
  Row::Row: Send + 'static,
  N: ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  ctx: D::Endpoints,

  provider_end_b: ProviderEndpoint<B>,

  consumer_end_a: ConsumerEndpoint<A>,

  phantom: PhantomData<(N, C, Row)>,
}

impl<N, C, A, B, Row, D>
  NeedInternalSession<
    N,
    C,
    A,
    B,
    Row,
    D,
    Pin<Box<dyn Future<Output = ()> + Send>>,
  > for ContRunner2<N, C, A, B, Row, D>
where
  B: Protocol,
  C: Context,
  D: Context,
  Row: ToRow,
  Row: Send + 'static,
  Row::Row: Send + 'static,
  N: ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  fn on_internal_session(
    self: Box<Self>,
    cont: InternalSession<N, C, A, B, Row, D>,
  ) -> Pin<Box<dyn Future<Output = ()> + Send>>
  where
    A: Protocol,
    B: Protocol,
    C: Context,
    N: ContextLens<C, InternalChoice<Row>, A, Deleted = D>,
  {
    Box::pin(async move {
      let ctx1 = self.ctx;

      let ctx2 = <N as ContextLens<C, InternalChoice<Row>, A>>::insert_target(
        self.consumer_end_a,
        ctx1,
      );

      unsafe_run_session(cont.session, ctx2, self.provider_end_b.get_applied())
        .await;
    })
  }
}

impl<B, N, C, Row, D>
  ElimField<
    'static,
    Merge<ConsumerEndpointF, InternalSessionF<N, C, B, Row, D>>,
    Pin<Box<dyn Future<Output = ()> + Send>>,
  > for ContRunner1<N, C, B, Row, D>
where
  B: Protocol,
  C: Context,
  D: Context,
  Row: ToRow,
  Row: Send + 'static,
  Row::Row: Send + 'static,
  N: ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  fn elim_field<A: 'static>(
    self,
    fa: App<
      'static,
      Merge<ConsumerEndpointF, InternalSessionF<N, C, B, Row, D>>,
      A,
    >,
  ) -> Pin<Box<dyn Future<Output = ()> + Send>>
  {
    let (consumer_end_a, session1) = fa.get_applied();

    let session2 = session1.get_applied();

    let ContRunner1 {
      ctx,
      provider_end_b,
      ..
    } = self;

    let cont = ContRunner2::<N, C, A, B, Row, D> {
      ctx,
      consumer_end_a,
      provider_end_b,
      phantom: PhantomData,
    };

    *with_internal_session(session2, Box::new(cont))
  }
}
