use std::marker::PhantomData;

use super::{
  super::cloak_session::*,
  traits::*,
};
use crate::internal::{
  base::*,
  functional::*,
};

pub struct InjectSessionF<C>(PhantomData<C>);

pub struct InjectExternal<C, A>
{
  injector: Box<dyn SessionInjector<C, A>>,
}

pub fn create_inject_session<Row, C, A, I>(
  injector: I
) -> InjectExternal<Row, C, A>
where
  I: SessionInjector<Row, C, A> + 'static,
{
  InjectExternal {
    injector: Box::new(injector),
  }
}

pub fn run_inject_session<Row, C, A>(
  inject: InjectExternal<Row, C, A>,
  session: PartialSession<C, A>,
) -> AppSum<Row::Row, SessionF<C>>
where
  C: Context,
  A: Protocol,
  Row: ToRow,
{
  inject.injector.inject_session(session)
}
