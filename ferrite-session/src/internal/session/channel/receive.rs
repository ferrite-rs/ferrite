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

  unsafe_create_session::<C1, ReceiveChannel<A, B>, _, _>(
    move |ctx1, (chan_receiver, provider_end)| async move {
      let consumer_end = chan_receiver.recv().await.unwrap();

      let ctx2 = C1::append_context(ctx1, (wrap_type_app(consumer_end), ()));

      unsafe_run_session(cont2, ctx2, provider_end).await;
    },
  )
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
  unsafe_create_session(move |ctx1, provider_end_b| async move {
    let (consumer_end_1, ctx2) = M::extract_source(ctx1);

    let ctx3 = M::insert_target((), ctx2);

    let (endpoint, ctx4) = N::extract_source(ctx3);

    let (chan_sender, consumer_end_2) = endpoint.get_applied();

    chan_sender.send(consumer_end_1.get_applied()).unwrap();

    let ctx5 = N::insert_target(wrap_type_app(consumer_end_2), ctx4);

    unsafe_run_session(cont, ctx5, provider_end_b).await;
  })
}
