use tokio::task;

use crate::internal::{
  base::*,
  functional::*,
  protocol::{
    End,
    LinearToShared,
    Lock,
    SendValue,
    SharedToLinear,
  },
};

pub fn accept_shared_session<F>(
  cont: PartialSession<(Lock<F>, ()), F::Applied>
) -> SharedSession<LinearToShared<F>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  unsafe_create_shared_session(
    move |receiver1: Receiver<(
      SenderOnce<()>,
      SenderOnce<LinearToShared<F>>,
    )>| async move {
      let (lock_producer_end, lock_consumer_end) =
        <Lock<F>>::create_endpoints();

      let (producer_end, consumer_end) = F::Applied::create_endpoints();

      let m_sender1 = receiver1.recv().await;

      if let Some((sender5, sender6)) = m_sender1 {
        sender6
          .send(LinearToShared {
            linear: Box::new(consumer_end),
          })
          .unwrap();

        sender5.send(()).unwrap();

        lock_producer_end.send(Lock { unlock: receiver1 }).unwrap();

        debug!("[accept_shared_session] calling cont");

        unsafe_run_session(
          cont,
          (wrap_type_app(lock_consumer_end), ()),
          producer_end,
        )
        .await;

        debug!("[accept_shared_session] returned from cont");
      } else {
        // shared session is terminated with all references to it
        // being dropped
      }
    },
  )
}

pub fn detach_shared_session<F, C>(
  cont: SharedSession<LinearToShared<F>>
) -> PartialSession<(Lock<F>, C), SharedToLinear<LinearToShared<F>>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
  C: EmptyContext,
{
  unsafe_create_session::<(Lock<F>, C), SharedToLinear<LinearToShared<F>>, _, _>(
    move |(lock_consumer_end, _), receiver| async move {
      debug!("[detach_shared_session] receiving sender2");

      let lock_receiver = lock_consumer_end.get_applied();

      let Lock { unlock: receiver2 } = lock_receiver.recv().await.unwrap();

      receiver.recv().await.unwrap();

      debug!("[detach_shared_session] received sender2");

      unsafe_run_shared_session(cont, receiver2).await;

      debug!("[detach_shared_session] ran cont");
    },
  )
}

pub fn async_acquire_shared_session<F>(
  shared: SharedChannel<LinearToShared<F>>,
  cont1: impl FnOnce(Z) -> PartialSession<(F::Applied, ()), End> + Send + 'static,
) -> task::JoinHandle<()>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  debug!("[async_acquire_shared_session] acquiring shared session");

  let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

  task::spawn(async move {
    let (provider_end_1, consumer_end_1) = End::create_endpoints();

    let LinearToShared { linear } = receiver4.recv().await.unwrap();

    let consumer_end_2 = linear.get_applied();

    let cont2 = cont1(Z);

    let ctx = (wrap_type_app(consumer_end_2), ());

    unsafe_run_session(cont2, ctx, provider_end_1).await;

    consumer_end_1.recv().await.unwrap();

    receiver3.recv().await.unwrap();
  })
}

pub fn async_acquire_shared_session_with_result<T, F>(
  shared: SharedChannel<LinearToShared<F>>,
  cont1: impl FnOnce(Z) -> PartialSession<(F::Applied, ()), SendValue<T, End>>
    + Send
    + 'static,
) -> task::JoinHandle<T>
where
  F: Protocol,
  T: Send + 'static,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  debug!("[async_acquire_shared_session_with_result] acquiring shared session");

  let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

  task::spawn(async move {
    let (provider_end_1, consumer_end_1) =
      <SendValue<T, End>>::create_endpoints();

    let LinearToShared { linear } = receiver4.recv().await.unwrap();

    let consumer_end_2 = linear.get_applied();

    let cont2 = cont1(Z);

    let ctx = (wrap_type_app(consumer_end_2), ());

    unsafe_run_session(cont2, ctx, provider_end_1).await;

    receiver3.recv().await.unwrap();

    let (Value(val), end_receiver) = consumer_end_1.recv().await.unwrap();

    end_receiver.recv().await.unwrap();

    val
  })
}

pub fn acquire_shared_session<N, C1, C2, A1, A2, B>(
  shared: SharedChannel<LinearToShared<A1>>,
  cont1: impl FnOnce(N) -> PartialSession<C2, B> + Send + 'static,
) -> PartialSession<C1, B>
where
  N: Nat,
  C1: Context<Length = N>,
  C2: Context,
  A1: Protocol,
  A2: Protocol,
  B: Protocol,
  A1: SharedRecApp<SharedToLinear<LinearToShared<A1>>, Applied = A2>,
  C1: AppendContext<(A2, ()), Appended = C2>,
{
  unsafe_create_session(move |ctx1, provider_end_1| async move {
    let cont2 = cont1(N::nat());

    let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

    debug!("[acquire_shared_session] acquiring shared endpoint");

    receiver3.recv().await.unwrap();

    debug!("[acquire_shared_session] acquired shared endpoint");

    let LinearToShared { linear } = receiver4.recv().await.unwrap();

    let consumer_end_2 = linear.get_applied();

    let ctx2 = C1::append_context(ctx1, (wrap_type_app(consumer_end_2), ()));

    unsafe_run_session(cont2, ctx2, provider_end_1).await;
  })
}

pub fn release_shared_session<N, C1, C2, A, B>(
  _n: N,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, B>
where
  A: Protocol,
  B: Protocol,
  C1: Context,
  C2: Context,
  A: SharedRecApp<SharedToLinear<LinearToShared<A>>>,
  N: ContextLens<C1, SharedToLinear<LinearToShared<A>>, Empty, Target = C2>,
{
  unsafe_create_session(move |ctx1, provider_end_b| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let lock_sender = endpoint.get_applied();

    let ctx3 = N::insert_target((), ctx2);

    lock_sender.send(()).unwrap();

    unsafe_run_session(cont, ctx3, provider_end_b).await;
  })
}

pub fn shared_forward<A1, A2, C>(
  channel: SharedChannel<LinearToShared<A1>>
) -> PartialSession<(Lock<A1>, C), SharedToLinear<LinearToShared<A1>>>
where
  A1: Protocol,
  A2: Protocol,
  A1: SharedRecApp<SharedToLinear<LinearToShared<A1>>, Applied = A2>,
  C: EmptyContext,
{
  unsafe_create_session::<(Lock<A1>, C), SharedToLinear<LinearToShared<A1>>, _, _>(
    move |(lock_consumer_end, _), receiver1| async move {
      let lock_receiver = lock_consumer_end.get_applied();

      let Lock { unlock } = lock_receiver.recv().await.unwrap();

      receiver1.recv().await.unwrap();

      unsafe_forward_shared_channel(channel, unlock).await;
    },
  )
}

impl<F> SharedChannel<LinearToShared<F>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  pub fn acquire<C, A>(
    &self,
    cont: impl FnOnce(C::Length) -> PartialSession<C::Appended, A> + Send + 'static,
  ) -> PartialSession<C, A>
  where
    C: Context,
    A: Protocol,
    C: AppendContext<(F::Applied, ())>,
  {
    acquire_shared_session(self.clone(), cont)
  }
}
