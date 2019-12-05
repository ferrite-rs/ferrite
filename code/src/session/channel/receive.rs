use async_std::task;
use async_std::sync::{ Sender, Receiver, channel };
use async_macros::join;

use crate::process::{ ReceiveChannel };
use crate::base::*;
use crate::processes::*;
use crate::session::link::*;
use crate::session::forward::{ forward };

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

  return PartialSession {
    builder: Box::new( move | ins1, sender | {
      Box::pin(async move {
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

        (cont.builder)(ins2, sender2).await;
      })
    })
  }
}

pub fn receive_channel_slot
  < S, T, D, P, Q, Lens >
(
  _ : Lens,
  cont : PartialSession < T, Q >
) ->
  PartialSession < S, ReceiveChannel < P, Q > >
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
      P
    >
{
  return PartialSession {
    builder: Box::new( |
      ins1,
      sender
    | {
      let ((), ins2) =
        < Lens as
          ProcessLens <
            S, T, D,
            Inactive,
            P
          >
        > :: split_channels (ins1);

      let (sender1, receiver1) :
        (Sender <(
          Receiver < P::Value >,
          Sender < Q::Value >
        )>, _)
        = channel(1);

      Box::pin(async move {
        let child1 = task::spawn(async move {
          sender.send(sender1).await;
        });

        let child2 = task::spawn(async move {
          let (receiver2, sender2) :
            ( Receiver < P::Value >,
              Sender < Q::Value > )
            = receiver1.recv().await.unwrap();

          let ins3 =
            < Lens as
              ProcessLens <
                S, T, D,
                Inactive,
                P
              >
            > :: merge_channels (receiver2, ins2);

          (cont.builder)(ins3, sender2).await;
        });

        join!(child1, child2).await;
      })
    })
  }
}

/*
    Implication, Left Rule

                cont :: Q, Δ ⊢ S
    ========================================
      send_channel_to(cont) :: P, P ⊸ Q, Δ ⊢ S
 */
pub fn send_channel_to
  < TargetLens, SourceLens,
    S,
    T1, T2,
    D1, D2,
    P1, P2, Q
  >
  ( _ : TargetLens,
    _ : SourceLens,
    cont : PartialSession < T2, Q >
  ) ->
    PartialSession < S, Q >
where
  S : Processes + 'static,
  T1 : Processes + 'static,
  T2 : Processes + 'static,
  D1 : Processes + 'static,
  D2 : Processes + 'static,
  P1 : Process + 'static,
  P2 : Process + 'static,
  Q : Process + 'static,
  SourceLens :
    ProcessLens <
      S, T1, D1,
      P1,
      Inactive
    >,
  TargetLens :
    ProcessLens <
      T1, T2, D2,
      ReceiveChannel < P1, P2 >,
      P2
    >
{

  return  PartialSession {
    builder : Box::new( |
      ins1 : S :: Values,
      sender1 : Sender < Q::Value >
    | {
      Box::pin(async {
        let (receiver1, ins2) =
          < SourceLens as
            ProcessLens <
              S, T1, D1,
              P1,
              Inactive
            >
          > :: split_channels (ins1);

        let ins3 =
          < SourceLens as
            ProcessLens <
              S, T1, D1,
              P1,
              Inactive
            >
          > :: merge_channels ((), ins2);

        let (receiver2, ins4) =
          < TargetLens as
            ProcessLens <
              T1, T2, D2,
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
              T1, T2, D2,
              ReceiveChannel < P1, P2 >,
              P2
            >
          > :: merge_channels (receiver3, ins4);

        let child2 = task::spawn(async move {
          (cont.builder)(ins5, sender1).await;
        });

        join!(child1, child2).await;
      })
    })
  }
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
  session_2 ( | slot1, slot2 | {
    link ( slot2, p1,
      link ( slot1, p2,
        send_channel_to ( slot2, slot1,
          forward ( slot2 )
        ))
    )
  })
}
