use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };

use crate::base::*;
use crate::protocol::*;
use crate::functional::*;

pub fn offer_case
  < N, C, A, Row >
  ( _ : N,
    cont : PartialSession < C, A >
  ) ->
    PartialSession < C, InternalChoice < Row > >
where
  C : Context,
  A : Protocol,
  Row : SumRow < ReceiverApp >,
  N :
    Prism <
      Row,
      Elem = A
    >,
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver2) = channel(1);

      let child1 = task::spawn(async move {
        unsafe_run_session(cont, ctx, sender2).await;
      });

      let child2 = task::spawn(async move {
        sender1.send( InternalChoice {
          field : N::inject_elem (
            wrap_applied ( receiver2 ) )
        }).await;
      });

      join!(child1, child2).await;
    })
}
