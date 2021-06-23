use tokio::{
  task,
  try_join,
};

use crate::internal::{
  base::{
    once_channel,
    unsafe_create_session,
    unsafe_run_session,
    AppendContext,
    Context,
    ContextLens,
    Empty,
    PartialSession,
    Protocol,
  },
  functional::Nat,
  protocol::ReceiveChannel,
};

pub fn receive_channel<C1, C2, N, A, B>(
  cont: impl FnOnce(N) -> PartialSession<C2, B>
) -> PartialSession<C1, ReceiveChannel<A, B>>
where
  N: Nat,
  A: Protocol,
  B: Protocol,
  C1: Context<Length = N>,
  C2: Context,
  C1: AppendContext<(A, ()), Appended = C2>,
{
  let cont2 = cont(N::nat());

  unsafe_create_session(move |ctx1, sender| async move {
    let (sender1, receiver1) = once_channel();

    sender.send(ReceiveChannel(sender1)).unwrap();

    let (receiver2, sender2) = receiver1.recv().await.unwrap();

    let ctx2 = C1::append_context(ctx1, (receiver2, ()));

    unsafe_run_session(cont2, ctx2, sender2).await;
  })
}

pub fn receive_channel_slot<I, P, Q, N>(
  _: N,
  cont: PartialSession<N::Target, Q>,
) -> PartialSession<I, ReceiveChannel<P, Q>>
where
  P: Protocol,
  Q: Protocol,
  I: Context,
  N: ContextLens<I, Empty, P>,
{
  unsafe_create_session(move |ctx1, sender| async move {
    let ((), ctx2) = N::extract_source(ctx1);

    let (sender1, receiver1) = once_channel();

    let child1 = task::spawn(async move {
      sender.send(ReceiveChannel(sender1)).unwrap();
    });

    let child2 = task::spawn(async move {
      let (receiver2, sender2) = receiver1.recv().await.unwrap();

      let ctx3 =
        <N as ContextLens<I, Empty, P>>::insert_target(receiver2, ctx2);

      unsafe_run_session(cont, ctx3, sender2).await;
    });

    try_join!(child1, child2).unwrap();
  })
}

pub fn send_channel_to<N, M, C1, C2, C3, A1, A2, B>(
  _n: N,
  _m: M,
  cont: PartialSession<C3, B>,
) -> PartialSession<C1, B>
where
  C1: Context,
  C2: Context,
  C3: Context,
  A1: Protocol,
  A2: Protocol,
  B: Protocol,
  N: ContextLens<C2, ReceiveChannel<A1, A2>, A2, Target = C3>,
  M: ContextLens<C1, A1, Empty, Target = C2>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let (receiver1, ctx2) = M::extract_source(ctx1);

    let ctx3 = M::insert_target((), ctx2);

    let (receiver2, ctx4) = N::extract_source(ctx3);

    let ReceiveChannel(sender2) = receiver2.recv().await.unwrap();

    let (sender3, receiver3) = once_channel();

    let child1 = task::spawn(async move {
      sender2.send((receiver1, sender3)).unwrap();
    });

    let ctx5 = N::insert_target(receiver3, ctx4);

    let child2 = task::spawn(async move {
      unsafe_run_session(cont, ctx5, sender1).await;
    });

    try_join!(child1, child2).unwrap();
  })
}
