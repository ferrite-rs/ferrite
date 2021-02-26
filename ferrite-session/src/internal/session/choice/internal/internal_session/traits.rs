use super::structs::*;
use crate::internal::{
  base::*,
  functional::*,
  protocol::*,
};

pub trait HasInternalSession<N, C, A, B, Row, Del>: Send
{
  fn get_internal_session(
    self: Box<Self>
  ) -> InternalSession<N, C, A, B, Row, Del>
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del : Context,
    Row : RowCon,
    N : ContextLens<C, InternalChoice<Row>, A, Deleted = Del>;
}

pub trait NeedInternalSession<N, C, A, B, Row, Del, K>
{
  fn on_internal_session(
    self: Box<Self>,
    session : InternalSession<N, C, A, B, Row, Del>,
  ) -> K
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del : Context,
    Row : RowCon,
    N : ContextLens<C, InternalChoice<Row>, A, Deleted = Del>;
}

pub trait InternalSessionWitness<N, C, A, B, Row, Del, K>:
  HasInternalSession<N, C, A, B, Row, Del>
{
  fn with_internal_session(
    self: Box<Self>,
    cont : Box<dyn NeedInternalSession<N, C, A, B, Row, Del, K>>,
  ) -> K;
}
