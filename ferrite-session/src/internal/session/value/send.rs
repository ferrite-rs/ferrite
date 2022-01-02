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
  functional::wrap_type_app,
  protocol::SendValue,
};

pub fn send_value<T, C, A>(
  val: T,
  cont: PartialSession<C, A>,
) -> PartialSession<C, SendValue<T, A>>
where
  T: Send + 'static,
  A: Protocol,
  C: Context,
{
  unsafe_create_session::<C, SendValue<T, A>, _, _>(
    move |ctx, sender1| async move {
      let (provider_end, consumer_end) = A::create_endpoints();
      sender1.send((Value(val), consumer_end)).unwrap();

      unsafe_run_session(cont, ctx, provider_end).await;
    },
  )
}

pub fn receive_value_from<N, C1, C2, T, A, B>(
  _n: N,
  cont: impl FnOnce(T) -> PartialSession<C2, B> + Send + 'static,
) -> PartialSession<C1, B>
where
  A: Protocol,
  B: Protocol,
  C1: Context,
  C2: Context,
  T: Send + 'static,
  N: ContextLens<C1, SendValue<T, A>, A, Target = C2>,
{
  unsafe_create_session(move |ctx1, sender| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let receiver = endpoint.get_applied();

    let (Value(val), consumer_end) = receiver.recv().await.unwrap();

    let ctx3 = N::insert_target(wrap_type_app(consumer_end), ctx2);

    let cont2 = cont(val);

    unsafe_run_session(cont2, ctx3, sender).await;
  })
}
