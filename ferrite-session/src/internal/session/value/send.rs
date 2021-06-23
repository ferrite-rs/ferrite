use tokio::{
  task,
  try_join,
};

use crate::internal::{
  base::{
    once_channel,
    unsafe_create_session,
    unsafe_run_session,
    Context,
    ContextLens,
    PartialSession,
    Protocol,
    Value,
  },
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
  unsafe_create_session(move |ctx, sender1| async move {
    let (sender2, receiver2) = once_channel();

    let child1 = task::spawn(async move {
      sender1.send(SendValue((Value(val), receiver2))).unwrap();
    });

    let child2 = task::spawn(async move {
      unsafe_run_session(cont, ctx, sender2).await;
    });

    try_join!(child1, child2).unwrap();
  })
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
    let (receiver1, ctx2) = N::extract_source(ctx1);

    let SendValue((Value(val), receiver2)) = receiver1.recv().await.unwrap();

    let ctx3 = N::insert_target(receiver2, ctx2);

    let cont2 = cont(val);

    unsafe_run_session(cont2, ctx3, sender).await;
  })
}
