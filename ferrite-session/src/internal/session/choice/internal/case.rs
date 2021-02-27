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
    ElimSum,
    FlattenSumApp,
    IntersectSum,
    RowCon,
    SplitRow,
    SumApp,
    SumFunctor,
    SumFunctorInject,
  },
  protocol::InternalChoice,
};

pub fn case<N, C, D, B, Row, SessionSum, InjectSessionSum>(
  _ : N,
  cont1 : impl FnOnce(InjectSessionSum) -> SessionSum + Send + 'static,
) -> PartialSession<C, B>
where
  B : Protocol,
  C : Context,
  D : Context,
  Row : RowCon,
  Row : ElimSum,
  Row : SplitRow,
  Row : SumFunctor,
  Row : IntersectSum,
  Row : SumFunctorInject,
  Row : SumApp<InternalSessionF<N, C, B, Row, D>, Applied = SessionSum>,
  Row : FlattenSumApp<
    InjectSessionF<N, C, B, Row, D>,
    FlattenApplied = InjectSessionSum,
  >,
  N : ContextLens<C, InternalChoice<Row>, Empty, Deleted = D>,
  InjectSessionSum : Send + 'static,
{
  unsafe_create_session(move |ctx1, sender| async move {
    let (sum_chan, ctx2) = N::extract_source(ctx1);

    let InternalChoice {
      field: receiver_sum1,
    } = sum_chan.recv().await.unwrap();

    let (receiver_sum2, selector_sum) = receiver_to_selector(receiver_sum1);

    let cont3 = lift_unit_to_session(selector_sum);

    let cont3a = Row::flatten_sum(cont3);

    let cont4 = wrap_sum_app(cont1(cont3a));

    let cont5 = Row::intersect_sum(receiver_sum2, cont4);

    match cont5 {
      Some(cont6) => {
        run_case_cont(ctx2, sender, cont6).await;
      }
      None => {
        panic!("impossible happened: received mismatch choice continuation");
      }
    }
  })
}
