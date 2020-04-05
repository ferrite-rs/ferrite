// extern crate log;

use std::pin::Pin;
use async_std::task;
use async_macros::join;
use std::future::{ Future };
use async_std::sync::{ Sender, Receiver, channel };

use super::process::{
  Lock,
  SharedProtocol,
  SharedTyApp,
  LinearToShared,
  SharedToLinear,
};

use crate::base::{
  Protocol,
  Empty,
  Context,
  EmptyContext,
  AppendContext,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

use crate::processes::{
  NextSelector
};

pub struct SuspendedSharedSession < P >
where
  P : SharedProtocol
{
  exec_shared_session :
    Box < dyn
      FnOnce
        ( Sender <
            Receiver < P::SharedValue >
          >
        ) ->
          Pin < Box <
            dyn Future <
              Output = ()
            > + Send
          > >
      + Send
    >
}

pub struct SharedSession < P >
where
  P : SharedProtocol
{
  recv_shared_session :
    Sender <
      Sender <
        Receiver <
          P::SharedValue
        >
      >
    >
}

impl < P > Clone for
  SharedSession < P >
where
  P : SharedProtocol
{
  fn clone(&self) -> Self {
    SharedSession {
      recv_shared_session : self.recv_shared_session.clone()
    }
  }
}

pub fn run_shared_session
  < P >
  ( session : SuspendedSharedSession < P > )
  -> SharedSession < P >
where
  P : SharedProtocol
{
  let (sender1, receiver1) = channel (1);
  let (sender2, receiver2) = channel (1);

  task::spawn(async move {
    // debug!("[run_shared_session] exec_shared_session");
    (session.exec_shared_session)(sender1).await;
    // debug!("[run_shared_session] exec_shared_session returned");
  });

  task::spawn(async move {
    loop {
      let sender3
        : Option <
            Sender < Receiver < P :: SharedValue > >
          >
        = receiver2.recv().await;

      // debug!("[run_shared_session] received sender3");
      match sender3 {
        Some ( sender4 ) => {
          let receiver3 = receiver1.recv().await.unwrap();
          // debug!("[run_shared_session] received receiver3");
          sender4.send(receiver3).await;
          // debug!("[run_shared_session] sent receiver3");
        },
        None => {
          debug!("[run_shared_session] terminating shared session");
          return;
        }
      }
    }
  });

  SharedSession {
    recv_shared_session : sender2
  }
}

pub fn
  accept_shared_session
  < F >
  ( cont : PartialSession <
      (Lock < F >, ()),
      < F as
        SharedTyApp < F >
      > :: ToProtocol
    >
  ) ->
    SuspendedSharedSession <
      LinearToShared < F >
    >
where
  F : SharedTyApp < F > + Send + 'static
{
  SuspendedSharedSession {
    exec_shared_session : Box::new (
      move | sender1 | {
        Box::pin ( async move {
          let (sender2, receiver2) = channel (1);
          let (sender3, receiver3) = channel (1);

          let child1 = task::spawn ( async move {
            // debug!("[accept_shared_session] calling cont");
            unsafe_run_session
              ( cont, (receiver2, ()), sender3 ).await;
            // debug!("[accept_shared_session] returned from cont");
          });

          let sender12 = sender1.clone();

          let child2 = task::spawn ( async move {
            // debug!("[accept_shared_session] sending receiver3");
            sender1.send(receiver3).await;
            // debug!("[accept_shared_session] sent receiver3");
          });

          let child3 = task::spawn ( async move {
            // debug!("[accept_shared_session] sending sender12");
            sender2.send(sender12).await;
            // debug!("[accept_shared_session] sent sender12");
          });

          join! ( child1, child2, child3 ).await;
        })
      })
  }
}

pub fn
  detach_shared_session
  < F, I >
  ( cont : SuspendedSharedSession <
      LinearToShared < F >
    >
  ) ->
    PartialSession <
      (Lock < F >, I),
      SharedToLinear < F >
    >
where
  F : SharedTyApp < F > + Send + 'static,
  I : EmptyContext
{
  unsafe_create_session (
    async move |
      (receiver1, _)
        : ( Receiver <
              Sender <
                Receiver<
                  < < F as SharedTyApp < F > >
                    :: ToProtocol
                    as Protocol
                  > :: Payload
                >
              >
            >,
            I :: Values
          ),
      sender1
    | {
      let child1 = task::spawn ( async move {
        // debug!("[detach_shared_session] receiving sender2");
        let sender2 = receiver1.recv().await.unwrap();

        // debug!("[detach_shared_session] received sender2");
        (cont.exec_shared_session)(sender2).await;
        // debug!("[detach_shared_session] ran cont");
      });

      let child2 = task::spawn ( async move {
        // debug!("[detach_shared_session] sending sender1");
        sender1.send(()).await;
        // debug!("[detach_shared_session] sent sender1");
      });

      join! ( child1, child2 ).await;
    })
}

pub fn
  acquire_shared_session
  < F, I, P, Cont >
  ( shared : SharedSession <
      LinearToShared < F >
    >,
    cont_builder : Cont
  ) ->
    PartialSession < I, P >
where
  F : SharedTyApp < F > + 'static,
  P : Protocol,
  I : Context + NextSelector + 'static,
  I : AppendContext <
        ( < F as
            SharedTyApp < F >
          > :: ToProtocol
        , ()
        )
      >,
  Cont : FnOnce
        ( < I as NextSelector > :: Selector )
        ->
          PartialSession <
            < I as
              AppendContext <
                ( < F as
                    SharedTyApp < F >
                  > :: ToProtocol
                , ()
                )
              >
            > :: Appended,
            P
          >
{

  let cont = cont_builder (
    < I as NextSelector > :: make_selector ()
  );

  unsafe_create_session (
    async move | ctx1, sender1 | {
      let (sender2, receiver2) = channel (1);

      let child1 = task::spawn ( async move {
        // debug!("[acquire_shared_session] sending sender2");
        shared.recv_shared_session.send(sender2).await;
        // debug!("[acquire_shared_session] sent sender2");
      });

      let child2 = task::spawn ( async move {
        // debug!("[acquire_shared_session] receiving receiver4");
        let receiver4 = receiver2.recv().await.unwrap();
        // debug!("[acquire_shared_session] received receiver4");

        let ctx2 =
          < I as
            AppendContext <
              ( < F as
                  SharedTyApp < F >
                > :: ToProtocol
              , ()
              )
            >
          > :: append_context ( ctx1, (receiver4, ()) );

        unsafe_run_session
          ( cont, ctx2, sender1
          ).await;

        // debug!("[acquire_shared_session] ran cont");
      });

      join! (child1, child2).await;
    })
}

pub fn
  release_shared_session
  < F, I, P, N >
  ( _ : N,
    cont :
      PartialSession <
        N :: Target,
        P
      >
  ) ->
    PartialSession <
      I,
      P
    >
where
  P : Protocol,
  I : Context,
  F : SharedTyApp < F > + Send + 'static,
  N :
    ContextLens <
      I,
      SharedToLinear < F >,
      Empty
    >,
{
  unsafe_create_session (
    async move | ctx1, sender1 | {
      let (receiver2, ctx2) =
        N :: extract_source ( ctx1 );

      let ctx3 =
        N :: insert_target ( (), ctx2 );

      // debug!("[release_shared_session] waiting receiver2");
      receiver2.recv().await.unwrap();
      // debug!("[release_shared_session] received receiver2");
      unsafe_run_session
        ( cont, ctx3, sender1
        ).await;
      // debug!("[release_shared_session] ran cont");
    })
}
