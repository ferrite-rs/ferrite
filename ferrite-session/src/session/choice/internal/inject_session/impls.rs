use super::{
  super::internal_session::*,
  structs::*,
};
use crate::{
  base::*,
  functional::*,
  protocol::*,
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

impl<N, C, A, B, Row, Del> RunCont<N::Target, B>
  for InjectSession<N, C, A, B, Row, Del>
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N : ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
{
  type Ret = AppliedSum<Row, InternalSessionF<N, C, B, Row, Del>>;

  fn run_cont(
    self,
    session : PartialSession<N::Target, B>,
  ) -> AppliedSum<Row, InternalSessionF<N, C, B, Row, Del>>
  {
    self.injector.inject_session(session)
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
