use tokio::task;

use crate::internal::{
  base::*,
  functional::wrap_type_app,
};

pub fn fix_session<R, F, A, C>(
  cont: PartialSession<C, A>
) -> PartialSession<C, RecX<R, F>>
where
  C: Context,
  R: Context,
  F: Protocol,
  A: Protocol,
  F: RecApp<(RecX<R, F>, R), Applied = A>,
{
  unsafe_create_session::<C, RecX<R, F>, _, _>(move |ctx, sender1| async move {
    let (provider_end_a, client_end_a) = A::create_endpoints();

    let rec_end = RecEndpoint {
      applied: Box::new(client_end_a),
    };
    sender1.send(rec_end).unwrap();

    task::spawn(async move {
      unsafe_run_session(cont, ctx, provider_end_a).await;
    });
  })
}

pub fn unfix_session<N, C1, C2, A, B, R, F>(
  _n: N,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, B>
where
  B: Protocol,
  C1: Context,
  C2: Context,
  F: Protocol,
  R: Context,
  F: RecApp<(RecX<R, F>, R), Applied = A>,
  A: Protocol,
  N: ContextLens<C1, RecX<R, F>, A, Target = C2>,
{
  unsafe_create_session::<C1, B, _, _>(move |ctx1, provider_end| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let receiver1 = endpoint.get_applied();

    let rec_end = receiver1.recv().await.unwrap();

    let client_end = *rec_end.applied.get_applied();

    let ctx3 = N::insert_target(wrap_type_app(client_end), ctx2);

    task::spawn(async move {
      unsafe_run_session(cont, ctx3, provider_end).await;
    });
  })
}
