use std::marker::PhantomData;

use super::super::{cloak_session::*, inject_session::*};
use crate::{base::*, functional::*};

pub fn selector_to_inject_session<Row, C>(
  selector : AppliedSum<Row, ()>
) -> AppliedSum<Row, InjectSessionF<Row, C>>
where
  C : Context,
  Row : SumFunctorInject,
{

  lift_sum_inject(SelectorToCont::<Row, C>(PhantomData), selector)
}

struct SelectorToCont<Row, C>(PhantomData<(Row, C)>);

impl<Row, C> InjectLift<AppliedSum<Row, SessionF<C>>> for SelectorToCont<Row, C>
where
  C : Context,
  Row : 'static,
{
  type SourceF = ();

  type TargetF = SessionF<C>;

  type InjectF = InjectSessionF<Row, C>;

  fn lift_field<A>(
    self,
    inject1 : impl Fn(Applied<Self::TargetF, A>) -> AppliedSum<Row, SessionF<C>>
      + Send
      + 'static,
    _row : Applied<Self::SourceF, A>,
  ) -> Applied<Self::InjectF, A>
  where
    A : 'static,
  {

    let inject2 = SessionInjectorImpl {
      injector : Box::new(inject1),
    };

    let inject3 = create_inject_session(inject2);

    cloak_applied(inject3)
  }
}

struct SessionInjectorImpl<Row, C, A>
{
  injector : Box<
    dyn FnOnce(Applied<SessionF<C>, A>) -> AppliedSum<Row, SessionF<C>>
      + Send
      + 'static,
  >,
}

impl<Row, C, A> SessionInjector<Row, C, A> for SessionInjectorImpl<Row, C, A>
{
  fn inject_session(
    self: Box<Self>,
    session : PartialSession<C, A>,
  ) -> AppliedSum<Row, SessionF<C>>
  where
    C : Context,
    A : Protocol,
  {

    (self.injector)(cloak_applied(cloak_session(session)))
  }
}
