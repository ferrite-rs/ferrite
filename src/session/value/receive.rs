use std::future::{ Future };
use async_std::sync::{ Sender, channel };

use crate::protocol::{ ReceiveValue };

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
      FnOnce (T) -> Fut
      + Send + 'static
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
    move |
      ctx,
      sender1 : Sender <
        ReceiveValue < T, A >,
      >
    | async move {
      let (sender2, receiver2) = channel(1);

      sender1.send ( ReceiveValue ( sender2 ) ).await;

      let ( val, sender3 )
        = receiver2.recv().await.unwrap();

      let cont = cont_builder(val).await;

      unsafe_run_session
        ( cont, ctx, sender3
        ).await;
    })
}

pub fn send_value_to
  < N, C, A, B, T >
  ( _ : N,
    val : T,
    cont :
      PartialSession <
        N :: Target,
        A
      >
  ) ->
    PartialSession < C, A >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  T : Send + 'static,
  N :
    ContextLens <
      C,
      ReceiveValue < T, B >,
      B
    >
{
  unsafe_create_session (
    move | ctx1, sender1 | async move {
      let (receiver1, ctx2) = N :: extract_source ( ctx1 );

      let ReceiveValue ( sender2 )
        = receiver1.recv().await.unwrap();

      let (sender3, receiver3) = channel(1);

      let ctx3 = N :: insert_target( receiver3, ctx2 );

      sender2.send( (
        val,
        sender3
      ) ).await;

      unsafe_run_session
        (cont, ctx3, sender1
        ).await;
    })
}
