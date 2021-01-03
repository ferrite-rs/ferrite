
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
            Sender <
              Receiver < S >
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
    Sender <
      Sender <
        Receiver < S >
      >
    >
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
    sender: Receiver < Sender < Receiver < S > > >
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
            Sender <
              Receiver < S >
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
              Sender <
                Receiver < S >
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
        Sender <
          Receiver < S >
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
  -> Receiver < S >
where
  S : SharedProtocol
{
  let (sender, receiver) = bounded(1);

  let fut1 = session.endpoint.send( sender );
  let fut2 = async move {
    receiver.recv().await.unwrap()
  };

  let (receiver2, _) = fut2.join(fut1).await;

  receiver2
}


impl < A > serde::Serialize
  for SharedChannel < A >
where
  A : SharedProtocol
    + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
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
    let endpoint = <
      Sender <
        Sender <
          Receiver < A >
        >
      >
    >::deserialize(deserializer)?;

    Ok(SharedChannel{endpoint})
  }
}
