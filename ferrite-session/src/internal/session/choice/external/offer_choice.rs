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
    once_channel,
    unsafe_create_session,
    Context,
    Fixed,
    PartialSession,
    Protocol,
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
  },
  protocol::ExternalChoice,
};

pub fn offer_choice<C, Row, Choice, SessionSum, InjectSessionSum>(
  cont1: impl FnOnce(InjectSessionSum) -> SessionSum + Send + 'static
) -> PartialSession<C, Choice>
where
  Choice: Protocol,
  Choice: Fixed<Unfixed = ExternalChoice<Row>>,
  C: Context,
  Row: RowCon,
  Row: ElimSum,
  Row: SplitRow,
  Row: SumFunctor,
  Row: SumFunctorInject,
  Row: SumApp<SessionF<C>, Applied = SessionSum>,
  Row: FlattenSumApp<InjectSessionF<Row, C>, FlattenApplied = InjectSessionSum>,
  SessionSum: Send + 'static,
  InjectSessionSum: Send + 'static,
{
  unsafe_create_session(move |ctx, sender1| async move {
    let (sender2, receiver2) = once_channel();

    let payload1 = ExternalChoice::<Row> { sender: sender2 };
    let payload2 = Choice::fix(payload1);

    sender1.send(payload2).unwrap();

    let (Value(choice), sender3) = receiver2.recv().await.unwrap();

    let cont3 = selector_to_inject_session(choice);

    let cont4 = Row::flatten_sum(cont3);

    let cont5 = wrap_sum_app(cont1(cont4));

    run_choice_cont(ctx, sender3, cont5).await;
  })
}
