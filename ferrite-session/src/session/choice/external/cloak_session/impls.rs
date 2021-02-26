use super::{
  structs::*,
  traits::*,
};
use crate::{
  base::*,
  functional::*,
};

impl<C, A> HasPartialSession<C, A> for PartialSession<C, A>
where
  C : Context,
  A : Protocol,
{
  fn get_partial_session(self: Box<Self>) -> PartialSession<C, A>
  {
    *self
  }
}

impl<C, A, K> PartialSessionWitness<C, A, K> for PartialSession<C, A>
where
  C : Context,
  A : Protocol,
{
  fn with_partial_session(
    self: Box<Self>,
    cont : Box<dyn NeedPartialSession<C, A, K>>,
  ) -> K
  {
    cont.on_partial_session(*self)
  }
}

impl<C> TyCon for SessionF<C> where C : Send + 'static {}

impl<C, A> TypeApp<A> for SessionF<C>
where
  C : Send + 'static,
  A : Send + 'static,
{
  type Applied = CloakedSession<C, A>;
}
