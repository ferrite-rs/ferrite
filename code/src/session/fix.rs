use async_macros::join;

use crate::base::*;
use async_std::task;
use async_std::sync::{ Sender, channel };

pub fn fix_session
  < F1, F2, C >
  ( cont: PartialSession < C, F2 > )
  ->
    PartialSession <
      C,
      Fix < F1 >
    >
where
  C : Context,
  F1 : Protocol,
  F2 : Protocol,
  F1 :
    TypeApp <
      Unfix <
        Fix < F1 >
      >,
      Applied = F2
    >,
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver)
        : ( Sender < F2 >, _ )
        = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send ( fix ( val ) ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx, sender2 ) );

      join!(child1, child2).await;
    })
}

pub fn unfix_session
  < F1, F2, C >
  ( cont:
      PartialSession <
        C,
        Fix < F1 >
      >
  ) ->
    PartialSession < C, F2 >
where
  C : Context,
  F1 : Protocol,
  F2 : Protocol,
  F1 :
    TypeApp <
      Unfix <
        Fix < F1 >
      >,
      Applied = F2
    >,
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver) = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send ( unfix ( val ) ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx, sender2
          ) );

      join!(child1, child2).await;
    })
}

pub fn succ_session
  < I, P >
  ( cont : PartialSession < I, P > )
  -> PartialSession < I, S < P > >
where
  P : Protocol,
  I : Context,
{
  unsafe_create_session (
    async move | ctx, sender | {
      let (sender2, receiver) = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender.send ( succ ( val ) ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx, sender2
          ) );

      join!(child1, child2).await;
    })
}

pub fn unfix_session_for
  < I, P, F, N >
  ( _ : N,
    cont :
      PartialSession <
        N :: Target,
        P
      >
  ) ->
    PartialSession < I, P >
where
  P : Protocol,
  I : Context,
  F : Protocol,
  F :
    TypeApp < Unfix <
      Fix < F >
    > >,
  F :: Applied : Protocol,
  N :
    ContextLens <
      I,
      Fix < F >,
      F :: Applied,
    >,
{
  unsafe_create_session(
    async move | ctx1, sender1 | {
      let (receiver1, ctx2) =
        N :: extract_source ( ctx1 );

        let (sender2, receiver2) = channel(1);

      let ctx3 =
        N :: insert_target ( receiver2, ctx2 );

      let child1 = task::spawn ( async move {
        let val = receiver1.recv().await.unwrap();
        sender2.send( unfix ( val ) ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx3, sender1
          ));

      join!(child1, child2).await;
    })
}
