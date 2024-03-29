use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use tokio::task;

use crate::internal::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    Context,
    PartialSession,
    Protocol,
    ProviderEndpointF,
  },
  functional::{
    App,
    AppSum,
    NaturalTransformation,
    RowCon,
    SumFunctor,
    ToRow,
    TyCon,
    TypeApp,
  },
  protocol::ExternalChoice,
  session::choice::run_cont::RunCont,
};

pub fn offer_choice<C, Row1, Row2>(
  cont1: impl for<'r> FnOnce(
      AppSum<'r, Row2, InjectSessionF<'r, Row1, C>>,
    ) -> ContSum<'r, Row1, C>
    + Send
    + 'static
) -> PartialSession<C, ExternalChoice<Row1>>
where
  C: Context,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  Row2: SumFunctor,
{
  unsafe_create_session::<C, ExternalChoice<Row1>, _, _>(
    move |ctx, choice_receiver| async move {
      let provider_end_sum = choice_receiver.recv().await.unwrap();

      let cont_sum_1 =
        provider_end_sum_to_cont_sum::<C, Row1, Row2>(ctx, provider_end_sum);

      let res = cont1(cont_sum_1);

      task::spawn(async move {
        res.future.await;
      });
    },
  )
}

pub struct InjectSession<'r, Row, C: Context, A>
{
  ctx: C::Endpoints,
  provider_end: App<'r, ProviderEndpointF, A>,
  phantom: PhantomData<(Box<dyn Invariant<'r>>, Row)>,
}

pub struct InjectSessionF<'r, Row, C>(PhantomData<&'r (Row, C)>);

pub struct ContSum<'r, Row, C>
{
  future: Pin<Box<dyn Future<Output = ()> + Send + 'r>>,
  phantom: PhantomData<(Box<dyn Invariant<'r>>, C, Row)>,
}

impl<'r, Row: 'r, C: Context, A: Protocol> RunCont<C, A>
  for InjectSession<'r, Row, C, A>
where
  Row: Send,
{
  type Ret = ContSum<'r, Row, C>;

  fn run_cont(
    self,
    session: PartialSession<C, A>,
  ) -> Self::Ret
  {
    ContSum {
      future: Box::pin(async move {
        unsafe_run_session(session, self.ctx, self.provider_end.get_applied())
          .await;
      }),
      phantom: PhantomData,
    }
  }
}

pub trait Invariant<'r>: Send {}

impl<'r, Row, C: 'static> TyCon for InjectSessionF<'r, Row, C> {}

impl<'r, 'a, Row, C: Context, A: 'r> TypeApp<'a, A>
  for InjectSessionF<'r, Row, C>
where
  Row: Send,
{
  type Applied = InjectSession<'r, Row, C, A>;
}

fn provider_end_sum_to_cont_sum<C, Row1: 'static, Row2: 'static>(
  ctx: C::Endpoints,
  provider_end_sum: AppSum<'static, Row2, ProviderEndpointF>,
) -> AppSum<Row2, InjectSessionF<'static, Row1, C>>
where
  Row1: Send,
  Row2: SumFunctor,
  C: Context,
{
  struct ProviderEndToCont<C: Context>
  {
    ctx: C::Endpoints,
  }

  impl<'r, Row: Send, C: Context>
    NaturalTransformation<'r, ProviderEndpointF, InjectSessionF<'r, Row, C>>
    for ProviderEndToCont<C>
  {
    fn lift<A: 'r>(
      self,
      provider_end: App<'r, ProviderEndpointF, A>,
    ) -> App<'r, InjectSessionF<'r, Row, C>, A>
    {
      App::new(InjectSession {
        ctx: self.ctx,
        provider_end,
        phantom: PhantomData,
      })
    }
  }

  Row2::lift_sum(ProviderEndToCont { ctx }, provider_end_sum)
}
