use std::marker::PhantomData;

use super::super::{
  cloak_session::*,
  inject_session::*,
};
use crate::internal::{
  base::*,
  functional::*,
};

pub fn selector_to_inject_session<Row1, Row2, C>(
  selector: AppSum<Row2, ()>
) -> AppSum<Row2, InjectSessionF<Row1, C>>
where
  C: Context,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: SumFunctorInject,
{
  lift_sum_inject(SelectorToCont::<Row1, C>(PhantomData), selector)
}

struct SelectorToCont<Row, C>(PhantomData<(Row, C)>);

impl<Row1, Row2, C> InjectLift<AppSum<Row2, SessionF<C>>>
  for SelectorToCont<Row1, C>
where
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: RowCon,
  C: Context,
{
  type InjectF = InjectSessionF<Row1, C>;
  type SourceF = ();
  type TargetF = SessionF<C>;

  fn lift_field<A>(
    self,
    inject1: impl Fn(App<Self::TargetF, A>) -> AppSum<Row2, SessionF<C>>
      + Send
      + 'static,
    _row: App<Self::SourceF, A>,
  ) -> App<Self::InjectF, A>
  where
    A: 'static,
  {
    let inject2 = SessionInjectorImpl {
      injector: Box::new(inject1),
    };

    let inject3 = create_inject_session(inject2);

    wrap_type_app(inject3)
  }
}

struct SessionInjectorImpl<Row, C, A>
{
  injector: Box<
    dyn FnOnce(App<SessionF<C>, A>) -> AppSum<Row, SessionF<C>>
      + Send
      + 'static,
  >,
}

impl<Row1, Row2, C, A> SessionInjector<Row1, C, A>
  for SessionInjectorImpl<Row2, C, A>
where
  Row1: ToRow<Row = Row2>,
{
  fn inject_session(
    self: Box<Self>,
    session: PartialSession<C, A>,
  ) -> AppSum<Row2, SessionF<C>>
  where
    C: Context,
    A: Protocol,
  {
    (self.injector)(wrap_type_app(cloak_session(session)))
  }
}
