use core::marker::PhantomData;

use tokio::task;

use super::types::*;
use crate::internal::base::{
  channel::*,
  protocol::*,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SerializedSharedChannel<S>
where
  S: SharedProtocol,
{
  acquire_sender: IpcSender<()>,
  acquire_receiver: IpcReceiver<()>,
  linear_sender: OpaqueSender,
  linear_receiver: OpaqueReceiver,
  phantom: PhantomData<S>,
}

impl<S> Clone for SerializedSharedChannel<S>
where
  S: SharedProtocol,
{
  fn clone(&self) -> Self
  {
    SerializedSharedChannel {
      acquire_sender: self.acquire_sender.clone(),
      acquire_receiver: self.acquire_receiver.clone(),
      linear_sender: self.linear_sender.clone(),
      linear_receiver: self.linear_receiver.clone(),
      phantom: PhantomData,
    }
  }
}

fn serialize_shared_channel<S>(
  channel: SharedChannel<S>
) -> SerializedSharedChannel<S>
where
  S: SharedProtocol + ForwardChannel,
{
  let (sender1, receiver1) = ipc_channel::<()>();

  let (sender2, receiver2) = ipc_channel::<()>();

  let (sender3, receiver3) = opaque_channel();

  let (sender4, receiver4) = opaque_channel();

  task::spawn(async move {
    loop {
      let receiver1 = receiver1.clone();

      let signal = task::spawn_blocking(move || receiver1.recv())
        .await
        .unwrap();

      match signal {
        Some(()) => {
          let (sender5, receiver5) = once_channel::<()>();

          let (sender6, receiver6) = once_channel::<S>();

          {
            let channel = channel.clone();

            let sender2 = sender2.clone();

            let receiver3 = receiver3.clone();

            let sender4 = sender4.clone();

            debug!("[serialize_shared_channel] acquiring local shared channel");

            channel.endpoint.send((sender5, sender6)).unwrap();

            receiver5.recv().await.unwrap();

            debug!("[serialize_shared_channel] acquired local shared channel");

            sender2.send(());

            receiver6.forward_to(sender4, receiver3);
          }
        }
        None => break,
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

fn deserialize_shared_channel<S>(
  channel: SerializedSharedChannel<S>
) -> SharedChannel<S>
where
  S: SharedProtocol + ForwardChannel + Send,
{
  let (sender1, receiver1) = unbounded::<(SenderOnce<()>, SenderOnce<S>)>();

  task::spawn(async move {
    while let Some((sender2, sender3)) = receiver1.recv().await {
      debug!("[deserialize_shared_channel] acquiring remote shared channel");

      channel.acquire_sender.send(());

      let acquire_receiver = channel.acquire_receiver.clone();

      task::spawn_blocking(move || {
        acquire_receiver.recv().unwrap();
      })
      .await
      .unwrap();

      debug!("[deserialize_shared_channel] acquired remote shared channel");

      sender2.send(()).unwrap();

      let channel2 = channel.clone();

      sender3.forward_to(channel2.linear_sender, channel2.linear_receiver);
    }
  });

  SharedChannel { endpoint: sender1 }
}

impl<A> serde::Serialize for SharedChannel<A>
where
  A: SharedProtocol + ForwardChannel,
{
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serialize_shared_channel(self.clone()).serialize(serializer)
  }
}

impl<'a, A> serde::Deserialize<'a> for SharedChannel<A>
where
  A: SharedProtocol + ForwardChannel,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>,
  {
    let channel = <SerializedSharedChannel<A>>::deserialize(deserializer)?;

    Ok(deserialize_shared_channel(channel))
  }
}
