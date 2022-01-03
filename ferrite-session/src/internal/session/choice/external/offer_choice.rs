use core::marker::PhantomData;

use tokio::task;

use crate::internal::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    Context,
    PartialSession,
    Protocol,
    ProviderEndpoint,
    ProviderEndpointF,
  },
  functional::{
    wrap_type_app,
    App,
    AppSum,
    ElimSum,
    FlattenSumApp,
    NaturalTransformation,
    RowCon,
    SplitRow,
    SumFunctor,
    SumFunctorInject,
    ToRow,
    TyCon,
    TypeApp,
  },
  protocol::ExternalChoice,
  session::choice::run_cont::RunCont,
};

pub fn offer_choice<C, Row1, Row2, InjectSessionSum>(
  cont1: impl FnOnce(InjectSessionSum) + Send + 'static
) -> PartialSession<C, ExternalChoice<Row1>>
where
  C: Context,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  Row2: ElimSum,
  Row2: SplitRow,
  Row2: SumFunctor,
  Row2: SumFunctorInject,
  // Row2: SumApp<SessionF<C>, Applied = SessionSum>,
  Row2: FlattenSumApp<'static, ContF<C>, FlattenApplied = InjectSessionSum>,
  InjectSessionSum: Send + 'static,
{
  unsafe_create_session::<C, ExternalChoice<Row1>, _, _>(
    move |ctx, choice_receiver| async move {
      let provider_end_sum = choice_receiver.recv().await.unwrap();

      let cont_sum_1 =
        provider_end_sum_to_cont_sum::<C, Row2>(ctx, provider_end_sum);

      let cont_sum_2 = Row2::flatten_sum(cont_sum_1);

      cont1(cont_sum_2)
    },
  )
}

impl<C: Context, A: Protocol> RunCont<C, A> for ChoiceCont<C, A>
{
  type Ret = ();

  fn run_cont(
    self,
    session: PartialSession<C, A>,
  )
  {
    task::spawn(async move {
      unsafe_run_session(session, self.ctx, self.provider_end.get_applied())
        .await;
    });
  }
}

pub struct ChoiceCont<C: Context, A>
{
  ctx: C::Endpoints,
  provider_end: ProviderEndpoint<A>,
}

pub struct ContF<C>(PhantomData<C>);

impl<C: 'static> TyCon for ContF<C> {}

impl<'a, C: Context, A: 'static> TypeApp<'a, A> for ContF<C>
{
  type Applied = ChoiceCont<C, A>;
}

fn provider_end_sum_to_cont_sum<C, Row: 'static>(
  ctx: C::Endpoints,
  provider_end_sum: AppSum<'static, Row, ProviderEndpointF>,
) -> AppSum<Row, ContF<C>>
where
  Row: SumFunctor,
  C: Context,
{
  struct ProviderEndToCont<C: Context>
  {
    ctx: C::Endpoints,
  }

  impl<C: Context> NaturalTransformation<'static, ProviderEndpointF, ContF<C>>
    for ProviderEndToCont<C>
  {
    fn lift<A: 'static>(
      self,
      provider_end: App<'static, ProviderEndpointF, A>,
    ) -> App<'static, ContF<C>, A>
    {
      wrap_type_app(ChoiceCont {
        ctx: self.ctx,
        provider_end,
      })
    }
  }

  Row::lift_sum(ProviderEndToCont { ctx }, provider_end_sum)
}
