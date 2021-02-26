use std::any::Any;

use super::{
  structs::*,
  traits::*,
};
use crate::internal::{
  base::*,
  functional::*,
  protocol::*,
};

pub fn cloak_internal_session<N, C, A, B, Row, Del>(
  session1 : PartialSession<N::Target, B>
) -> CloakInternalSession<N, C, A, B, Row, Del>
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N : 'static,
  N : ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
{
  let session2 = InternalSession::<N, C, A, B, Row, Del> { session : session1 };

  CloakInternalSession {
    session : Box::new(session2),
  }
}

pub fn with_internal_session<N, C, A, B, Row, Del, K>(
  session : CloakInternalSession<N, C, A, B, Row, Del>,
  cont1 : Box<dyn NeedInternalSession<N, C, A, B, Row, Del, K>>,
) -> Box<K>
where
  N : 'static,
  C : 'static,
  A : 'static,
  B : 'static,
  Row : 'static,
  Del : 'static,
  K : 'static,
{
  let cont2 = InternalSessionContWrapper { cont : cont1 };

  let res = session.session.with_internal_session(Box::new(cont2));

  res.downcast().unwrap()
}

struct InternalSessionContWrapper<N, C, A, B, Row, Del, K>
{
  cont : Box<dyn NeedInternalSession<N, C, A, B, Row, Del, K>>,
}

impl<N, C, A, B, Row, Del, K>
  NeedInternalSession<N, C, A, B, Row, Del, Box<dyn Any>>
  for InternalSessionContWrapper<N, C, A, B, Row, Del, K>
where
  K : 'static,
{
  fn on_internal_session(
    self: Box<Self>,
    session : InternalSession<N, C, A, B, Row, Del>,
  ) -> Box<dyn Any>
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Del : Context,
    Row : RowCon,
    N : ContextLens<C, InternalChoice<Row>, A, Deleted = Del>,
  {
    let res = self.cont.on_internal_session(session);

    Box::new(res)
  }
}
