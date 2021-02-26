use std::{
  any::Any,
  marker::PhantomData,
};

use super::traits::*;
use crate::base::*;

pub struct SessionF<C>
{
  phantom : PhantomData<C>,
}

impl<C, A> CloakedSession<C, A>
where
  C : Context,
  A : Protocol,
{
  pub fn get_session(self) -> PartialSession<C, A>
  {
    self.session.get_partial_session()
  }
}

pub struct CloakedSession<C, A>
{
  pub session : Box<dyn PartialSessionWitness<C, A, Box<dyn Any>>>,
}

pub fn cloak_session<C, A>(
  session : PartialSession<C, A>
) -> CloakedSession<C, A>
where
  C : Context,
  A : Protocol,
{
  CloakedSession {
    session : Box::new(session),
  }
}
