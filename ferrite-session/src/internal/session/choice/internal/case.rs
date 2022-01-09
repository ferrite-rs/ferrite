use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use crate::internal::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    ConsumerEndpointF,
    Context,
    ContextLens,
    Empty,
    PartialSession,
    Protocol,
    ProviderEndpointF,
  },
  functional::{
    wrap_type_app,
    App,
    AppSum,
    NaturalTransformation,
    SumFunctor,
    ToRow,
    TyCon,
    TypeApp,
  },
  protocol::InternalChoice,
  session::choice::run_cont::RunCont,
};

pub fn case<N, C1, C2, B, Row1, Row2>(
  _: N,
  cont1: impl for<'r> FnOnce(
      AppSum<'r, Row2, ContF<'r, N, C2, B>>,
    ) -> ChoiceRet<'r, N, C2, B>
    + Send
    + 'static,
) -> PartialSession<C1, B>
where
  B: Protocol,
  C1: Context,
  C2: Context,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Row1: ToRow<Row = Row2>,
  N: ContextLens<C1, InternalChoice<Row1>, Empty, Target = C2>,
  Row2: SumFunctor,
  // Row2: RowCon,
  // Row2: ElimSum,
  // Row2: SplitRow,
  // Row2: IntersectSum,
  // Row2: SumFunctorInject,
  // Row2:
  //   SumApp<'static, InternalSessionF<N, C, B, Row1, D>, Applied = SessionSum>,
  // Row2: FlattenSumApp<
  //   'static,
  //   InjectSessionF<N, C, B, Row1, D>,
  //   FlattenApplied = InjectSessionSum,
  // >,
  // N: ContextLens<C, InternalChoice<Row1>, Empty, Deleted = D>,
  // SessionSum: Send + 'static,
  // InjectSessionSum: Send + 'static,
{
  unsafe_create_session::<C1, B, _, _>(move |ctx1, provider_end| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let ctx3 = N::insert_target((), ctx2);

    let consumer_end_sum_receiver = endpoint.get_applied();

    let consumer_end_sum = consumer_end_sum_receiver.recv().await.unwrap();

    let cont_sum = consumer_end_sum_to_cont_sum::<N, C2, B, Row2>(
      ctx3,
      wrap_type_app(provider_end),
      consumer_end_sum,
    );

    let res = cont1(cont_sum);
    res.future.await;

    // let (receiver_sum2, selector_sum) = receiver_to_selector(consumer_end_sum);

    // let cont3 = lift_unit_to_session(selector_sum);

    // let cont4 = wrap_sum_app(cont1(cont3));

    // let cont5 = Row2::intersect_sum(receiver_sum2, cont4);

    // match cont5 {
    //   Some(cont6) => {
    //     run_case_cont(ctx2, wrap_type_app(provider_end_b), cont6).await;
    //   }
    //   None => {
    //     panic!("impossible happened: received mismatch choice continuation");
    //   }
    // }
  })
}

pub struct ContF<'r, N, C, B>(PhantomData<&'r (N, C, B)>);

pub trait Invariant<'r>: Send {}

pub struct ChoiceCont<'r, N, C, B, A>
where
  C: Context,
{
  ctx: C::Endpoints,
  provider_end: App<'static, ProviderEndpointF, B>,
  consumer_end: App<'r, ConsumerEndpointF, A>,
  phantom: PhantomData<(Box<dyn Invariant<'r>>, N, C)>,
}

impl<'r, N, C, B> TyCon for ContF<'r, N, C, B> {}

impl<'r, 'a, N, C, B, A: 'r> TypeApp<'a, A> for ContF<'r, N, C, B>
where
  C: Context,
  N: Send,
{
  type Applied = ChoiceCont<'r, N, C, B, A>;
}

pub struct ChoiceRet<'r, N, C, B>
{
  future: Pin<Box<dyn Future<Output = ()> + Send + 'r>>,
  phantom: PhantomData<(Box<dyn Invariant<'r>>, N, C, B)>,
}

impl<'r, N, C1, C2, B, A> RunCont<C2, B> for ChoiceCont<'r, N, C1, B, A>
where
  A: Protocol,
  B: Protocol,
  C1: Context,
  C2: Context,
  N: ContextLens<C1, Empty, A, Target = C2>,
{
  type Ret = ChoiceRet<'r, N, C1, B>;

  fn run_cont(
    self,
    session: PartialSession<C2, B>,
  ) -> Self::Ret
  {
    ChoiceRet {
      future: Box::pin(async move {
        let ((), ctx1) = N::extract_source(self.ctx);
        let consumer_end = self.consumer_end.get_applied();

        let ctx2 = N::insert_target(wrap_type_app(consumer_end), ctx1);
        unsafe_run_session(session, ctx2, self.provider_end.get_applied())
          .await;
      }),
      phantom: PhantomData,
    }
  }
}

fn consumer_end_sum_to_cont_sum<N, C, B, Row>(
  ctx: C::Endpoints,
  provider_end: App<'static, ProviderEndpointF, B>,
  consumer_end_sum: AppSum<'static, Row, ConsumerEndpointF>,
) -> AppSum<'static, Row, ContF<'static, N, C, B>>
where
  C: Context,
  N: Send,
  Row: SumFunctor + Send + 'static,
{
  struct Trans<C: Context, B>
  {
    ctx: C::Endpoints,
    provider_end: App<'static, ProviderEndpointF, B>,
  }

  impl<'r, N, C, B>
    NaturalTransformation<'r, ConsumerEndpointF, ContF<'r, N, C, B>>
    for Trans<C, B>
  where
    C: Context,
    N: Send,
  {
    fn lift<A: 'r>(
      self,
      consumer_end: App<'r, ConsumerEndpointF, A>,
    ) -> App<'r, ContF<'r, N, C, B>, A>
    {
      wrap_type_app(ChoiceCont {
        ctx: self.ctx,
        provider_end: self.provider_end,
        consumer_end,
        phantom: PhantomData,
      })
    }
  }

  Row::lift_sum(Trans { ctx, provider_end }, consumer_end_sum)
}
