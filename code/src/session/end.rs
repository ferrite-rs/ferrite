
use async_std::sync::{ Sender };
use std::future::{ Future };

use crate::process::{ End };
use crate::base::*;

/*

          cleaner() :: ()
    ===============================
      terminate_async (cleaner) :: · ⊢ 1

    Create a unit process (1) out of nothing.
 */
pub fn terminate_async < Ins, Func, Fut >
  ( cleaner: Func )
  -> PartialSession < Ins, End >
where
  Ins : EmptyList + 'static,
  Func : 
    FnOnce() -> Fut
      + Send + 'static,
  Fut :
    Future < Output = () > + Send
{
  create_partial_session (
    async move |_, sender: Sender<()>| {
      cleaner().await;
      sender.send(()).await;
    })
}

pub fn terminate < Ins >
  () ->
    PartialSession < Ins, End >
where
  Ins : EmptyList + 'static
{
  terminate_async ( async || { } )
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
  < Lens, S, T, D, P, Func, Fut >
  ( _ : Lens,
    cont_builder : Func
  ) ->
    PartialSession < S, P >
where
  S : Processes + 'static,
  T : Processes + 'static,
  D : Processes + 'static,
  P : Process + 'static,
  Func : 
    FnOnce () -> Fut
      + Send + 'static,
  Fut :
    Future <
      Output = PartialSession < T, P >
    > + Send,
  Lens : ProcessLens < S, T, D, End, Inactive >
{
  create_partial_session (
    async move |
      ins1 : <S as Processes>::Values,
      sender
    | {
      let (wait_chan, ins2) =
        < Lens as ProcessLens < S, T, D, End, Inactive > >
        :: split_channels (ins1);

      let ins3 =
        < Lens as ProcessLens < S, T, D, End, Inactive > >
        :: merge_channels ((), ins2);

      wait_chan.recv().await.unwrap();
      let cont = cont_builder().await;
      
      run_partial_session
        ( cont, ins3, sender
        ).await;
    })
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
  wait_async ( lens, async move || {
    cont
  })
}
