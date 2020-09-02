use async_std::task;
use async_macros::join;
use async_std::sync::{ Receiver, channel };

pub use crate::base::{
  Nat,
  Z,
  Empty,
  TypeApp,
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

pub use crate::context::*;
pub use crate::protocol::choice2::*;

pub fn offer_case
  < N, C, A, Row, Canon >
  ( _ : N,
    cont : PartialSession < C, A >
  ) ->
    PartialSession < C, InternalChoice < Row > >
where
  C : Context,
  A : Protocol,
  Row : Send + 'static,
  Row : Iso < Canon = Canon >,
  Canon : SumRow < ReceiverCon >,
  N :
    IntroSum <
      Canon,
      ReceiverCon,
      Elem = Receiver < A >
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
          field : N::intro_sum ( receiver2 )
        }).await;
      });

      join!(child1, child2).await;
    })
}
