use crate::internal::{
  base::*,
  functional::App,
  protocol::*,
};

pub fn wrap_session<C, T>(
  cont: PartialSession<C, T::Unwrap>
) -> PartialSession<C, Wrap<T>>
where
  C: Context,
  T: Wrapper,
  T: Send + 'static,
  T::Unwrap: Protocol,
{
  unsafe_create_session::<C, Wrap<T>, _, _>(move |ctx, sender1| async move {
    let (provider_end, client_end) = T::Unwrap::create_endpoints();

    sender1
      .send(Wrap {
        unwrap: Box::new(client_end),
      })
      .unwrap();

    unsafe_run_session(cont, ctx, provider_end).await;
  })
}

pub fn unwrap_session<N, C, T, A>(
  _: N,
  cont: PartialSession<N::Target, A>,
) -> PartialSession<C, A>
where
  C: Context,
  A: Protocol,
  T: Wrapper + Send + 'static,
  N: ContextLens<C, Wrap<T>, T::Unwrap>,
{
  unsafe_create_session(move |ctx1, provider_end_a| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let receiver = endpoint.get_applied();

    let wrapped = receiver.recv().await.unwrap();

    let unwrapped = wrapped.unwrap.unwrap();

    let ctx3 = N::insert_target(App::new(unwrapped), ctx2);

    unsafe_run_session(cont, ctx3, provider_end_a).await;
  })
}
