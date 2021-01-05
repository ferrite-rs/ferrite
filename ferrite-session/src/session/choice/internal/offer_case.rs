use async_std::task;
use async_macros::join;

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
  Row : RowApp < ReceiverF >,
  N :
    Prism <
      Row,
      Elem = A
    >,
{
  unsafe_create_session (
    move | ctx, sender1 | async move {
      let (sender2, receiver2) = once_channel();

      let child1 = task::spawn(async move {
        unsafe_run_session(cont, ctx, sender2).await;
      });

      let child2 = task::spawn(async move {
        sender1.send( InternalChoice {
          field : N::inject_elem (
            cloak_applied ( receiver2 ) )
        }).await.unwrap();
      });

      join!(child1, child2).await;
    })
}
