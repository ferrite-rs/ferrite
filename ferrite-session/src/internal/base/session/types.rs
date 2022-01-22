use core::{
  future::Future,
  pin::Pin,
};

use crate::internal::base::{
  context::Context,
  protocol::{
    Protocol,
    ProviderEndpoint,
  },
};

pub type Session<P> = PartialSession<(), P>;

pub struct PartialSession<C, A>
where
  A: Protocol,
  C: Context,
{
  pub(crate) executor: Box<
    dyn FnOnce(
        C::Endpoints,
        ProviderEndpoint<A>,
      ) -> Pin<Box<dyn Future<Output = ()> + Send>>
      + Send,
  >,
}
