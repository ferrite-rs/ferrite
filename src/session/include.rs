use std::collections::{ LinkedList };

use crate::protocol::{ End };

use crate::base::{
  Protocol,
  Session,
  Empty,
  Context,
  AppendContext,
  ContextLens,
  PartialSession,
};

use crate::context::{
  append_emtpy_slot,
};

use crate::session::end::{
  wait,
  terminate,
  wait_async,
};

use crate::session::cut::{
  Cut,
  First
};

pub fn include_session
  < C, A, B >
  ( session : Session < A >,
    cont : impl FnOnce
      ( C :: Length )
      ->
        PartialSession <
          C :: Appended,
          B
        >
  ) ->
    PartialSession < C, B >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  C : AppendContext < ( A, () ) >,
{
  First :: cut ( cont, session  )
}

pub fn wait_session
  < I, P >
  ( session1 : Session < End >,
    cont : PartialSession < I, P >
  ) ->
    PartialSession < I, P >
where
  P : Protocol,
  I : Context,
  I : AppendContext < (End, ()) >,
  I : AppendContext < (Empty, ()) >,
  I::Length :
    ContextLens <
      < I as
        AppendContext < (End, ()) >
      >::Appended,
      End,
      Empty,
      Target =
        < I as
          AppendContext < (Empty, ()) >
        >::Appended
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
  P : Protocol,
  I : AppendContext < (End, ()) >,
  I : AppendContext < (Empty, ()) >,
  I::Length :
    ContextLens <
      < I as
        AppendContext < (End, ()) >
      >::Appended,
      End,
      Empty,
      Target =
        < I as
          AppendContext < (Empty, ()) >
        >::Appended
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
