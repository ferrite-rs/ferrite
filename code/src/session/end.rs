
use async_std::sync::{ Sender };
use std::future::{ Future };

use crate::process::{ End };

use crate::base::{
  Protocol,
  Session,
  Empty,
  Context,
  EmptyContext,
  ContextLens,
  PartialSession,
  run_partial_session,
  unsafe_create_session,
};

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
  Ins : EmptyContext,
  Func :
    FnOnce() -> Fut
      + Send + 'static,
  Fut :
    Future < Output = () > + Send
{
  unsafe_create_session (
    async move |_, sender: Sender<()>| {
      cleaner().await;
      sender.send(()).await;
    })
}

pub fn terminate < Ins >
  () ->
    PartialSession < Ins, End >
where
  Ins : EmptyContext
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
  < N, I, P, Func, Fut >
  ( _ : N,
    cont_builder : Func
  ) ->
    PartialSession < I, P >
where
  I : Context,
  P : Protocol,
  Func :
    FnOnce () -> Fut
      + Send + 'static,
  Fut :
    Future <
      Output =
        PartialSession <
          N :: Target,
          P
        >
    > + Send,
  N : ContextLens < I, End, Empty >
{
  unsafe_create_session (
    async move |
      ins1 : I :: Values,
      sender
    | {
      let (wait_chan, ins2) =
        < N as
          ContextLens <
            I,
            End,
            Empty
          >
        > :: split_channels (ins1);

      let ins3 =
        < N as
          ContextLens <
            I,
            End,
            Empty
          >
        > :: merge_channels ((), ins2);

      wait_chan.recv().await.unwrap();
      let cont = cont_builder().await;

      run_partial_session
        ( cont, ins3, sender
        ).await;
    })
}

pub fn wait
  < N, I, P >
  ( lens : N,
    cont :
      PartialSession <
        N :: Target,
        P
      >
  ) ->
    PartialSession < I, P >
where
  I : Context,
  P : Protocol,
  N : ContextLens < I, End, Empty >
{
  wait_async ( lens, async move || {
    cont
  })
}
