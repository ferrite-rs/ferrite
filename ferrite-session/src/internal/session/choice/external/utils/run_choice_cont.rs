use std::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use super::super::cloak_session::*;
use crate::internal::{
  base::*,
  functional::*,
};

pub async fn run_choice_cont<Row, C>(
  ctx: C::Endpoints,
  sender: SenderOnce<AppSum<Row, ReceiverF>>,
  cont1: AppSum<Row, SessionF<C>>,
) where
  C: Context,
  Row: ElimSum,
  Row: SplitRow,
  Row: SumFunctorInject,
{
  let res = lift_sum_inject(RunSession { ctx }, cont1);

  let (receiver_sum, cont6) = Row::split_row(res);

  sender.send(receiver_sum).unwrap();

  Row::elim_sum(ElimConst {}, cont6).await;
}

struct RunSession<C>
where
  C: Context,
{
  ctx: C::Endpoints,
}

struct SessionRunner<C, A>
where
  C: Context,
{
  ctx: C::Endpoints,
  phantom: PhantomData<A>,
}

impl<C, A>
  NeedPartialSession<
    C,
    A,
    (
      ReceiverOnce<A>,
      Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ),
  > for SessionRunner<C, A>
where
  C: Context,
{
  fn on_partial_session(
    self: Box<Self>,
    cont: PartialSession<C, A>,
  ) -> (
    ReceiverOnce<A>,
    Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
  )
  where
    C: Context,
    A: Protocol,
  {
    let (sender, receiver) = once_channel();

    let future = Box::pin(async move {
      unsafe_run_session(cont, self.ctx, sender).await;
    });

    (receiver, future)
  }
}

impl<Root, C> InjectLift<Root> for RunSession<C>
where
  C: Context,
{
  type InjectF =
    Merge<ReceiverF, Const<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>;
  type SourceF = SessionF<C>;
  type TargetF = ();

  fn lift_field<A>(
    self,
    _inject: impl Fn(App<Self::TargetF, A>) -> Root + Send + 'static,
    cont1: App<Self::SourceF, A>,
  ) -> App<Self::InjectF, A>
  where
    A: Send + 'static,
  {
    let cont2: CloakedSession<C, A> = get_applied(cont1);

    let runner: SessionRunner<C, A> = SessionRunner {
      ctx: self.ctx,
      phantom: PhantomData,
    };

    let (receiver, future) = *with_session(cont2, Box::new(runner));

    wrap_type_app((wrap_type_app(receiver), wrap_type_app(future)))
  }
}
