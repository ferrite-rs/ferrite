use std::collections::LinkedList;

use crate::internal::{
  base::{
    AppendContext,
    Context,
    ContextLens,
    Empty,
    PartialSession,
    Protocol,
    Session,
  },
  protocol::End,
  session::{
    context::append_emtpy_slot,
    cut::{
      AllRight,
      Cut,
    },
    end::{
      terminate,
      wait,
    },
  },
};

pub fn include_session<C1, C2, N, A, B>(
  session: Session<A>,
  cont: impl FnOnce(N) -> PartialSession<C2, B>,
) -> PartialSession<C1, B>
where
  A: Protocol,
  B: Protocol,
  C1: Context<Length = N>,
  C2: Context,
  C1: AppendContext<(A, ()), Appended = C2>,
{
  AllRight::cut(session, cont)
}

pub fn wait_session<I, P>(
  session1: Session<End>,
  cont: PartialSession<I, P>,
) -> PartialSession<I, P>
where
  P: Protocol,
  I: Context,
  I: AppendContext<(End, ())>,
  I: AppendContext<(Empty, ())>,
  I::Length: ContextLens<
    <I as AppendContext<(End, ())>>::Appended,
    End,
    Empty,
    Target = <I as AppendContext<(Empty, ())>>::Appended,
  >,
{
  include_session(session1, move |chan| wait(chan, append_emtpy_slot(cont)))
}

pub fn wait_sessions<I, P>(
  sessions: Vec<Session<End>>,
  cont: PartialSession<I, P>,
) -> PartialSession<I, P>
where
  P: Protocol,
  I: AppendContext<(End, ())>,
  I: AppendContext<(Empty, ())>,
  I::Length: ContextLens<
    <I as AppendContext<(End, ())>>::Appended,
    End,
    Empty,
    Target = <I as AppendContext<(Empty, ())>>::Appended,
  >,
{
  wait_session(join_sessions(sessions), cont)
}

pub fn join_sessions(sessions: Vec<Session<End>>) -> Session<End>
{
  do_join_sessions(sessions.into_iter().collect())
}

fn do_join_sessions(mut sessions: LinkedList<Session<End>>) -> Session<End>
{
  match sessions.pop_front() {
    Some(session) => include_session(session, move |c1| {
      include_session(do_join_sessions(sessions), move |c2| {
        wait(c1, wait(c2, terminate()))
      })
    }),
    None => terminate(),
  }
}
