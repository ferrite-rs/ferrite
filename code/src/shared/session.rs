extern crate log;

use crate::base::*;
use crate::processes::*;
use crate::shared::process::*;
use async_std::sync::{ Sender, Receiver, channel };
use std::pin::Pin;
use std::future::{ Future };
use async_std::task;
use async_macros::join;

pub struct SuspendedSharedSession < P >
where
  P : SharedProcess
{
  pub exec_shared_session :
    Box < dyn
      FnOnce
        ( Sender <
            Receiver < P::SharedValue >
          >
        ) ->
          Pin < Box < dyn Future <
            Output = ()
          > + Send > >
      + Send
    >
}

pub struct SharedSession < P >
where
  P : SharedProcess
{
  pub recv_shared_session :
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
  P : SharedProcess
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
  P : SharedProcess + 'static
{
  let (sender1, receiver1) = channel (999);
  let (sender2, receiver2) = channel (999);

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
        SharedAlgebra < F >
      > :: ToProcess
    >
  ) ->
    SuspendedSharedSession <
      LinearToShared < F >
    >
where
  F : SharedAlgebra < F > + 'static
{
  SuspendedSharedSession {
    exec_shared_session : Box::new(
      move | sender1 | {
        Box::pin ( async move {
          let (sender2, receiver2) = channel (999);
          let (sender3, receiver3) = channel (999);

          let child1 = task::spawn ( async move {
            // debug!("[accept_shared_session] calling cont");
            (cont.builder)( (receiver2, ()), sender3 ).await;
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
  F : SharedAlgebra < F > + 'static,
  I : EmptyList
{
  PartialSession {
    builder:
      Box::new(move |
        (receiver1, _),
        sender1
      | {
        Box::pin ( async move {
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
      })
  }
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
  F : SharedAlgebra < F > + 'static,
  P : Process + 'static,
  I : Processes + NextSelector + 'static,
  I : Appendable <
        ( < F as
            SharedAlgebra < F >
          > :: ToProcess
        , ()
        )
      >,
  Cont : FnOnce
        ( < I as NextSelector > :: Selector )
        ->
          PartialSession <
            < I as
              Appendable <
                ( < F as
                    SharedAlgebra < F >
                  > :: ToProcess
                , ()
                )
              >
            > :: AppendResult,
            P
          >
{

  let cont = cont_builder(
    < I as NextSelector > :: make_selector ()
  );

  PartialSession {
    builder : Box::new(
      move | ins1, sender1 | {
        Box::pin ( async move {
          let (sender2, receiver2) = channel (999);

          let child1 = task::spawn ( async move {
            // debug!("[acquire_shared_session] sending sender2");
            shared.recv_shared_session.send(sender2).await;
            // debug!("[acquire_shared_session] sent sender2");
          });

          let child2 = task::spawn ( async move {
            // debug!("[acquire_shared_session] receiving receiver4");
            let receiver4 = receiver2.recv().await.unwrap();
            // debug!("[acquire_shared_session] received receiver4");

            let ins2 =
              < I as
                Appendable <
                  ( < F as
                      SharedAlgebra < F >
                    > :: ToProcess
                  , ()
                  )
                >
              > :: append_channels ( ins1, (receiver4, ()) );

            (cont.builder)(ins2, sender1).await;
            // debug!("[acquire_shared_session] ran cont");
          });

          join! (child1, child2).await;
        })
      })
  }
}

pub fn
  release_shared_session
  < F, I1, I2, I3, P, Lens >
  ( _ : Lens,
    cont : PartialSession < I2, P >
  ) ->
    PartialSession <
      I1,
      P
    >
where
  P : Process + 'static,
  I1 : Processes + 'static,
  I2 : Processes + 'static,
  I3 : Processes + 'static,
  F : SharedAlgebra < F >,
  Lens :
    ProcessLens <
      I1,
      I2,
      I3,
      SharedToLinear < F >,
      Inactive
    >,
{
  PartialSession {
    builder : Box::new(
      move | ins1, sender1 | {
        Box::pin ( async move {
          let (receiver2, ins2) =
            < Lens as
              ProcessLens <
                I1,
                I2,
                I3,
                SharedToLinear < F >,
                Inactive
              >
            > :: split_channels ( ins1 );

          let ins3 =
            < Lens as
              ProcessLens <
                I1,
                I2,
                I3,
                SharedToLinear < F >,
                Inactive
              >
            > :: merge_channels ( (), ins2 );

          // debug!("[release_shared_session] waiting receiver2");
          receiver2.recv().await.unwrap();
          // debug!("[release_shared_session] received receiver2");
          (cont.builder)(ins3, sender1).await;
          // debug!("[release_shared_session] ran cont");
        })
      })
  }
}
