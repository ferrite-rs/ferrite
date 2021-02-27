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
    cloak_applied,
    Prism,
    SumApp,
  },
  protocol::InternalChoice,
};

pub fn offer_case<N, C, A, Row>(
  _ : N,
  cont : PartialSession<C, A>,
) -> PartialSession<C, InternalChoice<Row>>
where
  C : Context,
  A : Protocol,
  Row : SumApp<ReceiverF>,
  N : Prism<Row, Elem = A>,
{
  unsafe_create_session(move |ctx, sender1| async move {
    let (sender2, receiver2) = once_channel();

    let child1 = task::spawn(async move {
      unsafe_run_session(cont, ctx, sender2).await;
    });

    let child2 = task::spawn(async move {
      sender1
        .send(InternalChoice {
          field : N::inject_elem(cloak_applied(receiver2)),
        })
        .unwrap();
    });

    let _ = join!(child1, child2).await;
  })
}
