// extern crate log;

use async_std::task;
use async_macros::join;
use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver, channel };

use super::lock::Lock;
use super::fix::SharedTypeApp;
use super::protocol::SharedProtocol;
use super::linear_to_shared::LinearToShared;
use super::shared_to_linear::SharedToLinear;
use super::shared_session::*;

use crate::base::{
  Protocol,
  Nat,
  Empty,
  Context,
  EmptyContext,
  AppendContext,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

pub fn run_shared_session
  < P >
  ( session : SuspendedSharedSession < P > )
  -> SharedSession < P >
where
  P : SharedProtocol
{
  let (sender1, receiver1) = channel (1);

  let ( session2, receiver2 ) = unsafe_offer_shared_session ();

  task::spawn(async move {
    // debug!("[run_shared_session] exec_shared_session");
    unsafe_run_shared_session ( session, sender1 ).await;
    // debug!("[run_shared_session] exec_shared_session returned");
  });

  task::spawn(async move {
    loop {
      let sender3
        : Option <
            Sender < Receiver < P > >
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

  session2
}

pub fn
  accept_shared_session
  < F >
  ( cont : PartialSession <
      (Lock < F >, ()),
      F :: Applied
    >
  ) ->
    SuspendedSharedSession <
      LinearToShared < F >
    >
where
  F : Send + 'static,
  F : SharedTypeApp < SharedToLinear < F > >
{
  unsafe_create_shared_session (
    async move |
      sender1 :
        Sender < Receiver <
          LinearToShared < F >
        > >
    | {
        let (sender2, receiver2)
          : (Sender < Lock < F > >, _)
          = channel (1);

        let (sender3, receiver3)
          : (Sender < LinearToShared < F > >, _)
          = channel (1);

        let (sender4, receiver4)
          : (Sender < F :: Applied >, _)
          = channel (1);

        let child1 = task::spawn ( async move {
          // debug!("[accept_shared_session] calling cont");
          unsafe_run_session
            ( cont, (receiver2, ()), sender4 ).await;
          // debug!("[accept_shared_session] returned from cont");
        });

        let child2 = task::spawn ( async move {
          let linear = receiver4.recv().await.unwrap();
          sender3.send ( LinearToShared { linear: linear } ).await;
        });

        let sender12 = sender1.clone();

        let child3 = task::spawn ( async move {
          // debug!("[accept_shared_session] sending receiver3");
          sender1.send(
            receiver3
          ).await;
          // debug!("[accept_shared_session] sent receiver3");
        });

        let child4 = task::spawn ( async move {
          // debug!("[accept_shared_session] sending sender12");
          sender2.send(
            Lock { unlock : sender12 }
          ).await;
          // debug!("[accept_shared_session] sent sender12");
        });

        join! ( child1, child2, child3, child4 ).await;
      }
  )
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
  F : Send + 'static,
  F : SharedTypeApp < SharedToLinear < F > >,
  I : EmptyContext
{
  unsafe_create_session (
    async move |
      (receiver1, _)
        : ( Receiver <
              Lock < F >
            >,
            I :: Endpoints
          ),
      sender1
    | {
      let child1 = task::spawn ( async move {
        // debug!("[detach_shared_session] receiving sender2");
        let Lock { unlock :  sender2 }
          = receiver1.recv().await.unwrap();

        // debug!("[detach_shared_session] received sender2");
        unsafe_run_shared_session ( cont, sender2 ).await;
        // debug!("[detach_shared_session] ran cont");
      });

      let child2 = task::spawn ( async move {
        // debug!("[detach_shared_session] sending sender1");
        sender1.send (
          SharedToLinear ( PhantomData )
        ).await;
        // debug!("[detach_shared_session] sent sender1");
      });

      join! ( child1, child2 ).await;
    })
}

pub fn
  acquire_shared_session
  < F, C, A >
  ( shared : SharedSession <
      LinearToShared < F >
    >,
    cont_builder : impl
      FnOnce
        ( C :: Length )
        ->
          PartialSession <
            C :: Appended,
            A
          >
  ) ->
    PartialSession < C, A >
where
  C : Context,
  A : Protocol,
  F : Send + 'static,
  F : SharedTypeApp < SharedToLinear < F > >,
  C :
    AppendContext <
      ( F :: Applied , () )
    >,
{
  let cont = cont_builder (
    < C:: Length as Nat > :: nat ()
  );

  unsafe_create_session (
    async move | ctx1, sender1 | {
      let (sender2, receiver2) = channel (1);
      let (sender3, receiver3) = channel (1);

      let child1 = task::spawn ( async move {
        // debug!("[acquire_shared_session] sending sender2");
        unsafe_receive_shared_session ( shared, sender2 ).await;
        // debug!("[acquire_shared_session] sent sender2");
      });

      let child2 = task::spawn ( async move {
        // debug!("[acquire_shared_session] receiving receiver4");
        let receiver4 = receiver2.recv().await.unwrap();
        // debug!("[acquire_shared_session] received receiver4");

        let ctx2 =
          C :: append_context ( ctx1, (receiver3, ()) );

        let child21 = task::spawn ( async move {
          let LinearToShared { linear } = receiver4.recv().await.unwrap();
          sender3.send(linear).await;
        });

        let child22 = task::spawn ( async move {
          unsafe_run_session
            ( cont, ctx2, sender1
            ).await;
        });

        join! (child21, child22).await;

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
  F : Send + 'static,
  F : SharedTypeApp < SharedToLinear < F > >,
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
