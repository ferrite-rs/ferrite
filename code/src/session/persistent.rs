use std::sync::Arc;
use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };

use crate::base::*;
use crate::processes::*;

pub struct PersistentSession < P >
where
  P : Process
{
  pub new_session : Arc <
    dyn Fn () -> Session < P >
      + Send + Sync
  >
}

impl < P >
  Clone for
  PersistentSession < P >
where
  P : Process
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
  P : Process,
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
  P : Process + 'static,
  Q : Process + 'static,
  I : Processes + NextSelector + 'static,
  I : Appendable < ( P, () ) >,
  F : FnOnce
        ( < I as NextSelector > :: Selector )
        ->
          PartialSession <
            < I as
              Appendable <
                ( P, () )
              >
            > :: AppendResult,
            Q
          >
{
  let session2 = session1.clone();

  let cont = cont_builder (
    < I as NextSelector > :: make_selector ()
  );

  return PartialSession {
    builder: Box::new( move | ins1, sender1 | {
      Box::pin(async move {
        let session3 = (session2.new_session)();

        let (sender2, receiver2) = channel(1);

        let child1 = task::spawn(async move {
          (session3.builder)((), sender2).await;
        });

        let ins2 =
          < I as
            Appendable <
              ( P, () )
            >
          > :: append_channels ( ins1, (receiver2, ()) );

        let child2 = task::spawn(async move {
          (cont.builder)(ins2, sender1).await;
        });

        join!(child1, child2).await;
      })
    })
  }
}