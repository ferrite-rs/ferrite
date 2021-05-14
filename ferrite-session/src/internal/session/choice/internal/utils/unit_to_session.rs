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
  row: AppSum<Row2, ()>
) -> AppSum<Row2, InjectSessionF<N, C, B, Row1, D>>
where
  B: Protocol,
  C: Context,
  D: Context,
  N: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  Row2: SumFunctorInject,
{
  lift_sum_inject(LiftUnitToSession(PhantomData), row)
}

struct LiftUnitToSession<N, C, D, A, Row>(PhantomData<(N, C, D, A, Row)>);

impl<N, C, D, B, Row1, Row2>
  InjectLift<AppSum<Row2, InternalSessionF<N, C, B, Row1, D>>>
  for LiftUnitToSession<N, C, D, B, Row1>
where
  B: Protocol,
  C: Context,
  D: Context,
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
        App<Self::TargetF, A>,
      ) -> AppSum<Row2, InternalSessionF<N, C, B, Row1, D>>
      + Send
      + 'static,
    _row: App<Self::SourceF, A>,
  ) -> App<Self::InjectF, A>
  where
    A: Send + 'static,
  {
    let inject2 = SessionInjectorImpl {
      injector: Box::new(inject1),
    };

    let inject3 = InjectSession {
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
        App<InternalSessionF<N, C, B, Row, Del>, A>,
      ) -> AppSum<Row::Row, InternalSessionF<N, C, B, Row, Del>>
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
  ) -> AppSum<Row::Row, InternalSessionF<N, C, B, Row, Del>>
  where
    A: Protocol,
    B: Protocol,
    C: Context,
    Del: Context,
    Row: ToRow,
    N: ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
  {
    let session2 =
      cloak_internal_session::<N, C, A, B, Row, Row::Row, Del>(session1);

    (self.injector)(wrap_type_app(session2))
  }
}
