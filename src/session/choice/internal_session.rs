use std::marker::PhantomData;
use std::any::Any;

use crate::base::*;
use crate::protocol::*;
use crate::functional::*;

pub struct InternalSessionF
  < N, C, B, Row, Del >
{
  phantom: PhantomData <( N, C, B, Row, Del )>
}

pub struct InternalSession
  < N, C, A, B, Row, Del >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A,
      Deleted = Del
    >
{
  pub session:
    PartialSession <
      N :: Target,
      B
    >
}

trait HasInternalSession
  < N, C, A, B, Row, Del >
  : Send
{
  fn get_internal_session
    ( self: Box < Self > )
    -> InternalSession < N, C, A, B, Row, Del >
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del : Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del
      >
  ;
}

pub trait NeedInternalSession
  < N, C, A, B, Row, Del, K >
{
  fn on_internal_session
    ( self: Box < Self >,
      session: InternalSession < N, C, A, B, Row, Del >
    ) -> K
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del : Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del,
      >
  ;
}

trait InternalSessionWitness
  < N, C, A, B, Row, Del, K >
  : HasInternalSession < N, C, A, B, Row, Del >
{
  fn with_internal_session
    ( self: Box < Self >,
      cont: Box < dyn
        NeedInternalSession
          < N, C, A, B, Row, Del, K >
        >,
    ) -> K
  ;
}

impl < N, C, A, B, Row, Del >
  HasInternalSession
  < N, C, A, B, Row, Del >
  for InternalSession < N, C, A, B, Row, Del >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A,
      Deleted = Del,
    >
{
  fn get_internal_session
    ( self: Box < Self > )
    -> InternalSession < N, C, A, B, Row, Del >
  {
    *self
  }
}

impl < N, C, A, B, Row, Del, K >
  InternalSessionWitness
  < N, C, A, B, Row, Del, K >
  for
  InternalSession < N, C, A, B, Row, Del >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A,
      Deleted = Del,
    >
{
  fn with_internal_session
    ( self: Box < Self >,
      cont: Box < dyn
        NeedInternalSession
          < N, C, A, B, Row, Del, K >
        >,
    ) -> K
  {
    cont.on_internal_session(*self)
  }
}

pub struct WrapInternalSession
  < N, C, A, B, Row, Del >
{
  session:
    Box < dyn
      InternalSessionWitness <
        N, C, A, B, Row, Del,
        Box < dyn Any >
      > >
}

impl < N, C, B, Row, Del >
  TyCon
  for InternalSessionF < N, C, B, Row, Del, >
where
  N: 'static,
  C: 'static,
  B: 'static,
  Row: 'static,
  Del: 'static,
{ }

impl < N, C, A, B, Row, Del >
  TypeApp < A >
  for InternalSessionF < N, C, B, Row, Del >
where
  A: 'static,
  N: 'static,
  C: 'static,
  B: 'static,
  Row: 'static,
  Del: 'static,
{
  type Applied =
    WrapInternalSession <
      N, C, A, B, Row, Del,
    >;
}

struct InternalSessionContWrapper
  < N, C, A, B, Row, Del, K >
{
  cont: Box < dyn
    NeedInternalSession
      < N, C, A, B, Row, Del, K >
  >,
}

impl < N, C, A, B, Row, Del, K >
  NeedInternalSession <
    N, C, A, B, Row, Del, Box < dyn Any >
  >
  for InternalSessionContWrapper <
    N, C, A, B, Row, Del, K
  >
where
  K: 'static,
{
  fn on_internal_session
    ( self: Box < Self >,
      session: InternalSession < N, C, A, B, Row, Del >
    ) -> Box < dyn Any >
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del : Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del,
      >
  {
    let res = self.cont.on_internal_session( session );
    Box::new( res )
  }
}

pub fn wrap_internal_session
  < N, C, A, B, Row, Del >
  ( session1:
      PartialSession <
        N :: Target,
        B
      >
  ) ->
    WrapInternalSession <
      N, C, A, B, Row, Del >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Del: Context,
  Row : RowCon,
  N: 'static,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A,
      Deleted = Del,
    >
{
  let session2 = InternalSession
    :: < N, C, A, B, Row, Del >
    {
      session: session1
    };

  WrapInternalSession {
    session: Box::new( session2 )
  }
}

pub fn with_internal_session
  < N, C, A, B, Row, Del, K >
  ( session:
      WrapInternalSession
      < N, C, A, B, Row, Del >,
    cont1:
      Box < dyn
        NeedInternalSession
        < N, C, A, B, Row, Del, K >
      >,
  ) ->
    Box < K >
where
  N: 'static,
  C: 'static,
  A: 'static,
  B: 'static,
  Row: 'static,
  Del: 'static,
  K: 'static,
{
  let cont2 = InternalSessionContWrapper {
    cont: cont1
  };

  let res = session.session.with_internal_session(
    Box::new( cont2 ) );

  res.downcast().unwrap()
}