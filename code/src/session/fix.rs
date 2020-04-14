use async_macros::join;

use crate::base::*;
use crate::process::*;
use async_std::task;
use async_std::sync::{ Sender, channel };

pub fn fix_session
  < F0, F1, F2, P1, P2, C >
  ( cont: PartialSession < C, F2 > )
  ->
    PartialSession <
      C,
      FixProtocol < F0 >
    >
where
  C : Context,
  F0 : Send + 'static,
  P1 : Send + 'static,
  P2 : Send + 'static,
  F0 : TyApp < Z, Applied=F1 >,
  F1 : Protocol < Payload = P1 >,
  F2 : Protocol < Payload = P2 >,
  F1 :
    TyApp <
      Unfix <
        FixProtocol < F0 >
      >,
      Applied = F2
    >,
  P1 :
    TyApp <
      Unfix <
        Fix < P1 >
      >,
      Applied = P2,
    >,
{
  unsafe_create_session (
    async move |
      ctx,
      sender1 : Sender < Fix < P1 > >
    | {
      let (sender2, receiver)
        : ( Sender < P2 >, _ )
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
  < F0, F1, F2, P1, P2, C >
  ( cont:
      PartialSession <
        C,
        FixProtocol < F0 >
      >
  ) ->
    PartialSession < C, F2 >
where
  C : Context,
  F0 : Send + 'static,
  P1 : Send + 'static,
  P2 : Send + 'static,
  F0 : TyApp < Z, Applied = F1 >,
  F1 : Protocol < Payload = P1 >,
  F2 : Protocol < Payload = P2 >,
  F1 :
    TyApp <
      Unfix <
        FixProtocol < F0 >
      >,
      Applied = F2
    >,
  P1 :
    TyApp <
      Unfix <
        Fix < P1 >
      >,
      Applied = P2,
    >,
{
  unsafe_create_session (
    async move |
      ctx,
      sender1 : Sender < P2 >
    | {
      let (sender2, receiver)
        : ( Sender < Fix < P1 > > , _ )
        = channel(1);

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
  < I, P, G, F, N >
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
  G : Send + 'static,
  G : TyApp < Z, Applied=F >,
  F : Protocol,
  F :
    TyApp < Unfix <
      FixProtocol < G >
    > >,
  F :: Payload :
    TyApp <
      Unfix <
        Fix < F :: Payload >
      >,
      Applied =
        < < F as
            TyApp < Unfix <
              FixProtocol < G >
            > >
          > :: Applied
          as Protocol
        > :: Payload,
    >,
  < F as
    TyApp < Unfix <
      FixProtocol < G >
    > >
  > :: Applied : Protocol,
  < F :: Payload as
    TyApp < Unfix <
      Fix < F :: Payload >
    > >
  > :: Applied :
    Send,
  N :
    ContextLens <
      I,
      FixProtocol < G >,
      < F as
        TyApp < Unfix <
          FixProtocol < G >
        > >
      > :: Applied,
    >
{
  unsafe_create_session(
    async move | ctx1, sender1 | {
      let (receiver1, ctx2) =
        N :: extract_source ( ctx1 );

        let (sender2, receiver2)
        : ( Sender <
              < < F as
                  TyApp < Unfix <
                    FixProtocol < G >
                  > >
                > :: Applied
                as Protocol
              > :: Payload
            >
          , _
          )
        = channel(1);

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
