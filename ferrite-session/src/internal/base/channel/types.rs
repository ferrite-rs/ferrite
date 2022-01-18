use core::{
  marker::PhantomData,
  mem,
  ops::DerefMut,
};
use std::sync::{
  Arc,
  Mutex,
};

use ipc_channel::ipc;
use serde::{
  self,
  Deserialize,
  Serialize,
};
use tokio::sync::{
  mpsc,
  oneshot,
  Mutex as AsyncMutex,
};

pub struct Value<T>(pub T);

pub struct Sender<T>(pub mpsc::UnboundedSender<T>);

pub struct Receiver<T>(pub Arc<AsyncMutex<mpsc::UnboundedReceiver<T>>>);

pub struct SenderOnce<T>(pub oneshot::Sender<T>);

pub struct ReceiverOnce<T>(pub oneshot::Receiver<T>);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OpaqueReceiver(pub Arc<Mutex<Option<ipc::OpaqueIpcReceiver>>>);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OpaqueSender(pub Arc<Mutex<Option<ipc::OpaqueIpcSender>>>);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct IpcSender<T>
{
  pub sender: OpaqueSender,
  pub phantom: PhantomData<T>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct IpcReceiver<T>
{
  pub receiver: OpaqueReceiver,
  pub phantom: PhantomData<T>,
}

// TODO: Define proper error type
#[derive(Debug)]
pub struct SendError(pub String);

impl<T> Receiver<T>
{
  pub async fn recv(&self) -> Option<T>
  {
    self.0.lock().await.recv().await
  }
}

impl<T> SenderOnce<T>
{
  pub fn send(
    self,
    msg: T,
  ) -> Result<(), SendError>
  {
    self
      .0
      .send(msg)
      .map_err(|_| SendError(String::from("failed to send")))
  }
}

impl<T> ReceiverOnce<T>
{
  pub async fn recv(self) -> Result<T, oneshot::error::RecvError>
  {
    self.0.await
  }

  pub async fn close(mut self)
  {
    self.0.close()
  }
}

impl<T> IpcSender<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub fn send(
    &self,
    data: T,
  )
  {
    self.sender.send(data)
  }
}

impl<T> IpcReceiver<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub fn recv(&self) -> Option<T>
  {
    self.receiver.recv()
  }
}

impl OpaqueSender
{
  pub fn send<T>(
    &self,
    val: T,
  ) where
    T: for<'de> Deserialize<'de> + Serialize,
  {
    let mut cell = self.0.lock().unwrap();

    let sender1 = mem::take(cell.deref_mut()).unwrap();

    let sender2 = sender1.to();

    sender2.send(val).unwrap();

    let _ = mem::replace(cell.deref_mut(), Some(sender2.to_opaque()));
  }
}

impl OpaqueReceiver
{
  pub fn recv<T>(&self) -> Option<T>
  where
    T: for<'de> Deserialize<'de> + Serialize,
  {
    let mut cell = self.0.lock().unwrap();

    let receiver1 = mem::take(cell.deref_mut()).unwrap();

    let receiver2 = receiver1.to();

    let val = receiver2.recv().ok();

    let _ = mem::replace(cell.deref_mut(), Some(receiver2.to_opaque()));

    val
  }
}
