use async_macros::join;
use async_std::task;
use async_std::sync::{ channel };

use crate::base::{
  Process,
  Session,
  Inactive,
  Processes,
  ProcessLens,
  PartialSession,
  run_partial_session,
  create_partial_session,
};

/*
  Cut (Communication)

    cont1 :: Δ1, Q, Δ2 ⊢ P    cont2 :: Δ3 ⊢ Q
  ==============================================
       link(cont1, cont2) :: Δ1, Δ2, Δ3 ⊢ P
 */

pub fn link
  < I, P, Q,
    Lens
  >
  ( _ : Lens,
    cont1 :
      Session < Q >,
    cont2 :
      PartialSession <
        Lens :: Target,
        P
      >
  ) ->
    PartialSession < I, P >
where
  P : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static,
  Lens :
    ProcessLens <
      I,
      Inactive,
      Q
    >
{
  create_partial_session (
    async move | ins1, p_sender | {
      let (q_sender, q_receiver) = channel(1);

      let (_, ins2) =
        Lens :: split_channels (ins1);

      let ins3 =
        Lens :: merge_channels (q_receiver, ins2);

      let child1 = task::spawn(async {
        run_partial_session
          ( cont1, (), q_sender
          ).await;
      });

      let child2 = task::spawn(async {
        run_partial_session
          ( cont2, ins3, p_sender
          ).await;
      });

      join!(child1, child2).await;
    })
}
