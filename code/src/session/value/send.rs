use async_std::task;
use async_macros::join;
use std::future::{ Future };
use async_std::sync::{ Sender, Receiver, channel };

use crate::process::{ SendValue };

use crate::base::{
  Process,
  Processes,
  ProcessLens,
  PartialSession,
  run_partial_session,
  create_partial_session,
};

/*
          cont_builder() :: T ; cont :: Δ ⊢ P
    ==============================================
      send_value_async(cont_builder) :: Δ ⊢  T ∧ P
 */
pub fn send_value_async
  < T, Ins, P, Func, Fut >
  ( cont_builder: Func
  ) ->
    PartialSession <
      Ins,
      SendValue < T, P >
    >
where
  T   : Send + 'static,
  P   : Process + 'static,
  Ins : Processes + 'static,
  Func :
    FnOnce() -> Fut
      + Send + 'static,
  Fut :
    Future <
      Output = ( T,  PartialSession < Ins, P > )
    > + Send
{
  create_partial_session (
    async move |
      ins : Ins::Values,
      sender1 : Sender < (
        T,
        Receiver < P::Value >
      ) >
    | {
      let (sender2, receiver2) = channel(1);

      let (result, cont) = cont_builder().await;

      let child1 = task::spawn(async move {
        sender1.send((result, receiver2)).await;
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
  < T, Ins, P >
  ( val : T,
    cont : PartialSession < Ins, P >
  ) ->
    PartialSession <
      Ins,
      SendValue < T, P >
    >
where
  T   : Send + 'static,
  P   : Process + 'static,
  Ins : Processes + 'static
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
  < Lens, I, T, Q, P, Func, Fut >
  ( _ : Lens,
    cont_builder : Func
  ) ->
    PartialSession < I, Q >
where
  P : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static,
  T : Send + 'static,
  Func :
    FnOnce( T ) -> Fut
      + Send + 'static,
  Fut :
    Future <
      Output =
        PartialSession <
          Lens :: Target,
          Q
        >
    > + Send,
  Lens :
    ProcessLens <
      I,
      SendValue < T, P >,
      P
    >
{
  create_partial_session (
    async move |
      ins1 : I :: Values,
      sender : Sender < Q :: Value >
    | {
      let (receiver1, ins2) =
        < Lens as
          ProcessLens <
            I,
            SendValue < T, P >,
            P
          >
        >
        :: split_channels ( ins1 );

      let (val, receiver2) = receiver1.recv().await.unwrap();

      let ins3 =
        < Lens as
          ProcessLens <
            I,
            SendValue < T, P >,
            P
          >
        >
        :: merge_channels (receiver2, ins2);

      let cont = cont_builder(val).await;

      run_partial_session
        ( cont, ins3, sender
        ).await;
    })
}
