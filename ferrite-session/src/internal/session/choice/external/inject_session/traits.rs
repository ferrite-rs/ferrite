use core::future::Future;

use crate::internal::{
  base::*,
};

pub trait SessionInjector<C, A>: Send
{
  fn inject_session(
    self: Box<Self>,
    session: PartialSession<C, A>,
  ) -> Box<dyn Future<Output=()> + Send>
  where
    C: Context,
    A: Protocol,
  ;
}
