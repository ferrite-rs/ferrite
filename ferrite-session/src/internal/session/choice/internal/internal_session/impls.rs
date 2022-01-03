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

impl<'a, N, C, A, B, Row, Del> TypeApp<'a, A>
  for InternalSessionF<N, C, B, Row, Del>
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

impl<N, C, A, B, Row1, Row2, Del> HasInternalSession<N, C, A, B, Row1, Del>
  for InternalSession<N, C, A, B, Row1, Del>
where
  A: Protocol,
  B: Protocol,
  C: Context,
  Del: Context,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  N: ContextLens<C, InternalChoice<Row1>, A, Deleted = Del>,
{
  fn get_internal_session(
    self: Box<Self>
  ) -> InternalSession<N, C, A, B, Row1, Del>
  {
    *self
  }
}

impl<N, C, A, B, Row1, Row2, Del, K>
  InternalSessionWitness<N, C, A, B, Row1, Del, K>
  for InternalSession<N, C, A, B, Row1, Del>
where
  A: Protocol,
  B: Protocol,
  C: Context,
  Del: Context,
  Row1: ToRow<Row = Row2>,
  Row1: Send + 'static,
  Row2: Send + 'static,
  Row2: RowCon,
  N: ContextLens<C, InternalChoice<Row1>, A, Deleted = Del>,
{
  fn with_internal_session(
    self: Box<Self>,
    cont: Box<dyn NeedInternalSession<N, C, A, B, Row1, Del, K>>,
  ) -> K
  {
    cont.on_internal_session(*self)
  }
}
