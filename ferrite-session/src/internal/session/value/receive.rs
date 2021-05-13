use crate::internal::{
  base::{
    once_channel,
    unsafe_create_session,
    unsafe_run_session,
    Context,
    ContextLens,
    PartialSession,
    Protocol,
    SenderOnce,
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
  unsafe_create_session(
    move |ctx, sender1: SenderOnce<ReceiveValue<T, A>>| async move {
      let (sender2, receiver2) = once_channel();

      sender1.send(ReceiveValue(sender2)).unwrap();

      let (Value(val), sender3) = receiver2.recv().await.unwrap();

      let cont2 = cont(val);

      unsafe_run_session(cont2, ctx, sender3).await;
    },
  )
}

pub fn send_value_to<N, C, A, B, T>(
  _: N,
  val: T,
  cont: PartialSession<N::Target, A>,
) -> PartialSession<C, A>
where
  A: Protocol,
  B: Protocol,
  C: Context,
  T: Send + 'static,
  N: ContextLens<C, ReceiveValue<T, B>, B>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let (receiver1, ctx2) = N::extract_source(ctx1);

    let ReceiveValue(sender2) = receiver1.recv().await.unwrap();

    let (sender3, receiver3) = once_channel();

    let ctx3 = N::insert_target(receiver3, ctx2);

    sender2.send((Value(val), sender3)).unwrap();

    unsafe_run_session(cont, ctx3, sender1).await;
  })
}
