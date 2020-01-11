use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };
use std::collections::{ LinkedList };

use crate::process::{ End };

use crate::base::{
  Process,
  Session,
  Inactive,
  Processes,
  Appendable,
  ProcessLens,
  PartialSession,
  run_partial_session,
  create_partial_session,
};

use crate::processes::{
  NextSelector,
  append_emtpy_slot
};

use crate::session::end::{
  wait,
  terminate,
  wait_async,
};

pub fn
  include_session
  < I, P, Q, F >
  ( session1 : Session < P >,
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
  let cont = cont_builder (
    < I as NextSelector > :: make_selector ()
  );

  create_partial_session (
    async move | ins1, sender1 | {
      let (sender2, receiver2) = channel(1);

      let child1 = task::spawn(async move {
        run_partial_session
          ( session1, (), sender2
          ).await;
      });

      let ins2 =
        < I as
          Appendable <
            ( P, () )
          >
        > :: append_channels ( ins1, (receiver2, ()) );

      let child2 = task::spawn(async move {
        run_partial_session
          ( cont, ins2, sender1
          ).await;
      });

      join!(child1, child2).await;
    })
}

pub fn wait_session
  < I, P >
  ( session1 : Session < End >,
    cont : PartialSession < I, P >
  ) ->
    PartialSession < I, P >
where
  P : Process + 'static,
  I : NextSelector + 'static,
  I : Appendable < (End, ()) >,
  I : Appendable < (Inactive, ()) >,
  < I as NextSelector >::Selector :
    ProcessLens <
      < I as
        Appendable < (End, ()) >
      >::AppendResult,
      End,
      Inactive,
      Target =
        < I as
          Appendable < (Inactive, ()) >
        >::AppendResult
    >
{
  include_session ( session1, move | chan | {
    wait_async ( chan, async move || {
      append_emtpy_slot ( cont )
    })
  })
}

pub fn wait_sessions
  < I, P >
  ( sessions :
      Vec <
        Session < End >
      >,
    cont : PartialSession < I, P >
  ) ->
    PartialSession < I, P >
where
  P : Process + 'static,
  I : NextSelector + 'static,
  I : Appendable < (End, ()) >,
  I : Appendable < (Inactive, ()) >,
  < I as NextSelector >::Selector :
    ProcessLens <
      < I as
        Appendable < (End, ()) >
      >::AppendResult,
      End,
      Inactive,
      Target =
        < I as
          Appendable < (Inactive, ()) >
        >::AppendResult
    >
{
  wait_session (
    join_sessions (sessions) ,
    cont
  )
}

pub fn join_sessions
  ( sessions :
      Vec <
        Session < End >
      >
  ) ->
    Session < End >
{
  do_join_sessions ( sessions.into_iter().collect() )
}

fn do_join_sessions
  ( mut sessions :
      LinkedList <
        Session < End >
      >
  ) ->
    Session < End >
{
  match sessions.pop_front() {
    Some (session) => {
      include_session ( session, move | c1 | {
        include_session (
          do_join_sessions ( sessions ),
          move | c2 | {
            wait ( c1,
              wait ( c2,
                terminate ()))
          })
      })
    },
    None => {
      terminate()
    }
  }
}