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
  protocol::{
    End,
    ReceiveChannel,
  },
  session::{
    channel::send_channel_to,
    end::wait,
    forward::forward,
    include::include_session,
  },
};

pub fn apply_channel<A, B>(
  f: Session<ReceiveChannel<A, B>>,
  a: Session<A>,
) -> Session<B>
where
  A: Protocol,
  B: Protocol,
{
  include_session(f, move |c1| {
    include_session(a, move |c2| send_channel_to(c1, c2, forward(c1)))
  })
}

pub fn send_channel_to_session<N, C1, C2, C3, C4, C5, A, B>(
  n: N,
  session: Session<ReceiveChannel<A, End>>,
  cont: PartialSession<C5, B>,
) -> PartialSession<C1, B>
where
  C1: Context,
  C2: Context,
  C3: Context,
  C4: Context,
  C5: Context,
  A: Protocol,
  B: Protocol,
  C1: AppendContext<(ReceiveChannel<A, End>, ()), Appended = C2>,
  C1::Length: ContextLens<C3, ReceiveChannel<A, End>, End, Target = C4>,
  C1::Length: ContextLens<C4, End, Empty, Target = C5>,
  N: ContextLens<C2, A, Empty, Target = C3>,
{
  include_session(session, |chan| send_channel_to(chan, n, wait(chan, cont)))
}
