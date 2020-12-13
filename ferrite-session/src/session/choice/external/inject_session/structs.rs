use std::marker::PhantomData;

use crate::base::*;
use crate::functional::*;

use super::traits::*;
use super::super::cloak_session::*;

pub struct InjectSessionF < Row, C >
  ( PhantomData <( Row, C )> );

pub struct InjectSession
  < Row, C, A >
{
  injector:
    Box < dyn
      SessionInjector < Row, C, A >
    >
}

pub fn create_inject_session < Row, C, A, I >
  ( injector: I )
  -> InjectSession < Row, C, A >
where
  I: SessionInjector < Row, C, A > + 'static
{
  InjectSession {
    injector: Box::new( injector )
  }
}

pub fn run_inject_session
  < Row, C, A >
  ( inject: InjectSession < Row, C, A >,
    session: PartialSession < C, A >
  ) ->
    AppliedSum <
      Row,
      SessionF < C >
    >
where
  C: Context,
  A: Protocol,
{
  inject.injector.inject_session( session )
}
