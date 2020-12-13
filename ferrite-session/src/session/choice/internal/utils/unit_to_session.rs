use std::marker::PhantomData;

use crate::base::*;
use crate::protocol::*;
use crate::functional::*;

use super::super::inject_session::*;
use super::super::internal_session::*;

pub fn lift_unit_to_session
  < N, C, D, B, Row >
  ( row: AppliedSum < Row, () > )
  ->
    AppliedSum <
      Row,
      InjectSessionF < N, C, B, Row, D >
    >
where
  B : Protocol,
  C : Context,
  D : Context,
  N : Send + 'static,
  Row : RowCon,
  Row : SumFunctorInject,
{
  lift_sum_inject
    ( LiftUnitToSession(PhantomData),
      row
    )
}

struct LiftUnitToSession < N, C, D, A, Row >
  ( PhantomData <( N, C, D, A, Row )> );

impl
  < N, C, D, B, Row >
  InjectLift <
    AppliedSum <
      Row,
      InternalSessionF < N, C, B, Row, D >
    >
  >
  for LiftUnitToSession < N, C, D, B, Row >
where
  B : Protocol,
  C : Context,
  D : Context,
  Row : RowCon,
  N : Send + 'static,
{
  type SourceF = ();

  type TargetF =
    InternalSessionF < N, C, B, Row, D >;

  type InjectF =
    InjectSessionF < N, C, B, Row, D >;

  fn lift_field < A >
    ( self,
      inject1:
        impl Fn
          ( Applied < Self::TargetF, A > )
          ->
            AppliedSum <
              Row,
              InternalSessionF < N, C, B, Row, D >
            >
        + Send + 'static,
      _row:
        Applied < Self::SourceF, A >
    ) ->
      Applied < Self::InjectF, A >
  where
    A: Send + 'static,
  {
    let inject2 = SessionInjectorImpl {
      injector: Box::new( inject1 )
    };

    let inject3 = InjectSession {
      injector : Box::new ( inject2 )
    };

    cloak_applied( inject3 )
  }
}

struct SessionInjectorImpl
  < N, C, A, B, Row, Del >
{
  injector: Box < dyn FnOnce
    ( Applied <
        InternalSessionF < N, C, B, Row, Del >,
        A
      >
    ) ->
      AppliedSum <
        Row,
        InternalSessionF < N, C, B, Row, Del >
      >
    + Send + 'static
  >
}

impl < N, C, A, B, Row, Del >
  SessionInjector
  < N, C, A, B, Row, Del >
  for SessionInjectorImpl
  < N, C, A, B, Row, Del >
{
  fn inject_session
    ( self: Box < Self >,
      session1:
        PartialSession <
          N :: Target,
          B
        >
    ) ->
      AppliedSum <
        Row,
        InternalSessionF < N, C, B, Row, Del >
      >
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del: Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del
      >
  {
    let session2 = cloak_internal_session ::
      < N, C, A, B, Row, Del >
      ( session1 );

    (self.injector)( cloak_applied ( session2 ) )
  }
}
