use std::{
  marker::PhantomData,
  mem,
  ops::DerefMut,
  sync::{
    Arc,
    Mutex,
  },
};

use ipc_channel::ipc;
use serde::{
  self,
  Deserialize,
  Serialize,
};
use tokio::{
  sync::{
    mpsc,
    oneshot,
    Mutex as AsyncMutex,
  },
  task,
};

use crate::internal::functional::*;

pub struct ReceiverF {}

pub struct SenderF {}

pub struct Value<T>(pub T);

pub struct Sender<T>(pub mpsc::UnboundedSender<T>);

pub struct Receiver<T>(pub Arc<AsyncMutex<mpsc::UnboundedReceiver<T>>>);

pub struct SenderOnce<T>(oneshot::Sender<T>);

pub struct ReceiverOnce<T>(oneshot::Receiver<T>);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OpaqueReceiver(Arc<Mutex<Option<ipc::OpaqueIpcReceiver>>>);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OpaqueSender(Arc<Mutex<Option<ipc::OpaqueIpcSender>>>);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct IpcSender<T>
{
  sender : OpaqueSender,
  phantom : PhantomData<T>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct IpcReceiver<T>
{
  receiver : OpaqueReceiver,
  phantom : PhantomData<T>,
}

// TODO: Define proper error type
#[derive(Debug)]
pub struct SendError(pub String);

pub trait ForwardChannel: Send + 'static
{
  fn forward_to(
    self,
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  );

  fn forward_from(
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  ) -> Self;
}

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

impl OpaqueSender
{
  pub fn send<T>(
    &self,
    val : T,
  ) where
    T : for<'de> Deserialize<'de> + Serialize,
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
    T : for<'de> Deserialize<'de> + Serialize,
  {
    let mut cell = self.0.lock().unwrap();

    let receiver1 = mem::take(cell.deref_mut()).unwrap();

    let receiver2 = receiver1.to();

    let val = receiver2.recv().ok();

    let _ = mem::replace(cell.deref_mut(), Some(receiver2.to_opaque()));

    val
  }
}

impl TyCon for ReceiverF {}

impl TyCon for SenderF {}

impl<P> TypeApp<P> for ReceiverF
where
  P : Send + 'static,
{
  type Applied = ReceiverOnce<P>;
}

impl<P> TypeApp<P> for SenderF
where
  P : Send + 'static,
{
  type Applied = SenderOnce<P>;
}

impl<T> IpcSender<T>
where
  T : for<'de> Deserialize<'de> + Serialize,
{
  pub fn send(
    &self,
    data : T,
  )
  {
    self.sender.send(data)
  }
}

impl<T> IpcReceiver<T>
where
  T : for<'de> Deserialize<'de> + Serialize,
{
  pub fn recv(&self) -> Option<T>
  {
    self.receiver.recv()
  }
}

pub fn ipc_channel<T>() -> (IpcSender<T>, IpcReceiver<T>)
where
  IpcReceiver<T> : Send,
{
  let (sender, receiver) = opaque_channel();

  (
    IpcSender {
      sender,
      phantom : PhantomData,
    },
    IpcReceiver {
      receiver,
      phantom : PhantomData,
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

impl<T> Clone for Sender<T>
{
  fn clone(&self) -> Sender<T>
  {
    Sender(self.0.clone())
  }
}

impl<T> Clone for Receiver<T>
{
  fn clone(&self) -> Receiver<T>
  {
    Receiver(self.0.clone())
  }
}

impl<T> Sender<T>
{
  pub fn send(
    &self,
    msg : T,
  ) -> Result<(), SendError>
  {
    self
      .0
      .send(msg)
      .map_err(|_| SendError(String::from("failed to send")))
  }
}

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
    msg : T,
  ) -> Result<(), SendError>
  {
    self
      .0
      .send(msg)
      .map_err(|_| SendError(String::from("failed to send")))
  }

  pub async fn close(mut self)
  {
    self.0.closed().await
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

impl ForwardChannel for ()
{
  fn forward_to(
    self,
    _ : OpaqueSender,
    _ : OpaqueReceiver,
  )
  {
  }

  fn forward_from(
    _ : OpaqueSender,
    _ : OpaqueReceiver,
  ) -> Self
  {
  }
}

impl<T> ForwardChannel for SenderOnce<T>
where
  T : ForwardChannel,
{
  fn forward_to(
    self,
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  )
  {
    task::spawn_blocking(move || {
      receiver.recv::<()>().unwrap();

      let payload = T::forward_from(sender, receiver);

      self.send(payload).unwrap();
    });
  }

  fn forward_from(
    sender1 : OpaqueSender,
    receiver1 : OpaqueReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();

    task::spawn(async move {
      let payload : T = receiver2.recv().await.unwrap();

      task::spawn_blocking(move || {
        sender1.send(());

        payload.forward_to(sender1, receiver1);
      });
    });

    sender2
  }
}

impl<T> ForwardChannel for ReceiverOnce<T>
where
  T : ForwardChannel,
{
  fn forward_to(
    self,
    sender1 : OpaqueSender,
    receiver1 : OpaqueReceiver,
  )
  {
    task::spawn(async move {
      let channel = self.recv().await.unwrap();

      task::spawn_blocking(move || {
        sender1.send(());

        channel.forward_to(sender1, receiver1);
      });
    });
  }

  fn forward_from(
    sender1 : OpaqueSender,
    receiver1 : OpaqueReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();

    task::spawn_blocking(move || {
      receiver1.recv::<()>().unwrap();

      let channel = T::forward_from(sender1, receiver1);

      sender2.send(channel).unwrap();
    });

    receiver2
  }
}

impl<T, C> ForwardChannel for (Value<T>, C)
where
  T : Send + 'static,
  T : Serialize + for<'de> Deserialize<'de>,
  C : ForwardChannel,
{
  fn forward_to(
    self,
    sender1 : OpaqueSender,
    receiver1 : OpaqueReceiver,
  )
  {
    let (Value(payload), channel) = self;

    task::spawn_blocking(move || {
      sender1.send(payload);

      channel.forward_to(sender1, receiver1)
    });
  }

  fn forward_from(
    sender1 : OpaqueSender,
    receiver1 : OpaqueReceiver,
  ) -> Self
  {
    let payload = receiver1.recv().unwrap();

    let channel = C::forward_from(sender1, receiver1);

    (Value(payload), channel)
  }
}

impl<F, X, T> ForwardChannel for Applied<F, X>
where
  X : Send + 'static,
  F : TypeApp<X, Applied = T>,
  T : ForwardChannel,
{
  fn forward_to(
    self,
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  )
  {
    self.get_applied().forward_to(sender, receiver)
  }

  fn forward_from(
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  ) -> Self
  {
    cloak_applied(T::forward_from(sender, receiver))
  }
}

impl<Row, F, T> ForwardChannel for AppliedSum<Row, F>
where
  F : TyCon,
  F : Send + 'static,
  Row : RowApp<F, Applied = T>,
  T : ForwardChannel,
{
  fn forward_to(
    self,
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  )
  {
    self.get_row().forward_to(sender, receiver)
  }

  fn forward_from(
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  ) -> Self
  {
    cloak_row(T::forward_from(sender, receiver))
  }
}

impl<A, B> ForwardChannel for Sum<A, B>
where
  A : ForwardChannel,
  B : ForwardChannel,
{
  fn forward_to(
    self,
    sender1 : OpaqueSender,
    receiver1 : OpaqueReceiver,
  )
  {
    match self {
      Sum::Inl(a) => {
        sender1.send(true);

        a.forward_to(sender1, receiver1)
      }
      Sum::Inr(b) => {
        sender1.send(false);

        b.forward_to(sender1, receiver1)
      }
    }
  }

  fn forward_from(
    sender1 : OpaqueSender,
    receiver1 : OpaqueReceiver,
  ) -> Self
  {
    if receiver1.recv().unwrap() {
      Sum::Inl(A::forward_from(sender1, receiver1))
    } else {
      Sum::Inr(B::forward_from(sender1, receiver1))
    }
  }
}

impl ForwardChannel for Bottom
{
  fn forward_to(
    self,
    _ : OpaqueSender,
    _ : OpaqueReceiver,
  )
  {
    match self {}
  }

  fn forward_from(
    _ : OpaqueSender,
    receiver1 : OpaqueReceiver,
  ) -> Self
  {
    receiver1.recv().unwrap()
  }
}
