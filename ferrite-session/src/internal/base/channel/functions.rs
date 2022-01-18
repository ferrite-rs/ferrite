use core::marker::PhantomData;
use std::sync::{
  Arc,
  Mutex,
};

use ipc_channel::ipc;
use tokio::sync::{
  mpsc,
  oneshot,
  Mutex as AsyncMutex,
};

use super::types::*;

pub fn once_channel<T>() -> (SenderOnce<T>, ReceiverOnce<T>)
{
  let (sender, receiver) = oneshot::channel();

  (SenderOnce(sender), ReceiverOnce(receiver))
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>)
{
  let (sender, receiver) = mpsc::unbounded_channel();

  (
    Sender(sender),
    Receiver(Arc::new(AsyncMutex::new(receiver))),
  )
}

pub fn ipc_channel<T>() -> (IpcSender<T>, IpcReceiver<T>)
where
  IpcReceiver<T>: Send,
{
  let (sender, receiver) = opaque_channel();

  (
    IpcSender {
      sender,
      phantom: PhantomData,
    },
    IpcReceiver {
      receiver,
      phantom: PhantomData,
    },
  )
}

pub fn opaque_channel() -> (OpaqueSender, OpaqueReceiver)
{
  let (sender, receiver) = ipc::channel::<()>().unwrap();

  (
    OpaqueSender(Arc::new(Mutex::new(Some(sender.to_opaque())))),
    OpaqueReceiver(Arc::new(Mutex::new(Some(receiver.to_opaque())))),
  )
}
