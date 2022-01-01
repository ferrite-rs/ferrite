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
    move |ctx, (val_receiver, provider_end)| async move {
      let Value(val) = val_receiver.recv().await.unwrap();

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
  unsafe_create_session(move |ctx1, sender1| async move {
    let ((val_sender, consumer_end), ctx2) = N::extract_source(ctx1);

    let ctx3 = N::insert_target(consumer_end, ctx2);

    val_sender.send(Value(val)).unwrap();

    unsafe_run_session(cont, ctx3, sender1).await;
  })
}
