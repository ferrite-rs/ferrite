use std::marker::PhantomData;

use super::super::{
  inject_session::*,
  internal_session::*,
};
use crate::internal::{
  base::*,
  functional::*,
  protocol::*,
};

pub fn lift_unit_to_session<N, C, D, B, Row1, Row2>(
  row: AppSum<'static, Row2, ()>
) -> AppSum<'static, Row2, InjectSessionF<N, C, B, Row1, D>>
where
  B: Protocol,
  C: Context,
  D: Context,
  N: Send + 'static,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  Row2: SumFunctorInject,
{
  lift_sum_inject(LiftUnitToSession(PhantomData), row)
}

struct LiftUnitToSession<N, C, D, A, Row>(PhantomData<(N, C, D, A, Row)>);

impl<N, C, D, B, Row1, Row2>
  InjectLift<'static, AppSum<'static, Row2, InternalSessionF<N, C, B, Row1, D>>>
  for LiftUnitToSession<N, C, D, B, Row1>
where
  B: Protocol,
  C: Context,
  D: Context,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  N: Send + 'static,
{
  type InjectF = InjectSessionF<N, C, B, Row1, D>;
  type SourceF = ();
  type TargetF = InternalSessionF<N, C, B, Row1, D>;

  fn lift_field<A>(
    self,
    inject1: impl Fn(
        App<'static, Self::TargetF, A>,
      ) -> AppSum<'static, Row2, InternalSessionF<N, C, B, Row1, D>>
      + Send
      + 'static,
    _row: App<'static, Self::SourceF, A>,
  ) -> App<'static, Self::InjectF, A>
  where
    A: Send + 'static,
  {
    let inject2 = SessionInjectorImpl {
      injector: Box::new(inject1),
    };

    let inject3 = InjectInternal {
      injector: Box::new(inject2),
    };

    wrap_type_app(inject3)
  }
}

struct SessionInjectorImpl<N, C, A, B, Row, Del>
where
  Row: ToRow,
{
  injector: Box<
    dyn FnOnce(
        App<'static, InternalSessionF<N, C, B, Row, Del>, A>,
      )
        -> AppSum<'static, Row::Row, InternalSessionF<N, C, B, Row, Del>>
      + Send
      + 'static,
  >,
}

impl<N, C, A, B, Row, Del> SessionInjector<N, C, A, B, Row, Del>
  for SessionInjectorImpl<N, C, A, B, Row, Del>
where
  Row: ToRow,
{
  fn inject_session(
    self: Box<Self>,
    session1: PartialSession<N::Target, B>,
  ) -> AppSum<'static, Row::Row, InternalSessionF<N, C, B, Row, Del>>
  where
    A: Protocol,
    B: Protocol,
    C: Context,
    Del: Context,
    Row: ToRow,
    Row: Send + 'static,
    Row::Row: Send + 'static,
    Row::Row: RowCon,
    N: ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
  {
    let session2 =
      cloak_internal_session::<N, C, A, B, Row, Row::Row, Del>(session1);

    (self.injector)(wrap_type_app(session2))
  }
}
