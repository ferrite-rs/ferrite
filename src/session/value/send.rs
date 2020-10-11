use async_std::task;
use async_macros::join;
use std::future::{ Future };
use async_std::sync::{ channel };

use crate::protocol::{ SendValue };

use crate::base::{
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

/*
          cont_builder() :: T ; cont :: Δ ⊢ P
    ==============================================
      send_value_async(cont_builder) :: Δ ⊢  T ∧ P
 */
pub fn send_value_async
  < T, C, A, Fut >
  ( cont_builder: impl
      FnOnce() -> Fut
      + Send + 'static
  ) ->
    PartialSession <
      C,
      SendValue < T, A >
    >
where
  T : Send + 'static,
  A : Protocol,
  C : Context,
  Fut :
    Future <
      Output = ( T,  PartialSession < C, A > )
    > + Send
{
  unsafe_create_session (
    move | ctx, sender1 | async move {
      let (sender2, receiver2) = channel(1);

      let (result, cont) = cont_builder().await;

      let child1 = task::spawn(async move {
        sender1.send(
          SendValue
            ( result,
              receiver2
            ) ).await;
      });

      let child2 = task::spawn(async move {
        unsafe_run_session
          ( cont, ctx, sender2
          ).await;
      });

      join!(child1, child2).await;
    })
}

pub fn send_value
  < T, C, A >
  ( val : T,
    cont : PartialSession < C, A >
  ) ->
    PartialSession <
      C,
      SendValue < T, A >
    >
where
  T : Send + 'static,
  A : Protocol,
  C : Context
{
  send_value_async ( move || async move {
    ( val, cont )
  })
}

/*
        cont_builder(x) :: Q, Δ ⊢ P    x :: T
    ================================================
      receive_value_from(cont_builder) :: T ∧ Q, Δ ⊢ P
 */
pub fn receive_value_from
  < N, C, T, A, B, Fut >
  ( _ : N,
    cont : impl
      FnOnce ( T ) -> Fut
      + Send + 'static
  ) ->
    PartialSession < C, B >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  T : Send + 'static,
  Fut :
    Future <
      Output =
        PartialSession <
          N :: Target,
          B
        >
    > + Send,
  N :
    ContextLens <
      C,
      SendValue < T, A >,
      A
    >
{
  unsafe_create_session (
    move | ctx1, sender | async move {
      let (receiver1, ctx2) =
        N :: extract_source ( ctx1 );

      let SendValue ( val, receiver2 )
        = receiver1.recv().await.unwrap();

      let ctx3 = N :: insert_target (receiver2, ctx2);

      let cont2 = cont(val).await;

      unsafe_run_session
        ( cont2, ctx3, sender
        ).await;
    })
}
