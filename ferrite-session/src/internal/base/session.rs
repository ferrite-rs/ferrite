use std::{
  future::Future,
  pin::Pin,
};

use crate::internal::base::{
  context::Context,
  protocol::Protocol,
};

pub type Session<P> = PartialSession<(), P>;

pub struct PartialSession<C, A>
where
  A: Protocol,
  C: Context,
{
  executor: Box<
    dyn FnOnce(
        C::Endpoints,
        A::ProviderEndpoint,
      ) -> Pin<Box<dyn Future<Output = ()> + Send>>
      + Send,
  >,
}

pub fn unsafe_create_session<C, A, Cont, Fut>(
  executor: Cont
) -> PartialSession<C, A>
where
  A: Protocol,
  C: Context,
  Cont: FnOnce(C::Endpoints, A::ProviderEndpoint) -> Fut + Send + 'static,
  Fut: Future<Output = ()> + Send,
{
  let executor2: Box<
    dyn FnOnce(
        C::Endpoints,
        A::ProviderEndpoint,
      ) -> Pin<Box<dyn Future<Output = ()> + Send>>
      + Send,
  > = Box::new(move |ctx, provider_end| {
    Box::pin(async {
      executor(ctx, provider_end).await;
    })
  });

  PartialSession {
    executor: executor2,
  }
}

pub async fn unsafe_run_session<C, A>(
  session: PartialSession<C, A>,
  ctx: C::Endpoints,
  provider_end: A::ProviderEndpoint,
) where
  A: Protocol,
  C: Context,
{
  (session.executor)(ctx, provider_end).await;
}
