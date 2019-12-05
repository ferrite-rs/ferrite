
use async_std::sync::{ Sender };
use std::pin::Pin;
use std::future::{ Future };

use crate::process::{ End };
use crate::base::*;

/*

          cleaner() :: ()
    ===============================
      terminate_async (cleaner) :: · ⊢ 1

    Create a unit process (1) out of nothing.
 */
pub fn terminate_async < F, Ins >
  ( cleaner: F )
  -> PartialSession < Ins, End >
where
  Ins : EmptyList,
  F : FnOnce() ->
        Pin < Box < dyn Future <
          Output = ()
        > + Send > >
      + Send + 'static
{
  return PartialSession {
    builder: Box::new(move |_, sender: Sender<()>| {
      Box::pin ( async move {
        cleaner().await;
        sender.send(()).await;
      })
    })
  };
}

pub fn terminate < Ins >
  () ->
    PartialSession < Ins, End >
where
  Ins : EmptyList
{
  terminate_async ( || {
    wrap_async(())
  } )
}

pub fn terminate_nil
  () ->
    Session < End >
{
  terminate ()
}

/*
          cont :: Δ ⊢ P
    ===========================
      wait_async (cont) :: 1, Δ ⊢ P

    Wait for a given input process to terminate, then continue as P.
 */

pub fn wait_async
  < Lens, S, T, D, P, F >
  ( _ : Lens,
    cont_builder : F
  ) ->
    PartialSession < S, P >
where
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  P : Process + 'static,
  F : FnOnce () ->
        Pin < Box < dyn Future <
          Output = PartialSession < T, P >
        > + Send > >
      + Send + 'static,
  Lens : ProcessLens < S, T, D, End, Inactive >
{
  return PartialSession {
    builder: Box::new(move |
      ins1 : <S as Processes>::Values,
      sender
    | {
      Box::pin(async {
        let (wait_chan, ins2) =
          < Lens as ProcessLens < S, T, D, End, Inactive > >
          :: split_channels (ins1);

        let ins3 =
          < Lens as ProcessLens < S, T, D, End, Inactive > >
          :: merge_channels ((), ins2);

        wait_chan.recv().await.unwrap();
        let cont = cont_builder().await;
        (cont.builder)(ins3, sender).await;
      })
    })
  };
}

pub fn wait
  < Lens, S, T, D, P >
  ( lens : Lens,
    cont : PartialSession < T, P >
  ) ->
    PartialSession < S, P >
where
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  P : Process + 'static,
  Lens : ProcessLens < S, T, D, End, Inactive >
{
  wait_async ( lens, || {
    wrap_async( cont )
  })
}
