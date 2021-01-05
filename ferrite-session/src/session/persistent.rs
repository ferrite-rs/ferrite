use std::sync::Arc;
use async_std::task;
use async_macros::join;

use crate::base::*;
use crate::functional::nat::*;

pub struct PersistentSession < P >
where
  P : Protocol
{
  new_session : Arc <
    dyn Fn () -> Session < P >
      + Send + Sync
  >
}

impl < P >
  Clone for
  PersistentSession < P >
where
  P : Protocol
{
  fn clone(&self) -> Self {
    PersistentSession {
      new_session : self.new_session.clone()
    }
  }
}

pub fn create_persistent_session
  < F, P >
  (f : F)
  -> PersistentSession < P >
where
  P : Protocol,
  F : Fn () -> Session < P >
      + Send + Sync + 'static
{
  return PersistentSession {
    new_session: Arc::new ( f )
  }
}

pub fn
  clone_session
  < I, P, Q, F >
  ( session1 : &PersistentSession < P >,
    cont_builder : F
  ) ->
    PartialSession < I, Q >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  I : AppendContext < ( P, () ) >,
  F : FnOnce
        ( I::Length )
        ->
          PartialSession <
            < I as
              AppendContext <
                ( P, () )
              >
            > :: Appended,
            Q
          >
{
  let session2 = session1.clone();

  let cont = cont_builder (
    I::Length::nat()
  );

  unsafe_create_session (
    move | ctx1, sender1 | async move {
      let session3 = (session2.new_session)();

      let (sender2, receiver2) = once_channel();

      let child1 = task::spawn(async move {
        unsafe_run_session
          ( session3, (), sender2
          ).await;
      });

      let ctx2 =
        < I as
          AppendContext <
            ( P, () )
          >
        > :: append_context ( ctx1, (receiver2, ()) );

      let child2 = task::spawn(async move {
        unsafe_run_session
          ( cont, ctx2, sender1
          ).await;
      });

      join!(child1, child2).await;
    })
}
