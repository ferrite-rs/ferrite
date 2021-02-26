use std::any::Any;

use super::{
  structs::*,
  traits::*,
};
use crate::base::*;

struct SessionContWrapper<C, A, K>
{
  cont : Box<dyn NeedPartialSession<C, A, K>>,
}

impl<C, A, K> NeedPartialSession<C, A, Box<dyn Any>>
  for SessionContWrapper<C, A, K>
where
  K : 'static,
{
  fn on_partial_session(
    self: Box<Self>,
    session : PartialSession<C, A>,
  ) -> Box<dyn Any>
  where
    C : Context,
    A : Protocol,
  {
    let res = self.cont.on_partial_session(session);

    Box::new(res)
  }
}

pub fn with_session<C, A, K>(
  session : CloakedSession<C, A>,
  cont1 : Box<dyn NeedPartialSession<C, A, K>>,
) -> Box<K>
where
  C : 'static,
  A : 'static,
  K : 'static,
{
  let cont2 = SessionContWrapper { cont : cont1 };

  let res = session.session.with_partial_session(Box::new(cont2));

  res.downcast().unwrap()
}
