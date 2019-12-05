use async_std::task;
use async_macros::join;
use async_std::sync::{ Sender, Receiver, channel };
use std::pin::Pin;
use std::future::{ Future };

use crate::process::{ SendValue };
use crate::base::*;

/*
          cont_builder() :: T ; cont :: Δ ⊢ P
    ==============================================
      send_value_async(cont_builder) :: Δ ⊢  T ∧ P
 */
pub fn send_value_async
  < T, Ins, P, F >
  ( cont_builder: F
  ) ->
    PartialSession <
      Ins,
      SendValue < T, P >
    >
where
  T   : Send + 'static,
  P   : Process + 'static,
  Ins : Processes + 'static,
  F   : FnOnce() ->
          Pin < Box < dyn Future <
            Output = ( T,  PartialSession < Ins, P > )
          > + Send > >
        + Send + 'static
{
  return PartialSession {
    builder:
      Box::new(move |
        ins : Ins::Values,
        sender1 : Sender < (
          T,
          Receiver < P::Value >
        ) >
      | {
        Box::pin ( async move {
          let (sender2, receiver2) = channel(1);

          let (result, cont) = cont_builder().await;

          let child1 = task::spawn(async move {
            sender1.send((result, receiver2)).await;
          });

          let child2 = task::spawn(async move {
            (cont.builder)(ins, sender2).await;
          });

          join!(child1, child2).await;
        })
      })
  }
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
  send_value_async ( move || {
    wrap_async ( ( val, cont ) )
  })
}

/*
        cont_builder(x) :: Q, Δ ⊢ P    x :: T
    ================================================
      receive_value_from(cont_builder) :: T ∧ Q, Δ ⊢ P
 */
pub fn receive_value_from
  < Lens, Ins1, Ins2, Ins3, T, Q, P, F >
  ( _ : Lens,
    cont_builder : F
  ) ->
    PartialSession < Ins1, Q >
where
  P : Process + 'static,
  Q : Process + 'static,
  Ins1 : Processes + 'static,
  Ins2 : Processes + 'static,
  Ins3 : Processes + 'static,
  T : Send + 'static,
  F : FnOnce( T ) ->
        Pin < Box < dyn Future <
          Output = PartialSession < Ins2, Q >
        > + Send > >
      + Send + 'static,
  Lens :
    ProcessLens <
      Ins1,
      Ins2,
      Ins3,
      SendValue < T, P >,
      P
    >
{
  return  PartialSession {
    builder: Box::new(move |
      ins1 : Ins1 :: Values,
      sender : Sender < Q :: Value >
    | {
      Box::pin ( async {
        let (receiver1, ins2) =
          < Lens as
            ProcessLens < Ins1, Ins2, Ins3, SendValue < T, P >, P >
          >
          :: split_channels ( ins1 );

        let (val, receiver2) = receiver1.recv().await.unwrap();

        let ins3 =
          < Lens as
            ProcessLens < Ins1, Ins2, Ins3, SendValue < T, P >, P >
          >
          :: merge_channels (receiver2, ins2);

        let cont = cont_builder(val).await;

        (cont.builder)(
          ins3,
          sender
        ).await;
      })
    })
  }
}
