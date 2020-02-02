use async_std::task;
use async_macros::join;
use async_std::sync::{
  Sender,
  Receiver,
  channel,
};

use crate::base::{
  Process,
  Session,
  Inactive,
  Processes,
  Appendable,
  ProcessLens,
  PartialSession,
  run_partial_session,
  create_partial_session,
};

use crate::processes::{
  NextSelector
};

use crate::process::{ ReceiveChannel };
use crate::session::forward::{ forward };
use crate::session::include::{ include_session };

/*
    Implication, Right Rule

          cont :: Δ, P  ⊢ Q
    ====================================
      receive_channel(cont) :: Δ  ⊢ P ⊸ Q
 */
pub fn receive_channel
  < Ins, P, Q, F >
  ( cont_builder : F )
  ->
    PartialSession <
      Ins,
      ReceiveChannel < P, Q >
    >
where
  P : Process + 'static,
  Q : Process + 'static,
  Ins : Processes + 'static,
  Ins : NextSelector,
  Ins : Appendable < ( P, () ) >,
  F : FnOnce
        ( < Ins as NextSelector > :: Selector )
        ->
          PartialSession <
            < Ins as
              Appendable <
                ( P, () )
              >
            > :: AppendResult,
            Q
          >
{
  let cont = cont_builder (
    < Ins as NextSelector > :: make_selector ()
  );

  create_partial_session (
    async move | ins1, sender | {
      let (sender1, receiver1) :
        ( Sender < (
            Receiver < P::Value >,
            Sender < Q::Value >
          ) >,
          _ )
        = channel(1);

      sender.send(sender1).await;

      let (receiver2, sender2) :
        ( Receiver< P::Value >,
          Sender < Q::Value >
        )
        = receiver1.recv().await.unwrap();

      let ins2 =
        < Ins as
          Appendable <
            ( P, () )
          >
        > :: append_channels ( ins1, (receiver2, ()) );

        run_partial_session
          ( cont, ins2, sender2
          ).await;
    })
}

pub fn receive_channel_slot
  < I, P, Q, N >
(
  _ : N,
  cont :
    PartialSession <
      N :: Target,
      Q
    >
) ->
  PartialSession < I, ReceiveChannel < P, Q > >
where
  P : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static,
  N :
    ProcessLens <
      I, Inactive, P
    >
{
  create_partial_session (
    async move | ins1, sender | {
      let ((), ins2) =
        < N as
          ProcessLens <
            I, Inactive, P
          >
        > :: split_channels (ins1);

      let (sender1, receiver1) :
        (Sender <(
          Receiver < P::Value >,
          Sender < Q::Value >
        )>, _)
        = channel(1);

      let child1 = task::spawn(async move {
        sender.send(sender1).await;
      });

      let child2 = task::spawn(async move {
        let (receiver2, sender2) :
          ( Receiver < P::Value >,
            Sender < Q::Value > )
          = receiver1.recv().await.unwrap();

        let ins3 =
          < N as
            ProcessLens <
              I, Inactive, P
            >
          > :: merge_channels (receiver2, ins2);

          run_partial_session
            ( cont, ins3, sender2
            ).await;
      });

      join!(child1, child2).await;
    })
}

/*
    Implication, Left Rule

                cont :: Q, Δ ⊢ S
    ========================================
      send_channel_to(cont) :: P, P ⊸ Q, Δ ⊢ S
 */
pub fn send_channel_to
  < TargetLens, SourceLens,
    I, P1, P2, Q
  >
  ( _ : TargetLens,
    _ : SourceLens,
    cont :
      PartialSession <
        TargetLens :: Target,
        Q
      >
  ) ->
    PartialSession < I, Q >
where
  I : Processes + 'static,
  P1 : Process + 'static,
  P2 : Process + 'static,
  Q : Process + 'static,
  SourceLens :
    ProcessLens <
      I,
      P1,
      Inactive
    >,
  TargetLens :
    ProcessLens <
      SourceLens :: Target,
      ReceiveChannel < P1, P2 >,
      P2
    >
{
  create_partial_session (
    async move | ins1, sender1 | {
      let (receiver1, ins2) =
        < SourceLens as
          ProcessLens <
            I,
            P1,
            Inactive
          >
        > :: split_channels (ins1);

      let ins3 =
        < SourceLens as
          ProcessLens <
            I,
            P1,
            Inactive
          >
        > :: merge_channels ((), ins2);

      let (receiver2, ins4) =
        < TargetLens as
          ProcessLens <
            SourceLens :: Target,
            ReceiveChannel < P1, P2 >,
            P2
          >
        > :: split_channels (ins3);

      let sender2 :
        Sender <
          ( Receiver < P1 :: Value >,
            Sender < P2 :: Value >
          )
        >
        = receiver2.recv().await.unwrap();

      let (sender3, receiver3) = channel(1);

      let child1 = task::spawn(async move {
        sender2.send((receiver1, sender3)).await;
      });

      let ins5 =
        < TargetLens as
          ProcessLens <
            SourceLens :: Target,
            ReceiveChannel < P1, P2 >,
            P2
          >
        > :: merge_channels (receiver3, ins4);

      let child2 = task::spawn(async move {
        run_partial_session
          ( cont, ins5, sender1
          ).await;
      });

      join!(child1, child2).await;
    })
}

/*
    Implication, Application

      p1 :: · ⊢ P ⊸ Q       p2 :: · ⊢ P
    ========================================
        apply_channel(p1, p2) :: · ⊢ Q
 */
pub fn apply_channel
  < P, Q >
(
  p1 : Session < ReceiveChannel < P, Q > >,
  p2 : Session < P >
) ->
  Session < Q >
where
  P : Process + 'static,
  Q : Process + 'static,
{
  include_session ( p1, | c1 | {
    include_session ( p2, | c2 | {
      send_channel_to ( c1, c2,
        forward ( c1 )
      )
    })
  })
}
