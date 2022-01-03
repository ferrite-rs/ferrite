use crate::internal::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    Context,
    ContextLens,
    PartialSession,
    Protocol,
  },
  functional::{
    wrap_type_app,
    Prism,
    RowCon,
    ToRow,
  },
  protocol::ExternalChoice,
};

pub fn choose<N, M, C1, C2, A, B, Row1, Row2>(
  _: N,
  _: M,
  cont: PartialSession<C2, A>,
) -> PartialSession<C1, A>
where
  C1: Context,
  C2: Context,
  A: Protocol,
  B: Protocol,
  Row2: RowCon,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  N: ContextLens<C1, ExternalChoice<Row1>, B, Target = C2>,
  M: Prism<Row2, Elem = B>,
{
  unsafe_create_session(move |ctx1, provider_end| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let choice_sender = endpoint.get_applied();

    let (provider_end_b, consumer_end_b) = B::create_endpoints();

    let provider_end_sum = M::inject_elem(wrap_type_app(provider_end_b));

    choice_sender.send(provider_end_sum).unwrap();

    let ctx3 = N::insert_target(wrap_type_app(consumer_end_b), ctx2);

    unsafe_run_session(cont, ctx3, provider_end).await;
  })
}
