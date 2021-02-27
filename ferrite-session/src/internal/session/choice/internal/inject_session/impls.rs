use super::structs::*;
use crate::internal::{
  base::*,
  functional::*,
  protocol::*,
  session::choice::{
    internal::internal_session::InternalSessionF,
    run_cont::RunCont,
  },
};

impl<N, C, B, Row, Del> TyCon for InjectSessionF<N, C, B, Row, Del>
where
  N : 'static,
  C : 'static,
  B : 'static,
  Row : 'static,
  Del : 'static,
{
}

impl<N, C1, C2, A, B, Row, Del, SessionSum> RunCont<C2, B>
  for InjectSession<N, C1, A, B, Row, Del>
where
  A : Protocol,
  B : Protocol,
  C1 : Context,
  C2 : Context,
  Del : Context,
  Row : RowCon,
  Row : SumApp<InternalSessionF<N, C1, B, Row, Del>, Applied = SessionSum>,
  N : ContextLens<C1, InternalChoice<Row>, A, Deleted = Del, Target = C2>,
{
  type Ret = SessionSum;

  fn run_cont(
    self,
    session : PartialSession<N::Target, B>,
  ) -> SessionSum
  {
    self.injector.inject_session(session).get_sum()
  }
}

impl<N, C, A, B, Row, Del> TypeApp<A> for InjectSessionF<N, C, B, Row, Del>
where
  N : Send + 'static,
  C : Send + 'static,
  A : Send + 'static,
  B : Send + 'static,
  Row : Send + 'static,
  Del : Send + 'static,
{
  type Applied = InjectSession<N, C, A, B, Row, Del>;
}
