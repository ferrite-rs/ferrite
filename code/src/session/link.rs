use async_macros::join;
use async_std::task;
use async_std::sync::{ channel };

use crate::base::{
  Protocol,
  AppendContext,
  Empty,
  Context,
  ContextLens,
  PartialSession,
  run_partial_session,
  unsafe_create_session,
};

/*
  Cut (Communication)

    cont1 :: Δ1, Q, Δ2 ⊢ P    cont2 :: Δ3 ⊢ Q
  ==============================================
       link(cont1, cont2) :: Δ1, Δ2, Δ3 ⊢ P
 */

pub fn cut
  < N, C1, C2, A, B >
  ( _ : N,
    cont1 :
      PartialSession < C1, A >,
    cont2 :
      PartialSession <
        N :: Target,
        B
      >
  ) ->
    PartialSession < C1::AppendResult, B >
where
  A : Protocol,
  B : Protocol,
  C1 : Context,
  C2 : Context,
  C1 : AppendContext < N::Deleted >,
  N :
    ContextLens <
      C2,
      Empty,
      A
    >
{
  unsafe_create_session (
    async move | ins1, b_sender | {
      let (ins2, ins3) =
        C1 :: split_channels (ins1);

      let (a_sender, a_receiver) = channel(1);

      let ins4 =
        N :: merge_channels ( a_receiver, ins3 );

      let child1 = task::spawn(async {
        run_partial_session
          ( cont1, ins2, a_sender
          ).await;
      });

      let child2 = task::spawn(async {
        run_partial_session
          ( cont2, ins4, b_sender
          ).await;
      });

      join!(child1, child2).await;
    })
}
