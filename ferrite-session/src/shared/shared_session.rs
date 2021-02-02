use serde;
use std::pin::Pin;
use std::marker::PhantomData;
use async_macros::join;
use std::future::Future;

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
    Sender <
      SenderOnce <
        ReceiverOnce < S >
      > >
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SerializedSharedChannel < S >
where
  S: SharedProtocol
{
  acquire_sender: OpaqueSender,
  acquire_receiver: OpaqueReceiver,
  linear_sender: OpaqueSender,
  linear_receiver: OpaqueReceiver,
  phantom: PhantomData<S>,
}

impl <S> Clone for
  SerializedSharedChannel < S >
where
  S: SharedProtocol
{
  fn clone(&self) -> Self {
    SerializedSharedChannel {
      acquire_sender: self.acquire_sender.clone(),
      acquire_receiver: self.acquire_receiver.clone(),
      linear_sender: self.linear_sender.clone(),
      linear_receiver: self.linear_receiver.clone(),
      phantom: PhantomData,
    }
  }
}

pub fn serialize_shared_channel <S>
  (channel: SharedChannel<S>)
  -> SerializedSharedChannel<S>
where
  S: SharedProtocol + ForwardChannel
{
  let (sender1, receiver1) = opaque_channel();
  let (sender2, receiver2) = opaque_channel();

  let (sender3, receiver3) = opaque_channel();
  let (sender4, receiver4) = opaque_channel();

  RUNTIME.spawn_blocking(move || {
    loop {
      match receiver1.recv() {
        Some(()) => {
          let (sender5, receiver5) =
            once_channel::<ReceiverOnce<S>>();

          let channel2 = channel.clone();
          let receiver6 = RUNTIME.block_on(async move {
            channel2.endpoint.send(sender5).await.unwrap();
            receiver5.recv().await.unwrap()
          });
          receiver6.forward_to(sender4.clone(), receiver3.clone());
          sender2.send(());
        }
        None => break
      }
    }
  });

  SerializedSharedChannel {
    acquire_sender: sender1,
    acquire_receiver: receiver2,
    linear_sender: sender3,
    linear_receiver: receiver4,
    phantom: PhantomData,
  }
}

pub fn deserialize_shared_channel <S>
  (channel: SerializedSharedChannel<S>)
  -> SharedChannel<S>
where
  S: SharedProtocol + ForwardChannel + Send
{
  let (sender1, receiver1) = unbounded::<SenderOnce<ReceiverOnce<S>>>();

  RUNTIME.spawn(async move {
    loop {
      match receiver1.recv().await {
        Some(sender2) => {
          let channel2 = channel.clone();

          let receiver2 : ReceiverOnce<S> =
            RUNTIME.spawn_blocking(move || {
              channel2.acquire_sender.send(());
              channel2.acquire_receiver.recv::<()>().unwrap();

              < ReceiverOnce<S> >::forward_from(
                channel2.linear_sender,
                channel2.linear_receiver,
              )
            }).await.unwrap();

          sender2.send(receiver2).await.unwrap();
        }
        None => break
      }
    }
  });

  SharedChannel {
    endpoint: sender1
  }
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
        SenderOnce <
          ReceiverOnce < S >
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

  let fut1 = session.endpoint.send( sender );
  let fut2 = async move {
    let receiver2 = receiver.recv().await.unwrap();
    receiver2
  };

  let (receiver2, _) = join!(fut2, fut1).await;

  receiver2
}


impl < A > serde::Serialize
  for SharedChannel < A >
where
  A : SharedProtocol + ForwardChannel
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serialize_shared_channel(self.clone()).serialize(serializer)

    // debug!("serializing shared channel");
    // self.endpoint.serialize(serializer)
  }
}

impl < 'a, A > serde::Deserialize<'a>
  for SharedChannel < A >
where
  A : SharedProtocol + ForwardChannel,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>
  {
    let channel = <
      SerializedSharedChannel<A>
    >::deserialize(deserializer)?;

    Ok(deserialize_shared_channel(channel))

    // let endpoint = <
    //   Sender <
    //     SenderOnce <
    //       ReceiverOnce < A >
    //     >
    //   >
    // >::deserialize(deserializer)?;

    // Ok(SharedChannel{endpoint})
  }
}
