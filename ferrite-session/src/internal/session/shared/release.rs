use crate::internal::{
  base::*,
  protocol::{
    LinearToShared,
    SharedToLinear,
  },
};

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
