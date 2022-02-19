use tokio::task;

use crate::internal::{
  base::*,
  functional::*,
  protocol::{
    End,
    LinearToShared,
    SendValue,
    SharedToLinear,
  },
};

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
    let (provider_end_1, client_end_1) = End::create_endpoints();

    let LinearToShared { linear } = receiver4.recv().await.unwrap();

    let client_end_2 = linear.get_applied();

    let cont2 = cont1(Z);

    let ctx = (App::new(client_end_2), ());

    unsafe_run_session(cont2, ctx, provider_end_1).await;

    client_end_1.recv().await.unwrap();

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
    let (provider_end_1, client_end_1) =
      <SendValue<T, End>>::create_endpoints();

    let LinearToShared { linear } = receiver4.recv().await.unwrap();

    let client_end_2 = linear.get_applied();

    let cont2 = cont1(Z);

    let ctx = (App::new(client_end_2), ());

    unsafe_run_session(cont2, ctx, provider_end_1).await;

    receiver3.recv().await.unwrap();

    let (Value(val), end_receiver) = client_end_1.recv().await.unwrap();

    end_receiver.recv().await.unwrap();

    val
  })
}

pub fn acquire_shared_session<C, F, A>(
  shared: SharedChannel<LinearToShared<F>>,
  cont1: impl FnOnce(C::Length) -> PartialSession<C::Appended, A> + Send + 'static,
) -> PartialSession<C, A>
where
  C: Context,
  F: Protocol,
  A: Protocol,
  F::Applied: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  C: AppendContext<(F::Applied, ())>,
{
  unsafe_create_session(move |ctx1, provider_end_1| async move {
    let cont2 = cont1(C::Length::nat());

    let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

    debug!("[acquire_shared_session] acquiring shared endpoint");

    receiver3.recv().await.unwrap();

    debug!("[acquire_shared_session] acquired shared endpoint");

    let LinearToShared { linear } = receiver4.recv().await.unwrap();

    let client_end_2 = linear.get_applied();

    let ctx2 = C::append_context(ctx1, (App::new(client_end_2), ()));

    unsafe_run_session(cont2, ctx2, provider_end_1).await;
  })
}
