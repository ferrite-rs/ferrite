use super::{
  structs::*,
  traits::*,
};
use crate::internal::{
  base::*,
  functional::*,
  protocol::*,
};

impl<N, C, B, Row, Del> TyCon for InternalSessionF<N, C, B, Row, Del>
where
  N: 'static,
  C: 'static,
  B: 'static,
  Row: 'static,
  Del: 'static,
{
}

impl<N, C, A, B, Row, Del> TypeApp<A> for InternalSessionF<N, C, B, Row, Del>
where
  A: 'static,
  N: 'static,
  C: 'static,
  B: 'static,
  Row: 'static,
  Del: 'static,
{
  type Applied = CloakInternalSession<N, C, A, B, Row, Del>;
}

impl<N, C, A, B, Row, Del> HasInternalSession<N, C, A, B, Row, Del>
  for InternalSession<N, C, A, B, Row, Del>
where
  A: Protocol,
  B: Protocol,
  C: Context,
  Del: Context,
  Row: RowCon,
  N: ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
{
  fn get_internal_session(
    self: Box<Self>
  ) -> InternalSession<N, C, A, B, Row, Del>
  {
    *self
  }
}

impl<N, C, A, B, Row, Del, K> InternalSessionWitness<N, C, A, B, Row, Del, K>
  for InternalSession<N, C, A, B, Row, Del>
where
  A: Protocol,
  B: Protocol,
  C: Context,
  Del: Context,
  Row: RowCon,
  N: ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
{
  fn with_internal_session(
    self: Box<Self>,
    cont: Box<dyn NeedInternalSession<N, C, A, B, Row, Del, K>>,
  ) -> K
  {
    cont.on_internal_session(*self)
  }
}
