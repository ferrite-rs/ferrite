
use std::pin::Pin;
use async_std::prelude::{ Future, FutureExt };
use async_std::sync::{ Sender, Receiver, channel };

use super::protocol::SharedProtocol;

pub struct SuspendedSharedSession < A >
where
  A : SharedProtocol
{
  executor :
    Box < dyn
      FnOnce
        ( Sender <
            Receiver < A >
          >
        ) ->
          Pin < Box <
            dyn Future <
              Output = ()
            > + Send
          > >
      + Send
    >
}

pub struct SharedSession < A >
where
  A : SharedProtocol
{
  endpoint :
    Sender <
      Sender <
        Receiver < A >
      >
    >
}

impl < A > Clone for
  SharedSession < A >
where
  A : SharedProtocol
{
  fn clone(&self) -> Self {
    SharedSession {
      endpoint : self.endpoint.clone()
    }
  }
}

pub async fn unsafe_run_shared_session < A >
  ( session: SuspendedSharedSession < A >,
    sender: Sender < Receiver < A > >
  )
where
  A : SharedProtocol
{
  (session.executor)(sender).await;
}

pub fn unsafe_create_shared_session
  < A, Fut >
  ( executor1 : impl
      FnOnce
        ( Sender <
            Receiver < A >
          >
        )
        -> Fut
      + Send + 'static
  ) ->
    SuspendedSharedSession < A >
where
  A : SharedProtocol,
  Fut : Future < Output=() > + Send
{
  let executor
    : Box <
        dyn FnOnce
          ( Sender <
              Receiver < A >
            >
          )
          -> Pin < Box < dyn Future < Output=() > + Send > >
        + Send
      >
    = Box::new (
        move | sender | {
          Box::pin ( async {
            executor1 ( sender ).await;
          } )
        });

  SuspendedSharedSession { executor }
}

pub fn unsafe_offer_shared_session < A >
  () ->
    ( SharedSession < A >,
      Receiver <
        Sender <
          Receiver < A >
        >
      >
    )
where
  A : SharedProtocol
{
  let ( sender, recceiver ) = channel(1);

  ( SharedSession { endpoint: sender }, recceiver )
}

pub async fn unsafe_receive_shared_session < A >
  ( session : SharedSession < A > )
  -> Receiver < A >
where
  A : SharedProtocol
{
  let (sender, receiver) = channel(1);

  let fut1 = session.endpoint.send( sender );
  let fut2 = async move {
    receiver.recv().await.unwrap()
  };

  let (receiver2, _) = fut2.join(fut1).await;

  receiver2
}