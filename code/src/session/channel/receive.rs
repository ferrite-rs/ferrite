use async_std::task;
use async_macros::join;
use async_std::sync::{
  Sender,
  Receiver,
  channel,
};

use crate::base::{
  Nat,
  Protocol,
  Session,
  Empty,
  Context,
  AppendContext,
  ContextLens,
  PartialSession,
  run_partial_session,
  unsafe_create_session,
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
  < C, A, B >
  ( cont_builder : impl
      FnOnce ( C::Length ) ->
        PartialSession <
          C :: Appended,
          B
        >
  )
  ->
    PartialSession <
      C,
      ReceiveChannel < A, B >
    >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  C : AppendContext < ( A, () ) >,
{
  let cont = cont_builder (
    C::Length::nat()
  );

  unsafe_create_session (
    async move | ins1, sender | {
      let (sender1, receiver1) :
        ( Sender < (
            Receiver < A::Value >,
            Sender < B::Value >
          ) >,
          _ )
        = channel(1);

      sender.send(sender1).await;

      let (receiver2, sender2) :
        ( Receiver< A::Value >,
          Sender < B::Value >
        )
        = receiver1.recv().await.unwrap();

      let ins2 = C :: append_channels (
            ins1, (receiver2, ()) );

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
  P : Protocol,
  Q : Protocol,
  I : Context,
  N :
    ContextLens <
      I, Empty, P
    >
{
  unsafe_create_session (
    async move | ins1, sender | {
      let ((), ins2) =
        < N as
          ContextLens <
            I, Empty, P
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
            ContextLens <
              I, Empty, P
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
  I : Context,
  P1 : Protocol,
  P2 : Protocol,
  Q : Protocol,
  SourceLens :
    ContextLens <
      I,
      P1,
      Empty
    >,
  TargetLens :
    ContextLens <
      SourceLens :: Target,
      ReceiveChannel < P1, P2 >,
      P2
    >
{
  unsafe_create_session (
    async move | ins1, sender1 | {
      let (receiver1, ins2) =
        < SourceLens as
          ContextLens <
            I,
            P1,
            Empty
          >
        > :: split_channels (ins1);

      let ins3 =
        < SourceLens as
          ContextLens <
            I,
            P1,
            Empty
          >
        > :: merge_channels ((), ins2);

      let (receiver2, ins4) =
        < TargetLens as
          ContextLens <
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
          ContextLens <
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
  P : Protocol,
  Q : Protocol,
{
  include_session ( p1, | c1 | {
    include_session ( p2, | c2 | {
      send_channel_to ( c1, c2,
        forward ( c1 )
      )
    })
  })
}
