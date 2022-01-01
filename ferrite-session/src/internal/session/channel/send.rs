use tokio::{
  task,
  try_join,
};

use crate::internal::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    AppendContext,
    Context,
    ContextLens,
    Empty,
    PartialSession,
    Protocol,
  },
  functional::{
    wrap_type_app,
    Nat,
  },
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
  unsafe_create_session::<C1, SendChannel<A, B>, _, _>(
    move |ctx1, (chan_sender, provider_end)| async move {
      let (endpoint, ctx2) = N::extract_source(ctx1);

      let consumer_end = endpoint.get_applied();

      let ctx3 = N::insert_target((), ctx2);

      chan_sender.send(consumer_end).unwrap();

      unsafe_run_session(cont, ctx3, provider_end).await;
    },
  )
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

  unsafe_create_session(move |ctx1, provider_end| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let (chan_receiver, consumer_end2) = endpoint.get_applied();

    let consumer_end1 = chan_receiver.recv().await.unwrap();

    let ctx3 = N::insert_target(wrap_type_app(consumer_end2), ctx2);

    let ctx4 = <N::Target as AppendContext<(A1, ())>>::append_context(
      ctx3,
      (wrap_type_app(consumer_end1), ()),
    );

    unsafe_run_session(cont2, ctx4, provider_end).await;
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

pub fn fork<A, B, C1, C2>(
  cont1: PartialSession<C1, A>,
  cont2: PartialSession<C2, B>,
) -> PartialSession<C1::Appended, SendChannel<A, B>>
where
  A: Protocol,
  B: Protocol,
  C1: Context,
  C2: Context,
  C1: AppendContext<C2>,
{
  unsafe_create_session::<C1::Appended, SendChannel<A, B>, _, _>(
    move |ctx, (chan_sender, provider_end_b)| async move {
      let (ctx1, ctx2) = C1::split_context(ctx);

      let (provider_end_a, consumer_end_a) = A::create_endpoints();

      let child1 = task::spawn(async move {
        unsafe_run_session(cont1, ctx1, provider_end_a).await;
      });

      chan_sender.send(consumer_end_a).unwrap();

      let child2 = task::spawn(async move {
        unsafe_run_session(cont2, ctx2, provider_end_b).await;
      });

      try_join!(child1, child2).unwrap();
    },
  )
}
