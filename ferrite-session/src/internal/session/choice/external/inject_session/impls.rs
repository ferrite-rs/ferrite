use super::{
  super::cloak_session::SessionF,
  structs::*,
};
use crate::internal::{
  base::*,
  functional::*,
  session::choice::run_cont::RunCont,
};

impl<Row, C> TyCon for InjectSessionF<Row, C>
where
  C: 'static,
  Row: 'static,
{
}

impl<A, C, Row> TypeApp<A> for InjectSessionF<Row, C>
where
  C: Context,
  A: 'static,
  Row: 'static,
{
  type Applied = InjectSession<Row, C, A>;
}

impl<Row1, Row2, C, A, SessionSum> RunCont<C, A> for InjectSession<Row1, C, A>
where
  C: Context,
  A: Protocol,
  Row1: ToRow<Row = Row2>,
  Row2: SumApp<SessionF<C>, Applied = SessionSum>,
  SessionSum: Send + 'static,
{
  type Ret = SessionSum;

  fn run_cont(
    self,
    session: PartialSession<C, A>,
  ) -> SessionSum
  {
    run_inject_session(self, session).get_sum()
  }
}
