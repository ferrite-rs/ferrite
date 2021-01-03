
use std::future::{ Future };

use crate::protocol::{ End };

use crate::base::{
  Protocol,
  Session,
  Empty,
  Context,
  EmptyContext,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

/*

          cleaner() :: ()
    ===============================
      terminate_async (cleaner) :: · ⊢ 1

    Create a unit protocol (1) out of nothing.
 */
pub fn terminate_async < C, Func, Fut >
  ( cleaner: Func )
  -> PartialSession < C, End >
where
  C : EmptyContext,
  Func :
    FnOnce() -> Fut
      + Send + 'static,
  Fut :
    Future < Output = () > + Send
{
  unsafe_create_session (
    move | _, sender | async move {
      cleaner().await;
      sender.send( End () ).await.unwrap();
    })
}

pub fn terminate < C >
  () ->
    PartialSession < C, End >
where
  C : EmptyContext
{
  terminate_async ( || async { } )
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

    Wait for a given input protocol to terminate, then continue as P.
 */

pub fn wait_async
  < N, C, A, Fut >
  ( _ : N,
    cont_builder : impl
      FnOnce () -> Fut
      + Send + 'static
  ) ->
    PartialSession < C, A >
where
  C : Context,
  A : Protocol,
  Fut :
    Future <
      Output =
        PartialSession <
          N :: Target,
          A
        >
    > + Send,
  N : ContextLens < C, End, Empty >
{
  unsafe_create_session (
    move | ctx1, sender | async move {
      let (receiver, ctx2) =
        N :: extract_source (ctx1);

      let ctx3 =
        N :: insert_target ((), ctx2);

      receiver.recv().await.unwrap();
      let cont = cont_builder().await;

      unsafe_run_session
        ( cont, ctx3, sender
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
  wait_async ( lens, move || async move {
    cont
  })
}
