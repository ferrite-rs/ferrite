use crate::base::*;

pub trait NeedPartialSession<C, A, K>
{
  fn on_partial_session(
    self: Box<Self>,
    session : PartialSession<C, A>,
  ) -> K
  where
    C : Context,
    A : Protocol;
}

pub trait HasPartialSession<C, A>: Send
{
  fn get_partial_session(self: Box<Self>) -> PartialSession<C, A>
  where
    C : Context,
    A : Protocol;
}

pub trait PartialSessionWitness<C, A, K>: HasPartialSession<C, A>
{
  fn with_partial_session(
    self: Box<Self>,
    cont : Box<dyn NeedPartialSession<C, A, K>>,
  ) -> K;
}
