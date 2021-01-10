
use std::pin::Pin;
use async_std::prelude::{ Future, FutureExt };

use crate::base::*;
use super::protocol::SharedProtocol;

pub struct SharedSession < S >
where
  S : SharedProtocol
{
  executor :
    Box < dyn
      FnOnce
        ( Receiver <
            SenderOnce <
              ReceiverOnce < S >
            >
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

pub struct SharedChannel < S >
where
  S : SharedProtocol
{
  endpoint :
    Sender < Forward <
      SenderOnce <
        ( Payload<()>,
          ReceiverOnce < S >
        )
      >
    > >
}

impl < S > Clone for
  SharedChannel < S >
where
  S : SharedProtocol
{
  fn clone ( &self ) -> Self {
    SharedChannel {
      endpoint : self.endpoint.clone()
    }
  }
}

pub async fn unsafe_run_shared_session < S >
  ( session: SharedSession < S >,
    sender: Receiver < SenderOnce < ReceiverOnce < S > > >
  )
where
  S : SharedProtocol
{
  (session.executor)(sender).await;
}

pub fn unsafe_create_shared_session
  < S, Fut >
  ( executor1 : impl
      FnOnce
        ( Receiver <
            SenderOnce <
              ReceiverOnce < S >
            >
          >
        )
        -> Fut
      + Send + 'static
  ) ->
    SharedSession < S >
where
  S : SharedProtocol,
  Fut : Future < Output=() > + Send
{
  let executor
    : Box <
        dyn FnOnce
          ( Receiver <
              SenderOnce <
                ReceiverOnce < S >
              >
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

  SharedSession { executor }
}

pub fn unsafe_create_shared_channel < S >
  () ->
    ( SharedChannel < S >,
      Receiver <
        Forward <
          SenderOnce <
            ( Payload<()>,
              ReceiverOnce < S >
            )
          >
        >
      >
    )
where
  S : SharedProtocol
{
  let ( sender, receiver ) = unbounded();

  ( SharedChannel { endpoint: sender }, receiver )
}

pub async fn unsafe_receive_shared_channel < S >
  ( session : SharedChannel < S > )
  -> ReceiverOnce < S >
where
  S : SharedProtocol
{
  let (sender, receiver) = once_channel();

  let fut1 = session.endpoint.send( Forward(sender) );
  let fut2 = async move {
    let (_, receiver2) = receiver.recv().await.unwrap();
    receiver2
  };

  let (receiver2, _) = fut2.join(fut1).await;

  receiver2
}


impl < A > serde::Serialize
  for SharedChannel < A >
where
  A : SharedProtocol
    + ForwardChannel
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    debug!("serializing shared channel");
    self.endpoint.serialize(serializer)
  }
}

impl < 'a, A > serde::Deserialize<'a>
  for SharedChannel < A >
where
  A : SharedProtocol
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>
  {
    debug!("deserializing shared channel");

    let endpoint = <
      Sender <
        SenderOnce <
          ReceiverOnce < A >
        >
      >
    >::deserialize(deserializer)?;

    Ok(SharedChannel{endpoint})
  }
}
