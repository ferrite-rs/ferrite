use super::super::cloak_session::*;
use crate::internal::{
  base::*,
  functional::*,
};

pub trait SessionInjector<Row, C, A>: Send
{
  fn inject_session(
    self: Box<Self>,
    session : PartialSession<C, A>,
  ) -> AppSum<Row, SessionF<C>>
  where
    C : Context,
    A : Protocol;
}
