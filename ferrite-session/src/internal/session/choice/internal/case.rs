use super::{
  inject_session::*,
  internal_session::*,
  utils::*,
};
use crate::internal::{
  base::{
    unsafe_create_session,
    Context,
    ContextLens,
    Empty,
    PartialSession,
    Protocol,
  },
  functional::{
    wrap_sum_app,
    wrap_type_app,
    ElimSum,
    FlattenSumApp,
    IntersectSum,
    RowCon,
    SplitRow,
    SumApp,
    SumFunctor,
    SumFunctorInject,
    ToRow,
  },
  protocol::InternalChoice,
};

pub fn case<N, C, D, B, Row1, Row2, SessionSum, InjectSessionSum>(
  _: N,
  cont1: impl FnOnce(InjectSessionSum) -> SessionSum + Send + 'static,
) -> PartialSession<C, B>
where
  B: Protocol,
  C: Context,
  D: Context,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  Row2: ElimSum,
  Row2: SplitRow,
  Row2: SumFunctor,
  Row2: IntersectSum,
  Row2: SumFunctorInject,
  Row2: SumApp<InternalSessionF<N, C, B, Row1, D>, Applied = SessionSum>,
  Row2: FlattenSumApp<
    InjectSessionF<N, C, B, Row1, D>,
    FlattenApplied = InjectSessionSum,
  >,
  N: ContextLens<C, InternalChoice<Row1>, Empty, Deleted = D>,
  SessionSum: Send + 'static,
  InjectSessionSum: Send + 'static,
{
  unsafe_create_session::<C, B, _, _>(move |ctx1, provider_end_b| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let consumer_end_sum_receiver = endpoint.get_applied();

    let consumer_end_sum = consumer_end_sum_receiver.recv().await.unwrap();

    let (receiver_sum2, selector_sum) = receiver_to_selector(consumer_end_sum);

    let cont3 = lift_unit_to_session(selector_sum);

    let cont3a = Row2::flatten_sum(cont3);

    let cont4 = wrap_sum_app(cont1(cont3a));

    let cont5 = Row2::intersect_sum(receiver_sum2, cont4);

    match cont5 {
      Some(cont6) => {
        run_case_cont(ctx2, wrap_type_app(provider_end_b), cont6).await;
      }
      None => {
        panic!("impossible happened: received mismatch choice continuation");
      }
    }
  })
}
