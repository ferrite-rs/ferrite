use serde;
use std::pin::Pin;
use std::marker::PhantomData;
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
            ( SenderOnce < () >,
              SenderOnce < S >
            )
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
      ( SenderOnce < () >,
        SenderOnce < S >
      )
    >
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SerializedSharedChannel < S >
where
  S: SharedProtocol
{
  acquire_sender: IpcSender < () >,
  acquire_receiver: IpcReceiver < () >,
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
  let (sender1, receiver1) = ipc_channel::<()>();
  let (sender2, receiver2) = ipc_channel::<()>();

  let (sender3, receiver3) = opaque_channel();
  let (sender4, receiver4) = opaque_channel();

  RUNTIME.spawn_blocking(move || {
    loop {
      match receiver1.recv() {
        Some(()) => {
          let (sender5, receiver5) = once_channel::<()>();
          let (sender6, receiver6) = once_channel::<S>();

          {
            let channel = channel.clone();
            let sender2 = sender2.clone();
            let receiver3 = receiver3.clone();
            let sender4 = sender4.clone();

            RUNTIME.spawn (async move {
              channel.endpoint.send((sender5, sender6)).unwrap();
              receiver5.recv().await.unwrap();
              receiver6.forward_to(sender4, receiver3);
              sender2.send(());
            });
          }
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
  let (sender1, receiver1) =
    unbounded::<( SenderOnce<()>, SenderOnce<S> )>();

  RUNTIME.spawn(async move {
    loop {
      match receiver1.recv().await {
        Some((sender2, sender3)) => {
          let channel2 = channel.clone();
          RUNTIME.spawn_blocking(move || {
            channel2.acquire_sender.send(());
            channel2.acquire_receiver.recv().unwrap();

            sender2.send(()).unwrap();
            sender3.forward_to(
              channel2.linear_sender,
              channel2.linear_receiver
            );
          });
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
    sender: Receiver <
      ( SenderOnce < () >,
        SenderOnce < S >
      )
    >
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
            ( SenderOnce < () >,
              SenderOnce < S >
            )
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
              ( SenderOnce < () >,
                SenderOnce < S >
              )
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
        ( SenderOnce < () >,
          SenderOnce < S >
        )
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
  let (sender1, receiver1) = once_channel::<()>();
  let (sender2, receiver2) = once_channel::<S>();

  session.endpoint.send( (sender1, sender2) ).unwrap();
  receiver1.recv().await.unwrap();

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
