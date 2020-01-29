use async_std::task;
use async_macros::join;
use std::future::{ Future };
use async_std::sync::{ Sender, Receiver, channel };

use crate::process::{ Val, ReceiveValue };

use crate::base::{
  Process,
  Processes,
  ProcessLens,
  PartialSession,
  run_partial_session,
  create_partial_session,
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
  P : Process + 'static,
  Ins : Processes + 'static,
  Func :
    FnOnce(T) -> Fut
    + Send + 'static,
  Fut :
    Future <
      Output = PartialSession < Ins, P >
    > + Send
{
  create_partial_session (
    async move |
      ins : Ins::Values,
      sender1 : Sender < (
        Sender < Val < T > >,
        Receiver < P::Value >
      ) >
    | {
      let (sender2, receiver2)
        : (Sender < Val < T > >, _)
        = channel(1);

      let (sender3, receiver3) = channel(1);

      let child1 = task::spawn ( async move {
        sender1.send((sender2, receiver3)).await;
      });


      let child2 = task::spawn ( async move {
        let val = receiver2.recv().await.unwrap();

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
  < Lens, I, P, Q, T, Func, Fut >
  ( _ : Lens,
    cont_builder : Func
  ) ->
    PartialSession < I, P >
where
  P : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static,
  T : Send + 'static,
  Func :
    FnOnce() -> Fut
    + Send + 'static,
  Fut :
    Future <
      Output =
        ( T,
          PartialSession <
            Lens :: Target,
            P
          > )
    > + Send,
  Lens :
    ProcessLens <
      I,
      ReceiveValue < T, Q >,
      Q
    >
{
  create_partial_session (
    async move |
      ins1: I :: Values,
      sender1 : Sender < P::Value >
    | {
      let (receiver1, ins2) =
        < Lens as
          ProcessLens <
            I,
            ReceiveValue < T, Q >,
            Q
          >
        >
        :: split_channels ( ins1 );

      let (sender2, receiver2) = receiver1.recv().await.unwrap();
      let (val, cont) = cont_builder().await;

      let ins3 =
        < Lens as
          ProcessLens <
            I,
            ReceiveValue < T, Q >,
            Q
          >
        > :: merge_channels( receiver2, ins2);

      let child1 = task::spawn(async move {
        sender2.send( Val {
          val: val
        }).await;
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
  < Lens, I, P, Q, T >
  ( lens : Lens,
    value : T,
    cont :
      PartialSession <
        Lens :: Target,
        P
      >
  ) ->
    PartialSession < I, P >
where
  P : Process + 'static,
  Q : Process + 'static,
  I : Processes + 'static,
  T : Send + 'static,
  Lens :
    ProcessLens <
      I,
      ReceiveValue < T, Q >,
      Q
    >
{
  send_value_to_async ( lens, async || {
    ( value, cont )
  })
}
