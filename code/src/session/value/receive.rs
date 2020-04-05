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
  unsafe_run_session,
  unsafe_create_session,
};

/*
              cont_builder(x) :: Δ ⊢ P
    ==============================================
      send_input(cont_builder) :: Δ ⊢ T ⊃ P
 */
pub fn receive_value
  < T, C, A, Fut >
  ( cont_builder : impl
      FnOnce (T) -> Fut + Send + 'static
  ) ->
     PartialSession < C, ReceiveValue < T, A > >
where
  T : Send + 'static,
  A : Protocol,
  C : Context,
  Fut :
    Future <
      Output = PartialSession < C, A >
    > + Send
{
  unsafe_create_session (
    async move |
      ctx,
      sender1 : Sender <
        Sender <(
          Val < T >,
          Sender < A::Payload >
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

        unsafe_run_session
          ( cont, ctx, sender3
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
  < N, T, C, A, B, Fut >
  ( _ : N,
    cont_builder : impl
      FnOnce() -> Fut
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
        ( T,
          PartialSession <
            N :: Target,
            B
          > )
    > + Send,
  N :
    ContextLens <
      C,
      ReceiveValue < T, A >,
      A
    >
{
  unsafe_create_session (
    async move | ctx1, sender1 | {
      let (receiver1, ctx2) = N :: extract_source ( ctx1 );

      let sender2 = receiver1.recv().await.unwrap();

      let (val, cont) = cont_builder().await;

      let (sender3, receiver3) = channel(1);

      let ctx3 = N :: insert_target( receiver3, ctx2 );

      let child1 = task::spawn(async move {
        sender2.send( (
          Val { val: val },
          sender3
        ) ).await;
      });

      let child2 = task::spawn(async move {
        unsafe_run_session
          (cont, ctx3, sender1
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
