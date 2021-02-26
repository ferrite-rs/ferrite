use super::{super::cloak_session::*, structs::*};
use crate::{base::*, functional::*, protocol::*};

impl<Row, C> TyCon for InjectSessionF<Row, C>
where
  C : 'static,
  Row : 'static,
{
}

impl<A, C, Row> TypeApp<A> for InjectSessionF<Row, C>
where
  C : Context,
  A : 'static,
  Row : 'static,
{
  type Applied = InjectSession<Row, C, A>;
}

impl<Row, C, A> RunCont<C, A> for InjectSession<Row, C, A>
where
  C : Context,
  A : Protocol,
{
  type Ret = AppliedSum<Row, SessionF<C>>;

  fn run_cont(
    self,
    session : PartialSession<C, A>,
  ) -> AppliedSum<Row, SessionF<C>>
  {

    run_inject_session(self, session)
  }
}
