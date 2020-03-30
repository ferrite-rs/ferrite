use async_std::task;
use async_macros::join;
use std::future::{ Future };
use async_std::sync::{ Sender, Receiver, channel };

use crate::process::{ Val, SendValue };

use crate::base::{
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  run_partial_session,
  unsafe_create_session,
};

/*
          cont_builder() :: T ; cont :: Δ ⊢ P
    ==============================================
      send_value_async(cont_builder) :: Δ ⊢  T ∧ P
 */
pub fn send_value_async
  < T, C, P, Func, Fut >
  ( cont_builder: Func
  ) ->
    PartialSession <
      C,
      SendValue < T, P >
    >
where
  T   : Send + 'static,
  P   : Protocol,
  C : Context,
  Func :
    FnOnce() -> Fut
      + Send + 'static,
  Fut :
    Future <
      Output = ( T,  PartialSession < C, P > )
    > + Send
{
  unsafe_create_session (
    async move |
      ins : C::Values,
      sender1 : Sender < (
        Val < T >,
        Receiver < P::Payload >
      ) >
    | {
      let (sender2, receiver2) = channel(1);

      let (result, cont) = cont_builder().await;

      let child1 = task::spawn(async move {
        sender1.send(
          ( Val {
              val : result
            },
            receiver2
          ) ).await;
      });

      let child2 = task::spawn(async move {
        run_partial_session
          ( cont, ins, sender2
          ).await;
      });

      join!(child1, child2).await;
    })
}

pub fn send_value
  < T, C, P >
  ( val : T,
    cont : PartialSession < C, P >
  ) ->
    PartialSession <
      C,
      SendValue < T, P >
    >
where
  T   : Send + 'static,
  P   : Protocol,
  C : Context
{
  send_value_async ( async move || {
    ( val, cont )
  })
}

/*
        cont_builder(x) :: Q, Δ ⊢ P    x :: T
    ================================================
      receive_value_from(cont_builder) :: T ∧ Q, Δ ⊢ P
 */
pub fn receive_value_from
  < N, I, T, Q, P, Func, Fut >
  ( _ : N,
    cont_builder : Func
  ) ->
    PartialSession < I, Q >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  T : Send + 'static,
  Func :
    FnOnce( T ) -> Fut
      + Send + 'static,
  Fut :
    Future <
      Output =
        PartialSession <
          N :: Target,
          Q
        >
    > + Send,
  N :
    ContextLens <
      I,
      SendValue < T, P >,
      P
    >
{
  unsafe_create_session (
    async move |
      ins1 : I :: Values,
      sender : Sender < Q :: Payload >
    | {
      let (receiver1, ins2) =
        < N as
          ContextLens <
            I,
            SendValue < T, P >,
            P
          >
        >
        :: split_channels ( ins1 );

      let (val, receiver2) = receiver1.recv().await.unwrap();

      let ins3 =
        < N as
          ContextLens <
            I,
            SendValue < T, P >,
            P
          >
        >
        :: merge_channels (receiver2, ins2);

      let cont = cont_builder(val.val).await;

      run_partial_session
        ( cont, ins3, sender
        ).await;
    })
}
