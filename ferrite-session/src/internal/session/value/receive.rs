use crate::internal::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    Context,
    ContextLens,
    PartialSession,
    Protocol,
    Value,
  },
  functional::App,
  protocol::ReceiveValue,
};

pub fn receive_value<T, C, A>(
  cont: impl FnOnce(T) -> PartialSession<C, A> + Send + 'static
) -> PartialSession<C, ReceiveValue<T, A>>
where
  T: Send + 'static,
  A: Protocol,
  C: Context,
{
  unsafe_create_session::<C, ReceiveValue<T, A>, _, _>(
    move |ctx, receiver1| async move {
      let (Value(val), provider_end) = receiver1.recv().await.unwrap();

      let cont2 = cont(val);

      unsafe_run_session(cont2, ctx, provider_end).await;
    },
  )
}

pub fn send_value_to<N, C1, C2, A, B, T>(
  _n: N,
  val: T,
  cont: PartialSession<C2, A>,
) -> PartialSession<C1, A>
where
  A: Protocol,
  B: Protocol,
  C1: Context,
  C2: Context,
  T: Send + 'static,
  N: ContextLens<C1, ReceiveValue<T, B>, B, Target = C2>,
{
  unsafe_create_session(move |ctx1, provider_end_a| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let (provider_end_b, client_end_b) = B::create_endpoints();

    let sender1 = endpoint.get_applied();

    let ctx3 = N::insert_target(App::new(client_end_b), ctx2);

    sender1.send((Value(val), provider_end_b)).unwrap();

    unsafe_run_session(cont, ctx3, provider_end_a).await;
  })
}
