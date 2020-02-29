use async_macros::join;

use crate::base::*;
use crate::process::*;
use async_std::task;
use async_std::sync::{ Sender, channel };

pub fn fix_session
  < F, I >
  ( cont:
      PartialSession <
        I,
        < F as
          TyApp < Recur <
            FixProtocol < F >
          > >
        > :: Type
      >
  ) ->
    PartialSession <
      I,
      FixProtocol < F >
    >
where
  I : Context,
  F : Protocol,
  F :
    TyApp < Recur <
      FixProtocol < F >
    > >,
  F :: Value :
    TyApp <
      Recur <
        Fix < F :: Value >
      >,
      Type =
        < < F as
            TyApp < Recur <
              FixProtocol < F >
            > >
          > :: Type
          as Protocol
        > :: Value,
    >,
  < F as
    TyApp < Recur <
      FixProtocol < F >
    > >
  > :: Type : Protocol,
  < F :: Value as
    TyApp < Recur <
        Fix < F :: Value >
      > >
  > :: Type :
    Send
{
  unsafe_create_session (
    async move |
      ins,
      sender1 :
        Sender <
          Fix < F :: Value >
        >
    | {
      let (sender2, receiver)
        : ( Sender <
              < < F as
                  TyApp < Recur <
                    FixProtocol < F >
                  > >
                > :: Type
                as Protocol
              > :: Value
            >
          , _
          )
        = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send ( fix ( val ) ).await;
      });

      let child2 = task::spawn(
        run_partial_session
          ( cont, ins, sender2
          ) );

      join!(child1, child2).await;
    })
}

pub fn unfix_session
  < F, I >
  ( cont:
      PartialSession <
        I,
        FixProtocol < F >
      >
  ) ->
    PartialSession <
      I,
      < F as
        TyApp < Recur <
          FixProtocol < F >
        > >
      > :: Type
    >
where
  I : Context,
  F : Protocol,
  F :
    TyApp < Recur <
      FixProtocol < F >
    > >,
  F :: Value :
    TyApp <
      Recur <
        Fix < F :: Value >
      >,
      Type =
        < < F as
            TyApp < Recur <
              FixProtocol < F >
            > >
          > :: Type
          as Protocol
        > :: Value,
    >,
  < F as
    TyApp < Recur <
      FixProtocol < F >
    > >
  > :: Type : Protocol,
  < F :: Value as
    TyApp < Recur <
        Fix < F :: Value >
      > >
  > :: Type :
    Send
{
  unsafe_create_session (
    async move |
      ins,
      sender1 :
        Sender <
          < < F as
              TyApp < Recur <
                FixProtocol < F >
              > >
            > :: Type
            as Protocol
          > :: Value
        >
    | {
      let (sender2, receiver)
        : ( Sender <
              Fix < F :: Value >
            >
          , _
          )
        = channel(1);


      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send ( unfix ( val ) ).await;
      });

      let child2 = task::spawn(
        run_partial_session
          ( cont, ins, sender2
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
    async move | ins, sender | {
      let (sender2, receiver) = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender.send ( succ ( val ) ).await;
      });

      let child2 = task::spawn(
        run_partial_session
          ( cont, ins, sender2
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
    TyApp < Recur <
      FixProtocol < F >
    > >,
  F :: Value :
    TyApp <
      Recur <
        Fix < F :: Value >
      >,
      Type =
        < < F as
            TyApp < Recur <
              FixProtocol < F >
            > >
          > :: Type
          as Protocol
        > :: Value,
    >,
  < F as
    TyApp < Recur <
      FixProtocol < F >
    > >
  > :: Type : Protocol,
  < F :: Value as
    TyApp < Recur <
      Fix < F :: Value >
    > >
  > :: Type :
    Send,
  N :
    ContextLens <
      I,
      FixProtocol < F >,
      < F as
        TyApp < Recur <
          FixProtocol < F >
        > >
      > :: Type,
    >
{
  unsafe_create_session(
    async move | ins1, sender1 | {
      let (receiver1, ins2) =
        N :: split_channels ( ins1 );

        let (sender2, receiver2)
        : ( Sender <
              < < F as
                  TyApp < Recur <
                    FixProtocol < F >
                  > >
                > :: Type
                as Protocol
              > :: Value
            >
          , _
          )
        = channel(1);

      let ins3 =
        N :: merge_channels ( receiver2, ins2 );

      let child1 = task::spawn ( async move {
        let val = receiver1.recv().await.unwrap();
        sender2.send( unfix ( val ) ).await;
      });

      let child2 = task::spawn(
        run_partial_session
          ( cont, ins3, sender1
          ));

      join!(child1, child2).await;
    })
}
