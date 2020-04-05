
use std::pin::Pin;
use std::future::Future;
use async_std::sync::Sender;

use crate::base::protocol::{ Protocol };
use crate::base::context::{ Context };

/// A session builder is a consumer for the given list of
/// input processes and output a process with given Out type.
pub type Session < P > =
  PartialSession < (), P >;

pub struct PartialSession
  < C, A >
where
  A: Protocol,
  C: Context
{
  executor : Box <
    dyn FnOnce( C::Values, Sender < A::Payload > )
      -> Pin < Box < dyn Future < Output=() > + Send > >
    + Send
  >
}

pub fn unsafe_create_session
  < I, P, Fut >
  (
    executor : impl
      FnOnce( I::Values, Sender < P::Payload > )
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
        dyn FnOnce( I::Values, Sender < P::Payload > )
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
  , ctx : C :: Values
  , sender : Sender < A :: Payload >
  )
where
  A: Protocol,
  C: Context
{
  (session.executor)(ctx, sender).await;
}
