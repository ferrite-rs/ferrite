use std::future::Future;

use crate::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    Context,
    ContextLens,
    Empty,
    EmptyContext,
    PartialSession,
    Protocol,
    Session,
  },
  protocol::End,
};

/*

         cleaner() :: ()
   ===============================
     terminate_async (cleaner) :: · ⊢ 1

   Create a unit protocol (1) out of nothing.
*/

pub fn terminate_async<C, Func, Fut>(cleaner : Func) -> PartialSession<C, End>
where
  C : EmptyContext,
  Func : FnOnce() -> Fut + Send + 'static,
  Fut : Future<Output = ()> + Send,
{
  unsafe_create_session(move |_, sender| async move {
    cleaner().await;

    sender.send(End()).unwrap();
  })
}

pub fn terminate<C>() -> PartialSession<C, End>
where
  C : EmptyContext,
{
  terminate_async(|| async {})
}

pub fn terminate_nil() -> Session<End>
{
  terminate()
}

/*
         cont :: Δ ⊢ P
   ===========================
     wait_async (cont) :: 1, Δ ⊢ P

   Wait for a given input protocol to terminate, then continue as P.
*/

pub fn wait<N, C, A>(
  _ : N,
  cont : PartialSession<N::Target, A>,
) -> PartialSession<C, A>
where
  C : Context,
  A : Protocol,
  N : ContextLens<C, End, Empty>,
{
  unsafe_create_session(move |ctx1, sender| async move {
    let (receiver, ctx2) = N::extract_source(ctx1);

    let ctx3 = N::insert_target((), ctx2);

    receiver.recv().await.unwrap();

    unsafe_run_session(cont, ctx3, sender).await;
  })
}
