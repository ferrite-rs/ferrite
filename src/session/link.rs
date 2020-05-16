use async_macros::join;
use async_std::task;
use async_std::sync::{ channel };

use crate::base::{
  Protocol,
  AppendContext,
  Context,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

/*
  Cut (Communication)

    cont1 :: Δ1, Q, Δ2 ⊢ P    cont2 :: Δ3 ⊢ Q
  ==============================================
       link(cont1, cont2) :: Δ1, Δ2, Δ3 ⊢ P
 */

pub fn cut
  < C1, C2, C3, C4, A, B >
  ( cont1 : PartialSession < C3, B >,
    cont2 : PartialSession < C2, A >
  ) ->
    PartialSession < C4, B >
where
  A : Protocol,
  B : Protocol,
  C1 : Context,
  C2 : Context,
  C3 : Context,
  C4 : Context,
  C1 :
    AppendContext <
      (A, ()),
      Appended = C3
    >,
  C1 :
    AppendContext <
      C2,
      Appended = C4
    >,
{
  unsafe_create_session (
    async move | ctx1, b_sender | {
      let (ctx2, ctx3) =
        < C1 as
          AppendContext < C2 >
        > :: split_context (ctx1);

      let (a_sender, a_receiver) = channel(1);

      let ctx4 =
        < C1 as
          AppendContext < (A, ()) >
        > :: append_context ( ctx2, (a_receiver, ()) );

      let child1 = task::spawn(async {
        unsafe_run_session
          ( cont1, ctx4, b_sender
          ).await;
      });

      let child2 = task::spawn(async {
        unsafe_run_session
          ( cont2, ctx3, a_sender
          ).await;
      });

      join!(child1, child2).await;
    })
}
