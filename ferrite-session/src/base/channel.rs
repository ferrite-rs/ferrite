use serde;
use std::mem;
use std::future::Future;
use ipc_channel::ipc;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tokio::{ task, runtime, sync::{mpsc, oneshot, Mutex as AsyncMutex} };
use serde::{ser, Serialize, Deserialize, Serializer, Deserializer};

use crate::functional::*;

pub struct Value<T>(pub T);

pub struct Sender<T>(pub mpsc::UnboundedSender<T>);

pub struct Receiver<T>(
  pub Arc<AsyncMutex<mpsc::UnboundedReceiver<T>>>);

pub struct SenderOnce<T>(oneshot::Sender<T>);

pub struct ReceiverOnce<T>(oneshot::Receiver<T>);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OpaqueReceiver(
  Arc < Mutex <
    Option<ipc::OpaqueIpcReceiver>
  > >
);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OpaqueSender(
  Arc < Mutex <
    Option<ipc::OpaqueIpcSender>
  > >
);

#[derive(Debug)]
pub struct SendError(pub String);

pub trait ForwardChannel: Send + 'static {
  fn forward_to(self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  );

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self;
}

lazy_static! {
  pub static ref RUNTIME : runtime::Runtime =
    runtime::Builder::new_multi_thread()
      .worker_threads(16)
      .max_blocking_threads(1024)
      .build()
      .unwrap();
}

pub fn spawn<T>(task: T) ->
  task::JoinHandle<T::Output>
where
  T: Future + Send + 'static,
  T::Output: Send + 'static,
{
  RUNTIME.spawn(task)
}

pub fn spawn_blocking<F, R>(f: F) ->
  task::JoinHandle<R>
where
  F: FnOnce() -> R + Send + 'static,
  R: Send + 'static,
{
  RUNTIME.spawn_blocking(f)
}

pub fn once_channel<T>() -> (SenderOnce<T>, ReceiverOnce<T>)
{
  let (sender, receiver) = oneshot::channel();
  (SenderOnce(sender), ReceiverOnce(receiver))
}

pub fn unbounded<T>()
  -> (Sender<T>, Receiver<T>)
{
  let (sender, receiver) = mpsc::unbounded_channel();
  (Sender(sender), Receiver(Arc::new(AsyncMutex::new(receiver))))
}

impl OpaqueSender {
  pub fn send <T> (&self, val: T)
  where
    T: for <'de> Deserialize<'de> + Serialize
  {
    let mut cell = self.0.lock().unwrap();
    let sender1 = mem::take(cell.deref_mut()).unwrap();
    let sender2 = sender1.to();
    sender2.send(val).unwrap();
    let _ = mem::replace(cell.deref_mut(), Some(sender2.to_opaque()));
  }
}

impl OpaqueReceiver {
  pub fn recv <T> (&self) -> Option<T>
  where
    T: for <'de> Deserialize<'de> + Serialize
  {
    let mut cell = self.0.lock().unwrap();
    let receiver1 = mem::take(cell.deref_mut()).unwrap();
    let receiver2 = receiver1.to();
    let val = receiver2.recv().ok();
    let _ = mem::replace(cell.deref_mut(), Some(receiver2.to_opaque()));
    val
  }
}

pub fn opaque_channel() -> (OpaqueSender, OpaqueReceiver)
{
  let (sender, receiver) = ipc::channel::<()>().unwrap();

  ( OpaqueSender(Arc::new(Mutex::new(Some(sender.to_opaque())))),
    OpaqueReceiver(Arc::new(Mutex::new(Some(receiver.to_opaque()))))
  )
}

pub fn serialize_channel <T>
  (payload: T)
  -> (OpaqueSender, OpaqueReceiver)
where
  T: ForwardChannel
{
  let (sender1, receiver1) = opaque_channel();
  let (sender2, receiver2) = opaque_channel();

  payload.forward_to(sender1, receiver2);

  (sender2, receiver1)
}

impl <T> Clone for Sender<T> {
  fn clone(&self) -> Sender<T> {
    Sender(self.0.clone())
  }
}

impl <T> Clone for Receiver<T> {
  fn clone(&self) -> Receiver<T> {
    Receiver(self.0.clone())
  }
}

impl <T> Sender <T> {
  pub async fn send (&self, msg: T)
    -> Result<(), SendError>
  {
    self.0.send(msg)
      .map_err(|_| SendError(String::from("failed to send")))
  }

  pub async fn close(&self)
  {
    self.0.closed().await
  }
}

impl <T> Receiver <T> {
  pub async fn recv(&self)
    -> Option<T>
  {
    self.0.lock().await.recv().await
  }

  pub async fn close(&self)
  {
    self.0.lock().await.close()
  }
}

impl <T> SenderOnce <T> {
  pub async fn send (self, msg: T)
    -> Result< (), SendError >
  {
    self.0.send(msg)
      .map_err(|_| SendError(String::from("failed to send")))
  }

  pub async fn close(mut self)
  {
    self.0.closed().await
  }
}

impl <T> ReceiverOnce <T> {
  pub async fn recv(self)
    -> Result<T, oneshot::error::RecvError>
  {
    self.0.await
  }

  pub async fn close(mut self)
  {
    self.0.close()
  }
}

impl ForwardChannel
  for ()
{
  fn forward_to(self,
    _: OpaqueSender,
    _: OpaqueReceiver,
  )
  { }

  fn forward_from(
    _: OpaqueSender,
    _: OpaqueReceiver,
  ) -> Self
  { }
}

impl <T> ForwardChannel
  for SenderOnce <T>
where
  T: ForwardChannel
{
  fn forward_to(self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    RUNTIME.spawn_blocking(move || {
      receiver.recv::<()>().unwrap();
      let payload = T::forward_from(sender, receiver);

      RUNTIME.spawn(async move {
        self.send(payload).await.unwrap();
      });
    });
  }

  fn forward_from(
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();

    RUNTIME.spawn(async move {
      let payload: T = receiver2.recv().await.unwrap();
      RUNTIME.spawn_blocking(move || {
        sender1.send(());
        payload.forward_to(sender1, receiver1);
      });
    });

    sender2
  }
}

impl <T> ForwardChannel
  for ReceiverOnce <T>
where
  T: ForwardChannel
{
  fn forward_to(self,
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  ) {
    RUNTIME.spawn(async move {
      let channel = self.recv().await.unwrap();

      spawn_blocking(move || {
        sender1.send(());
        channel.forward_to(sender1, receiver1);
      });
    });
  }

  fn forward_from(
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();

    RUNTIME.spawn_blocking(move || {
      receiver1.recv::<()>().unwrap();
      let channel = T::forward_from(sender1, receiver1);
      spawn(async move {
        sender2.send(channel).await.unwrap();
      });
    });

    receiver2
  }
}


impl < T, C > ForwardChannel
  for ( Value<T>, C )
where
  T: Send + 'static,
  T: Serialize + for<'de> Deserialize<'de>,
  C: ForwardChannel,
{
  fn forward_to(self,
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  )
  {
    let (Value(payload), channel) = self;

    RUNTIME.spawn_blocking(move || {
      sender1.send(payload);
      channel.forward_to(sender1, receiver1)
    });
  }

  fn forward_from(
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  ) -> Self
  {
    let payload = receiver1.recv().unwrap();
    let channel = C::forward_from(sender1, receiver1);

    (Value(payload), channel)
  }
}

impl < T > Serialize
  for Sender < T >
where
  T: ForwardChannel,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let sender = self.clone();

    let (sender1, receiver1) =
      ipc::channel::<
        (OpaqueSender, OpaqueReceiver)
      > ()
      .map_err(|err| ser::Error::custom(format!(
        "Failed to create IPC channel: {}", err)))?;

    RUNTIME.spawn_blocking(move || {
      loop {
        let res = receiver1.recv();
        match res {
          Ok((sender2, receiver2)) => {
            let payload = T::forward_from(sender2, receiver2);
            let sender3 = sender.clone();
            spawn(async move {
              sender3.send(payload).await.unwrap();
            });
          },
          Err(_) => break
        }
      }
    });

    sender1.serialize(serializer)
  }
}

impl < 'a, T > Deserialize <'a>
  for Sender < T >
where
  T: ForwardChannel,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'a>
  {
    let ipc_sender =
      < ipc::IpcSender <
          (OpaqueSender, OpaqueReceiver)
        >
      >::deserialize(deserializer)?;

    let (sender1, receiver1) = unbounded();

    spawn_blocking(move || {
      loop {
        match RUNTIME.block_on(receiver1.recv()) {
          Some(payload) => {
            let channel = serialize_channel(payload);
            ipc_sender.send(channel).unwrap();
          },
          None => break
        }
      }
    });

    Ok(sender1)
  }
}

impl < F, X, T >
  ForwardChannel
  for Applied < F, X >
where
  X: Send + 'static,
  F: TypeApp < X, Applied = T >,
  T: ForwardChannel,
{
  fn forward_to(self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    self.get_applied().forward_to(sender, receiver)
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    cloak_applied(
      T::forward_from(sender, receiver)
    )
  }
}

impl < Row, F, T >
  ForwardChannel
  for AppliedSum < Row, F >
where
  F: TyCon,
  F: Send + 'static,
  Row: RowApp < F, Applied = T >,
  T: ForwardChannel,
{
  fn forward_to(self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    self.get_row().forward_to(sender, receiver)
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    cloak_row(
      T::forward_from(sender, receiver)
    )
  }
}

impl < A, B >
  ForwardChannel
  for Sum < A, B >
where
  A: ForwardChannel,
  B: ForwardChannel,
{
  fn forward_to(self,
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
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
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  ) -> Self
  {
    if receiver1.recv().unwrap() {
      Sum::Inl(
        A::forward_from(sender1, receiver1))
    } else {
      Sum::Inr(
        B::forward_from(sender1, receiver1))
    }
  }
}

impl ForwardChannel for Bottom
{
  fn forward_to(self,
    _: OpaqueSender,
    _: OpaqueReceiver,
  )
  {
    match self {}
  }

  fn forward_from(
    _: OpaqueSender,
    receiver1: OpaqueReceiver,
  ) -> Self
  {
    receiver1.recv().unwrap()
  }
}

impl < T > Serialize
  for Receiver < T >
where
  T: ForwardChannel
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let receiver = self.clone();

    let (ipc_sender, ipc_receiver) =
      ipc::channel::<
        (OpaqueSender, OpaqueReceiver)
      > ()
      .map_err(|err| ser::Error::custom(format!(
        "Failed to create IPC channel: {}", err)))?;

    spawn_blocking(move || {
      loop {
        match RUNTIME.block_on(receiver.recv()) {
          Some(payload) => {
            let channel = serialize_channel(payload);
            ipc_sender.send(channel).unwrap();
          },
          None => break
        }
      }
    });

    ipc_receiver.serialize(serializer)
  }
}

impl < 'a, T > Deserialize <'a>
  for Receiver < T >
where
  T: ForwardChannel
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'a>
  {
    let ipc_receiver =
      < ipc::IpcReceiver <
          (OpaqueSender, OpaqueReceiver)
        >
      >::deserialize(deserializer)?;

    let (sender1, receiver1) = unbounded();

    spawn_blocking(move || {
      loop {
        let res = ipc_receiver.recv();
        match res {
          Ok((sender2, receiver2)) => {
            let payload = T::forward_from(sender2, receiver2);
            let sender3 = sender1.clone();
            spawn(async move {
              sender3.send(payload).await.unwrap();
            });
          },
          Err(_) => break
        }
      }
    });

    Ok(receiver1)
  }
}
