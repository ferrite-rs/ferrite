use super::{
  inject_session::*,
  internal_session::*,
  utils::*,
  super::run_cont::WrapRunCont,
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
    ToRow,
  },
  protocol::InternalChoice,
};

pub fn case<N, C, D, B, Row1, Row2, SessionSum, InjectSessionSum>(
  _: N,
  cont1: impl for <'a> FnOnce(WrapRunCont<'a, InjectSessionSum>) -> SessionSum + Send + 'static,
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
  unsafe_create_session(move |ctx1, sender| async move {
    let (sum_chan, ctx2) = N::extract_source(ctx1);

    let InternalChoice {
      field: receiver_sum1,
    } = sum_chan.recv().await.unwrap();

    let (receiver_sum2, selector_sum) = receiver_to_selector(receiver_sum1);

    let cont3 = lift_unit_to_session(selector_sum);

    let cont4 = Row2::flatten_sum(cont3);

    let cont5 = WrapRunCont::new(cont4);

    let cont6 = wrap_sum_app(cont1(cont5));

    let cont7 = Row2::intersect_sum(receiver_sum2, cont6);

    match cont7 {
      Some(cont8) => {
        run_case_cont(ctx2, sender, cont8).await;
      }
      None => {
        panic!("impossible happened: received mismatch choice continuation");
      }
    }
  })
}
