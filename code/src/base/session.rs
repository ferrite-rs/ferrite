
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
    dyn FnOnce( C::Values, Sender < A::Value > )
      -> Pin < Box < dyn Future < Output=() > + Send > >
    + Send
  >
}

pub fn unsafe_create_session
  < I, P, Fut >
  (
    executor : impl
      FnOnce( I::Values, Sender < P::Value > )
        -> Fut
      + Send + 'static
  ) ->
    PartialSession < I, P >
where
  P : Protocol + 'static,
  I : Context + 'static,
  Fut : Future < Output=() > + Send
{
  let executor2
    : Box <
        dyn FnOnce( I::Values, Sender < P::Value > )
          -> Pin < Box < dyn Future < Output=() > + Send > >
        + Send
      >
    = Box::new (
        move | ins, sender | {
          Box::pin ( async {
            executor ( ins, sender ).await;
          } )
        });

  PartialSession {
    executor : executor2
  }
}

pub async fn run_partial_session
  < I, P >
  ( session : PartialSession < I, P >
  , ins : I :: Values
  , sender : Sender < P :: Value >
  )
where
  P: Protocol,
  I: Context
{
  (session.executor)(ins, sender).await;
}
