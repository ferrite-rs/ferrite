use async_macros::join;
use tokio::task;

use crate::internal::{
  base::{
    once_channel,
    unsafe_create_session,
    unsafe_run_session,
    Context,
    PartialSession,
    Protocol,
    ReceiverF,
  },
  functional::{
    wrap_type_app,
    Prism,
    SumApp,
    ToRow,
  },
  protocol::InternalChoice,
};

pub fn offer_case<N, C, A, Row1, Row2>(
  _: N,
  cont: PartialSession<C, A>,
) -> PartialSession<C, InternalChoice<Row1>>
where
  C: Context,
  A: Protocol,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: SumApp<ReceiverF>,
  N: Prism<Row2, Elem = A>,
{
  unsafe_create_session(move |ctx, sender1| async move {
    let (sender2, receiver2) = once_channel();

    let child1 = task::spawn(async move {
      unsafe_run_session(cont, ctx, sender2).await;
    });

    let child2 = task::spawn(async move {
      sender1
        .send(InternalChoice {
          field: N::inject_elem(wrap_type_app(receiver2)),
        })
        .unwrap();
    });

    let _ = join!(child1, child2).await;
  })
}
