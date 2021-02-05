use serde;
use std::pin::Pin;
use std::marker::PhantomData;
use std::future::Future;

use crate::base::*;
use super::protocol::SharedProtocol;

pub type SharedPayload < S > =
  ( SenderOnce < () >,
    SenderOnce < S >,
    SenderOnce < () >,
  );

pub struct SharedSession < S >
where
  S : SharedProtocol
{
  executor :
    Box < dyn
      FnOnce
        ( Receiver < SharedPayload < S > >
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
  pub endpoint :
    Sender < SharedPayload < S > >
}

#[derive( serde::Serialize, serde::Deserialize)]
pub struct SerializedSharedChannel < S >
where
  S: SharedProtocol
{
  acquire_sender: IpcSender < () >,
  acquire_receiver: IpcReceiver < () >,
  release_receiver: IpcReceiver < () >,
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
      release_receiver: self.release_receiver.clone(),
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
  let (acquire_sender1, acquire_receiver1) =
    ipc_channel::< () >();

  let (acquire_sender2, acquire_receiver2) =
    ipc_channel::< () >();

  let (release_sender1, release_receiver1) =
    ipc_channel::< () >();

  let (sender3, receiver3) = opaque_channel();
  let (sender4, receiver4) = opaque_channel();

  RUNTIME.spawn_blocking(move || {
    loop {
      match acquire_receiver1.recv() {
        Some(()) => {
          let (sender5, receiver5) = once_channel::<()>();
          let (sender6, receiver6) = once_channel::<()>();

          let sender7 = < SenderOnce < S > >::forward_from(
            sender4.clone(), receiver3.clone() );

          channel.endpoint.send((sender5, sender7, sender6)).unwrap();

          {
            let acquire_sender2 = acquire_sender2.clone();
            let release_sender1 = release_sender1.clone();
            RUNTIME.block_on (async move {
              receiver5.recv().await.unwrap();
              acquire_sender2.send(());

              receiver6.recv().await.unwrap();
              release_sender1.send(());
            });
          }
        }
        None => break
      }
    }
  });

  SerializedSharedChannel {
    acquire_sender: acquire_sender1,
    acquire_receiver: acquire_receiver2,
    release_receiver: release_receiver1,
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
  let (sender1, receiver1) =
    unbounded::< SharedPayload < S > >();

  RUNTIME.spawn(async move {
    loop {
      match receiver1.recv().await {
        Some((sender1, sender2, sender3)) => {
          let channel = channel.clone();
          channel.acquire_sender.send(());
          RUNTIME.spawn_blocking(move || {
            channel.acquire_receiver.recv().unwrap();
            sender1.send(()).unwrap();

            sender2.forward_to(channel.linear_sender, channel.linear_receiver);
            channel.release_receiver.recv().unwrap();
            sender3.send(()).unwrap();
          }).await.unwrap();
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
    sender: Receiver < SharedPayload < S > >
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
        ( Receiver < SharedPayload < S > > )
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
          ( Receiver < SharedPayload < S > > )
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
      Receiver < SharedPayload < S > >
    )
where
  S : SharedProtocol
{
  let ( sender, receiver ) = unbounded();

  ( SharedChannel { endpoint: sender }, receiver )
}

pub fn unsafe_receive_shared_channel < S >
  ( session : SharedChannel < S > )
  -> (ReceiverOnce < () >, ReceiverOnce < S >, ReceiverOnce < () >)
where
  S : SharedProtocol
{
  let (sender1, receiver1) = once_channel::<()>();
  let (sender2, receiver2) = once_channel::<S>();
  let (sender3, receiver3) = once_channel::<()>();

  session.endpoint.send( (sender1, sender2, sender3) ).unwrap();

  (receiver1, receiver2, receiver3)
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
  }
}
