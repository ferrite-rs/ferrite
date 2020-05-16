use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };
use std::collections::{ LinkedList };

use crate::protocol::{ End };

use crate::base::{
  Nat,
  Protocol,
  Session,
  Empty,
  Context,
  AppendContext,
  ContextLens,
  PartialSession,
  unsafe_create_session,
  unsafe_run_session,
};

use crate::context::{
  append_emtpy_slot
};

use crate::session::end::{
  wait,
  terminate,
  wait_async,
};

use crate::session::link::cut;

pub fn include_session
  < C, A, B >
  ( session1 : Session < A >,
    cont_builder : impl FnOnce
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
  let cont = cont_builder (
    C::Length::nat ()
  );

  unsafe_create_session (
    async move | ctx1, sender1 | {
      let (sender2, receiver2) = channel(1);

      let child1 = task::spawn(async move {
        unsafe_run_session
          ( session1, (), sender2
          ).await;
      });

      let ctx2 =
        C :: append_context ( ctx1, (receiver2, ()) );

      let child2 = task::spawn(async move {
        unsafe_run_session
          ( cont, ctx2, sender1
          ).await;
      });

      join!(child1, child2).await;
    })
}

// Version of include_session that uses cut to prove that
// it is derivable from cut. The required constraint is
// more complicated so we don't use this in practice other
// than proving that it also works the same way.
pub fn
  include_session_cut
  < C, A, B >
  ( session : Session < A >,
    cont : impl FnOnce
      ( C :: Length )
      ->
        PartialSession <
          < C as
            AppendContext <
              ( A, () )
            >
          > :: Appended,
          B
        >
  ) ->
    PartialSession < C, B >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  C : AppendContext < ( A, () ) >,
  C : AppendContext < (),
        Appended = C
      >
{
  cut :: <C, (), _, _, _, _ >
    ( cont ( C::Length::nat () ), session )
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
