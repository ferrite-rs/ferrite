
use std::pin::Pin;
use std::future::Future;

use crate::base::channel::SenderOnce;
use crate::base::protocol::{ Protocol };
use crate::base::context::{ Context };

/// A session builder is a consumer for the given list of
/// input context and output a protocol with given Out type.
pub type Session < P > =
  PartialSession < (), P >;

pub struct PartialSession
  < C, A >
where
  A: Protocol,
  C: Context
{
  executor : Box <
    dyn FnOnce( C::Endpoints, SenderOnce < A > )
      -> Pin < Box < dyn Future < Output=() > + Send > >
    + Send
  >
}

pub fn unsafe_create_session
  < I, P, Fut >
  (
    executor : impl
      FnOnce( I::Endpoints, SenderOnce < P > )
        -> Fut
      + Send + 'static
  ) ->
    PartialSession < I, P >
where
  P : Protocol,
  I : Context,
  Fut : Future < Output=() > + Send
{
  let executor2
    : Box <
        dyn FnOnce( I::Endpoints, SenderOnce < P > )
          -> Pin < Box < dyn Future < Output=() > + Send > >
        + Send
      >
    = Box::new (
        move | ctx, sender | {
          Box::pin ( async {
            executor ( ctx, sender ).await;
          } )
        });

  PartialSession {
    executor : executor2
  }
}

pub async fn unsafe_run_session
  < C, A >
  ( session : PartialSession < C, A >
  , ctx : C :: Endpoints
  , sender : SenderOnce < A >
  )
where
  A: Protocol,
  C: Context
{
  (session.executor)(ctx, sender).await;
}
