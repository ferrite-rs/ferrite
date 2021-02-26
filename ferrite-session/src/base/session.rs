use std::{
  future::Future,
  pin::Pin,
};

use crate::base::{
  channel::SenderOnce,
  context::Context,
  protocol::Protocol,
};

pub type Session<P> = PartialSession<(), P>;

pub struct PartialSession<C, A>
where
  A : Protocol,
  C : Context,
{
  executor : Box<
    dyn FnOnce(
        C::Endpoints,
        SenderOnce<A>,
      ) -> Pin<Box<dyn Future<Output = ()> + Send>>
      + Send,
  >,
}

pub fn unsafe_create_session<C, A, Fut>(
  executor : impl FnOnce(C::Endpoints, SenderOnce<A>) -> Fut + Send + 'static
) -> PartialSession<C, A>
where
  A : Protocol,
  C : Context,
  Fut : Future<Output = ()> + Send,
{
  let executor2 : Box<
    dyn FnOnce(
        C::Endpoints,
        SenderOnce<A>,
      ) -> Pin<Box<dyn Future<Output = ()> + Send>>
      + Send,
  > = Box::new(move |ctx, sender| {
    Box::pin(async {
      executor(ctx, sender).await;
    })
  });

  PartialSession {
    executor : executor2,
  }
}

pub async fn unsafe_run_session<C, A>(
  session : PartialSession<C, A>,
  ctx : C::Endpoints,
  sender : SenderOnce<A>,
) where
  A : Protocol,
  C : Context,
{
  (session.executor)(ctx, sender).await;
}
