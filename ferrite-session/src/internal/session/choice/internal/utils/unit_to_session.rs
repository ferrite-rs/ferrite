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

pub fn lift_unit_to_session<N, C, D, B, Row>(
  row: AppSum<Row, ()>
) -> AppSum<Row, InjectSessionF<N, C, B, Row, D>>
where
  B: Protocol,
  C: Context,
  D: Context,
  N: Send + 'static,
  Row: RowCon,
  Row: SumFunctorInject,
{
  lift_sum_inject(LiftUnitToSession(PhantomData), row)
}

struct LiftUnitToSession<N, C, D, A, Row>(PhantomData<(N, C, D, A, Row)>);

impl<N, C, D, B, Row> InjectLift<AppSum<Row, InternalSessionF<N, C, B, Row, D>>>
  for LiftUnitToSession<N, C, D, B, Row>
where
  B: Protocol,
  C: Context,
  D: Context,
  Row: RowCon,
  N: Send + 'static,
{
  type InjectF = InjectSessionF<N, C, B, Row, D>;
  type SourceF = ();
  type TargetF = InternalSessionF<N, C, B, Row, D>;

  fn lift_field<A>(
    self,
    inject1: impl Fn(
        App<Self::TargetF, A>,
      ) -> AppSum<Row, InternalSessionF<N, C, B, Row, D>>
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
{
  injector: Box<
    dyn FnOnce(
        App<InternalSessionF<N, C, B, Row, Del>, A>,
      ) -> AppSum<Row, InternalSessionF<N, C, B, Row, Del>>
      + Send
      + 'static,
  >,
}

impl<N, C, A, B, Row, Del> SessionInjector<N, C, A, B, Row, Del>
  for SessionInjectorImpl<N, C, A, B, Row, Del>
{
  fn inject_session(
    self: Box<Self>,
    session1: PartialSession<N::Target, B>,
  ) -> AppSum<Row, InternalSessionF<N, C, B, Row, Del>>
  where
    A: Protocol,
    B: Protocol,
    C: Context,
    Del: Context,
    Row: RowCon,
    N: ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
  {
    let session2 = cloak_internal_session::<N, C, A, B, Row, Del>(session1);

    (self.injector)(wrap_type_app(session2))
  }
}
