use std::{
  future::Future,
  pin::Pin,
};

use tokio::task;

use crate::internal::{
  base::{
    context::Context,
    protocol::{
      Protocol,
      ProviderEndpoint,
    },
  },
  functional::type_app::wrap_type_app,
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
        ProviderEndpoint<A>,
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
        ProviderEndpoint<A>,
      ) -> Pin<Box<dyn Future<Output = ()> + Send>>
      + Send,
  > = Box::new(move |ctx, provider_end| {
    Box::pin(async {
      // run the executor as a separate async task to avoid stack overflow
      // due to overly deeply nested futures.
      task::spawn(async move {
        executor(ctx, provider_end.get_applied()).await;
      })
      .await
      .unwrap();
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
  task::spawn(async move {
    (session.executor)(ctx, wrap_type_app(provider_end)).await;
  })
  .await
  .unwrap();
}
