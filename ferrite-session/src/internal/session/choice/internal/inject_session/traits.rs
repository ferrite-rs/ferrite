use super::super::internal_session::*;
use crate::internal::{
  base::*,
  functional::*,
  protocol::*,
};

pub trait SessionInjector<N, C, A, B, Row, Del>: Send
{
  fn inject_session(
    self: Box<Self>,
    session : PartialSession<N::Target, B>,
  ) -> AppSum<Row, InternalSessionF<N, C, B, Row, Del>>
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del : Context,
    Row : RowCon,
    N : ContextLens<C, InternalChoice<Row>, A, Deleted = Del>;
}
