use async_std::task;
use async_macros::join;
use std::future::{ Future };
use async_std::sync::{ Sender, channel };

use crate::process::{ Val, ReceiveValue };

use crate::base::{
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  run_partial_session,
  unsafe_create_session,
};

/*
              cont_builder(x) :: Δ ⊢ P
    ==============================================
      send_input(cont_builder) :: Δ ⊢ T ⊃ P
 */
pub fn receive_value
  < T, Ins, P, Func, Fut >
  ( cont_builder : Func ) ->
     PartialSession < Ins, ReceiveValue < T, P > >
where
  T : Send + 'static,
  P : Protocol,
  Ins : Context,
  Func :
    FnOnce(T) -> Fut
    + Send + 'static,
  Fut :
    Future <
      Output = PartialSession < Ins, P >
    > + Send
{
  unsafe_create_session (
    async move |
      ins : Ins::Values,
      sender1 : Sender <
        Sender <(
          Val < T >,
          Sender < P::Value >
        )>,
      >
    | {
      let (sender2, receiver2) = channel(1);

      let child1 = task::spawn ( async move {
        sender1.send(sender2).await;
      });


      let child2 = task::spawn ( async move {
        let (val, sender3) = receiver2.recv().await.unwrap();

        let cont = cont_builder(val.val).await;

        run_partial_session
          ( cont, ins, sender3
          ).await;
      });

      join!(child1, child2).await;
    })
}

/*
          cont_builder() :: T ; Q, Δ ⊢ P
    ===========================================
      send_value_to_async(cont_builder) :: T ⊃ Q, Δ ⊢ P
 */
pub fn send_value_to_async
  < N, I, P, Q, T, Func, Fut >
  ( _ : N,
    cont_builder : Func
  ) ->
    PartialSession < I, P >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  T : Send + 'static,
  Func :
    FnOnce() -> Fut
    + Send + 'static,
  Fut :
    Future <
      Output =
        ( T,
          PartialSession <
            N :: Target,
            P
          > )
    > + Send,
  N :
    ContextLens <
      I,
      ReceiveValue < T, Q >,
      Q
    >
{
  unsafe_create_session (
    async move |
      ins1: I :: Values,
      sender1 : Sender < P::Value >
    | {
      let (receiver1, ins2) =
        < N as
          ContextLens <
            I,
            ReceiveValue < T, Q >,
            Q
          >
        >
        :: split_channels ( ins1 );

      let sender2 = receiver1.recv().await.unwrap();

      let (val, cont) = cont_builder().await;

      let (sender3, receiver3) = channel(1);

      let ins3 =
        < N as
          ContextLens <
            I,
            ReceiveValue < T, Q >,
            Q
          >
        > :: merge_channels( receiver3, ins2 );

      let child1 = task::spawn(async move {
        sender2.send( (
          Val { val: val },
          sender3
        ) ).await;
      });

      let child2 = task::spawn(async move {
        run_partial_session
          (cont, ins3, sender1
          ).await;
      });

      join!(child1, child2).await;
    })
}

pub fn send_value_to
  < N, I, P, Q, T >
  ( lens : N,
    value : T,
    cont :
      PartialSession <
        N :: Target,
        P
      >
  ) ->
    PartialSession < I, P >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  T : Send + 'static,
  N :
    ContextLens <
      I,
      ReceiveValue < T, Q >,
      Q
    >
{
  send_value_to_async ( lens, async || {
    ( value, cont )
  })
}
