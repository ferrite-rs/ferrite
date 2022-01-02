use super::{
  cloak_session::SessionF,
  inject_session::InjectSessionF,
  utils::{
    run_choice_cont,
    selector_to_inject_session,
  },
};
use crate::internal::{
  base::{
    unsafe_create_session,
    Context,
    PartialSession,
    Value,
  },
  functional::{
    wrap_sum_app,
    ElimSum,
    FlattenSumApp,
    RowCon,
    SplitRow,
    SumApp,
    SumFunctor,
    SumFunctorInject,
    ToRow,
  },
  protocol::ExternalChoice,
};

pub fn offer_choice<C, Row1, Row2, SessionSum, InjectSessionSum>(
  cont1: impl FnOnce(InjectSessionSum) -> SessionSum + Send + 'static
) -> PartialSession<C, ExternalChoice<Row1>>
where
  C: Context,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  Row2: ElimSum,
  Row2: SplitRow,
  Row2: SumFunctor,
  Row2: SumFunctorInject,
  Row2: SumApp<SessionF<C>, Applied = SessionSum>,
  Row2:
    FlattenSumApp<InjectSessionF<Row1, C>, FlattenApplied = InjectSessionSum>,
  SessionSum: Send + 'static,
  InjectSessionSum: Send + 'static,
{
  unsafe_create_session::<C, ExternalChoice<Row1>, _, _>(
    move |ctx, choice_receiver| async move {
      let (Value(choice), sum_sender) = choice_receiver.recv().await.unwrap();

      let cont3 = selector_to_inject_session(choice);

      let cont4 = Row2::flatten_sum(cont3);

      let cont5 = wrap_sum_app(cont1(cont4));

      run_choice_cont(ctx, sum_sender, cont5).await;
    },
  )
}
