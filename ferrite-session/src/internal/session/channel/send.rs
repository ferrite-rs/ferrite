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
  protocol::SendChannel,
};

pub fn send_channel_from<C1, C2, N, A, B>(
  _n: N,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, SendChannel<A, B>>
where
  A: Protocol,
  B: Protocol,
  C1: Context,
  C2: Context,
  N: ContextLens<C1, A, Empty, Target = C2>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let (p_chan, ctx2) = N::extract_source(ctx1);

    let (sender2, receiver2) = once_channel();

    let (sender3, receiver3) = once_channel();

    let ctx3 = N::insert_target((), ctx2);

    let child1 = task::spawn(async move {
      let p = p_chan.recv().await.unwrap();

      sender2.send(p).unwrap();
    });

    let child2 = task::spawn(async move {
      sender1.send(SendChannel(receiver2, receiver3)).unwrap();
    });

    let child3 = task::spawn(async {
      unsafe_run_session(cont, ctx3, sender3).await;
    });

    try_join!(child1, child2, child3).unwrap();
  })
}

pub fn receive_channel_from<C1, C2, C3, N, M, A1, A2, B>(
  _n: N,
  cont1: impl FnOnce(M) -> PartialSession<C3, B>,
) -> PartialSession<C1, B>
where
  A1: Protocol,
  A2: Protocol,
  B: Protocol,
  C1: Context<Length = M>,
  C2: AppendContext<(A1, ()), Appended = C3>,
  C3: Context,
  N: ContextLens<C1, SendChannel<A1, A2>, A2, Target = C2>,
  M: Nat,
{
  let cont2 = cont1(M::nat());

  unsafe_create_session(move |ctx1, sender1| async move {
    let (pair_chan, ctx2) = N::extract_source(ctx1);

    let SendChannel(p_chan, y_chan) = pair_chan.recv().await.unwrap();

    let ctx3 = N::insert_target(y_chan, ctx2);

    let ctx4 = <N::Target as AppendContext<(A1, ())>>::append_context(
      ctx3,
      (p_chan, ()),
    );

    unsafe_run_session(cont2, ctx4, sender1).await;
  })
}

/*
   Multiplicative Conjunction, Alternative Parallel Version


      cont1 :: Δ ⊢ P    cont2 :: Δ'  ⊢ Q
   ========================================
     fork(cont1, cont2) :: Δ, Δ' ⊢ P ⊗ Q

   Takes in two session builders and return a new session builder
   with its inputs combined and outputs a parallel context
*/

pub fn fork<P, Q, CP, CQ>(
  cont1: PartialSession<CP, P>,
  cont2: PartialSession<CQ, Q>,
) -> PartialSession<<CP as AppendContext<CQ>>::Appended, SendChannel<P, Q>>
where
  P: Protocol,
  Q: Protocol,
  CP: Context,
  CQ: Context,
  CP: AppendContext<CQ>,
  P: 'static,
  Q: 'static,
  CP: 'static,
  CQ: 'static,
{
  unsafe_create_session(move |ctx, sender| async move {
    let (ctx1, ctx2) = CP::split_context(ctx);

    let (sender1, receiver1) = once_channel();

    let (sender2, receiver2) = once_channel();

    // the first thread task::spawns immediately

    let child1 = task::spawn(async move {
      unsafe_run_session(cont1, ctx1, sender1).await;
    });

    // the sender here blocks until the inner channel pairs
    // are received on the other side
    let child2 = task::spawn(async move {
      sender.send(SendChannel(receiver1, receiver2)).unwrap();
    });

    // the second thread is blocked until the first channel is being accessed

    let child3 = task::spawn(async move {
      unsafe_run_session(cont2, ctx2, sender2).await;
    });

    try_join!(child1, child2, child3).unwrap();
  })
}

pub fn receive_channel_from_slot<I, P1, P2, Q, TargetLens, SourceLens>(
  _: SourceLens,
  _: TargetLens,
  cont: PartialSession<TargetLens::Target, Q>,
) -> PartialSession<I, Q>
where
  P1: Protocol,
  P2: Protocol,
  Q: Protocol,
  I: Context,
  SourceLens: ContextLens<I, SendChannel<P1, P2>, P2>,
  TargetLens: ContextLens<SourceLens::Target, Empty, P1>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let (pair_chan, ctx2) = SourceLens::extract_source(ctx1);

    let SendChannel(p_chan, y_chan) = pair_chan.recv().await.unwrap();

    let ctx3 = SourceLens::insert_target(y_chan, ctx2);

    let ((), ctx4) = TargetLens::extract_source(ctx3);

    let ctx5 = TargetLens::insert_target(p_chan, ctx4);

    unsafe_run_session(cont, ctx5, sender1).await;
  })
}
