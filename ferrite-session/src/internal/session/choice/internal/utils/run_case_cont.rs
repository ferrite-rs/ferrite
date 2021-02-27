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

pub async fn run_case_cont<N, C, D, B, Row>(
  ctx : D::Endpoints,
  sender : SenderOnce<B>,
  cont1 : AppSum<Row, Merge<ReceiverF, InternalSessionF<N, C, B, Row, D>>>,
) where
  C : Context,
  D : Context,
  B : Protocol,
  Row : ElimSum,
  N : ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  let cont2 = ContRunner1::<N, C, B, Row, D> {
    ctx,
    sender,
    phantom : PhantomData,
  };

  Row::elim_sum(cont2, cont1).await;
}

struct ContRunner1<N, C, B, Row, D>
where
  B : Protocol,
  C : Context,
  D : Context,
  Row : RowCon,
  N : ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  ctx : D::Endpoints,
  sender : SenderOnce<B>,
  phantom : PhantomData<(N, C, Row)>,
}

struct ContRunner2<N, C, A, B, Row, D>
where
  B : Protocol,
  C : Context,
  D : Context,
  Row : RowCon,
  N : ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  ctx : D::Endpoints,

  sender : SenderOnce<B>,

  receiver : ReceiverOnce<A>,

  phantom : PhantomData<(N, C, Row)>,
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
  B : Protocol,
  C : Context,
  D : Context,
  Row : RowCon,
  N : 'static,
  N : ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  fn on_internal_session(
    self: Box<Self>,
    cont : InternalSession<N, C, A, B, Row, D>,
  ) -> Pin<Box<dyn Future<Output = ()> + Send>>
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Row : RowCon,
    N : ContextLens<C, InternalChoice<Row>, A, Deleted = D>,
  {
    let ctx1 = self.ctx;

    let sender = self.sender;

    let receiver = self.receiver;

    let ctx2 = <N as ContextLens<C, InternalChoice<Row>, A>>::insert_target(
      receiver, ctx1,
    );

    Box::pin(async move {
      unsafe_run_session(cont.session, ctx2, sender).await;
    })
  }
}

impl<B, N, C, Row, D>
  ElimField<
    Merge<ReceiverF, InternalSessionF<N, C, B, Row, D>>,
    Pin<Box<dyn Future<Output = ()> + Send>>,
  > for ContRunner1<N, C, B, Row, D>
where
  B : Protocol,
  C : Context,
  D : Context,
  Row : RowCon,
  N : ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
{
  fn elim_field<A>(
    self,
    fa : App<Merge<ReceiverF, InternalSessionF<N, C, B, Row, D>>, A>,
  ) -> Pin<Box<dyn Future<Output = ()> + Send>>
  where
    A : Send + 'static,
  {
    let (receiver1, session1) = fa.get_applied();

    let receiver2 = receiver1.get_applied();

    let session2 = session1.get_applied();

    let ContRunner1 { ctx, sender, .. } = self;

    let cont = ContRunner2::<N, C, A, B, Row, D> {
      ctx,
      sender,
      receiver : receiver2,
      phantom : PhantomData,
    };

    *with_internal_session(session2, Box::new(cont))
  }
}
