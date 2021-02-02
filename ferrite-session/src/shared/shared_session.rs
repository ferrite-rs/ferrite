
use std::pin::Pin;
use std::sync::Arc;
use std::marker::PhantomData;
use ipc_channel::ipc;
use async_macros::join;
use std::future::Future;
use tokio::{task};

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

pub struct SerializedSharedChannel < S >
where
  S: SharedProtocol
{
  acquire_sender: ipc::IpcSender<()>,
  acquire_receiver: ipc::IpcReceiver<()>,
  linear_sender: ipc::OpaqueIpcSender,
  linear_receiver: ipc::OpaqueIpcReceiver,
  phantom: PhantomData<S>,
}

pub fn serialize_shared_channel <S>
  (channel: SharedChannel<S>)
  -> SerializedSharedChannel<S>
where
  S: SharedProtocol + ForwardChannel
{
  let (sender1, receiver1) = ipc::channel::<()>().unwrap();
  let (sender2, receiver2) = ipc::channel::<()>().unwrap();

  let (sender3, receiver3) = ipc::channel::<()>().unwrap();
  let (sender4, receiver4) = ipc::channel::<()>().unwrap();

  // task::spawn(async move {
  //   loop {
  //     task::spawn_blocking(|| {
  //       receiver1.recv().unwrap()
  //     }).await.unwrap();
  //   }
  // });

  SerializedSharedChannel {
    acquire_sender: sender1,
    acquire_receiver: receiver2,
    linear_sender: sender3.to_opaque(),
    linear_receiver: receiver4.to_opaque(),
    phantom: PhantomData,
  }
}

// fn forward_serialized_channel_channel
//   ( receiver1: ipc::IpcReceiver<()>,
//     sender1: ipc::IpcSender<()>,
//     receiver2: ipc::OpaqueIpcReceiver,
//     sender2: ipc::OpaqueIpcSender,
//   )
// {
//   match receiver1.recv() {
//     Ok(()) => {
//     }
//     Err(_) => {}
//   }
// }

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
    debug!("serializing shared channel");
    self.endpoint.serialize(serializer)
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
