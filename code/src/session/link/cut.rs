use crate::base::*;
use async_macros::join;
use async_std::task;
use async_std::sync::{ channel };

/*
  Cut (Communication)

    cont1 :: Δ1, Q, Δ2 ⊢ P    cont2 :: Δ3 ⊢ Q
  ==============================================
       link(cont1, cont2) :: Δ1, Δ2, Δ3 ⊢ P
 */

pub fn link
  < S, T, D,
    P, Q,
    Lens
  >
  ( _ : Lens,
    cont1 :
      Session < Q >,
    cont2 :
      PartialSession <
        T,
        P
      >
  ) ->
    PartialSession < S, P >
where
  P : Process + 'static,
  Q : Process + 'static,
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  Lens :
    ProcessLens <
      S, T, D,
      Inactive,
      Q
    >
{
  PartialSession {
    builder: Box::new ( move | ins1, p_sender | {
      Box::pin(async {
        let (q_sender, q_receiver) = channel(1);

        let (_, ins2) =
          < Lens as
            ProcessLens <
              S, T, D,
              Inactive,
              Q
            >
          > :: split_channels (ins1);

        let ins3 =
          < Lens as
            ProcessLens <
              S, T, D,
              Inactive,
              Q
            >
          > :: merge_channels (q_receiver, ins2);

        let child1 = task::spawn(async {
          (cont1.builder)((), q_sender).await;
        });

        let child2 = task::spawn(async {
          (cont2.builder)(ins3, p_sender).await;
        });

        join!(child1, child2).await;
      })
    })
  }
}
