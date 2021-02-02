use ipc_channel::ipc;
use std::sync::Arc;
use lazy_static::lazy_static;
use tokio::{ task, runtime, sync::{mpsc, oneshot, Mutex} };
use serde::{ser, Serialize, Deserialize, Serializer, Deserializer};

use crate::functional::*;

pub struct Value<T>(pub T);

pub struct Sender<T>(pub mpsc::UnboundedSender<T>);

pub struct Receiver<T>(
  pub Arc<Mutex<mpsc::UnboundedReceiver<T>>>);

pub struct SenderOnce<T>(oneshot::Sender<T>);

pub struct ReceiverOnce<T>(oneshot::Receiver<T>);

#[derive(Debug)]
pub struct SendError(pub String);

pub trait ForwardChannel: Send + 'static {
  fn forward_to(self,
    sender: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
  );

  fn forward_from(
    sender: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
  ) -> Self;
}

lazy_static! {
  static ref RUNTIME : runtime::Runtime = runtime::Runtime::new().unwrap();
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
  (Sender(sender), Receiver(Arc::new(Mutex::new(receiver))))
}

pub fn serialize_channel <T>
  (payload: T)
  -> (ipc::OpaqueIpcSender, ipc::OpaqueIpcReceiver)
where
  T: ForwardChannel
{
  let (sender1, receiver1) = ipc::channel::<()>().unwrap();
  let (sender2, receiver2) = ipc::channel::<()>().unwrap();

  payload.forward_to(
    sender1.to_opaque(), receiver2.to_opaque()
  );

  (sender2.to_opaque(), receiver1.to_opaque())
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
    _: ipc::OpaqueIpcSender,
    _: ipc::OpaqueIpcReceiver,
  )
  { }

  fn forward_from(
    _: ipc::OpaqueIpcSender,
    _: ipc::OpaqueIpcReceiver,
  ) -> Self
  { }
}

impl <T> ForwardChannel
  for SenderOnce <T>
where
  T: ForwardChannel
{
  fn forward_to(self,
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) {
    let receiver2: ipc::IpcReceiver<()> = receiver1.to();

    task::spawn_blocking(move || {
      receiver2.recv().unwrap();
      let payload = T::forward_from(sender1, receiver2.to_opaque());

      RUNTIME.block_on(async move {
        self.send(payload).await.unwrap();
      });
    });
  }

  fn forward_from(
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();
    let sender3: ipc::IpcSender<()> = sender1.to();

    task::spawn(async move {
      let payload: T = receiver2.recv().await.unwrap();
      task::spawn_blocking(move || {
        sender3.send(()).unwrap();
        payload.forward_to(sender3.to_opaque(), receiver1);
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
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) {
    let sender2: ipc::IpcSender<()> = sender1.to();

    task::spawn(async move {
      let channel = self.recv().await.unwrap();

      task::spawn_blocking(move || {
        sender2.send(()).unwrap();
        channel.forward_to(sender2.to_opaque(), receiver1);
      });
    });
  }

  fn forward_from(
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();
    let receiver3: ipc::IpcReceiver<()> = receiver1.to();

    task::spawn_blocking(move || {
      receiver3.recv().unwrap();
      let channel = T::forward_from(sender1, receiver3.to_opaque());
      task::spawn(async move {
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
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  )
  {
    let (Value(payload), channel) = self;
    let sender2: ipc::IpcSender<T> = sender1.to();

    task::spawn_blocking(move || {
      sender2.send(payload).unwrap();
      channel.forward_to(sender2.to_opaque(), receiver1)
    });
  }

  fn forward_from(
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    let receiver2: ipc::IpcReceiver<T> = receiver1.to();
    let payload = receiver2.recv().unwrap();
    let channel = C::forward_from(sender1, receiver2.to_opaque());

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
        (ipc::OpaqueIpcSender, ipc::OpaqueIpcReceiver)
      > ()
      .map_err(|err| ser::Error::custom(format!(
        "Failed to create IPC channel: {}", err)))?;

    task::spawn_blocking(move || {
      loop {
        let res = receiver1.recv();
        match res {
          Ok((sender2, receiver2)) => {
            let payload = T::forward_from(sender2, receiver2);
            let sender3 = sender.clone();
            task::spawn(async move {
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
          (ipc::OpaqueIpcSender, ipc::OpaqueIpcReceiver)
        >
      >::deserialize(deserializer)?;

    let (sender1, receiver1) = unbounded();

    task::spawn_blocking(move || {
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
    sender: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
  )
  {
    self.get_applied().forward_to(sender, receiver)
  }

  fn forward_from(
    sender: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
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
    sender: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
  )
  {
    self.get_row().forward_to(sender, receiver)
  }

  fn forward_from(
    sender: ipc::OpaqueIpcSender,
    receiver: ipc::OpaqueIpcReceiver,
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
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  )
  {
    let sender2: ipc::IpcSender < bool > = sender1.to();
    match self {
      Sum::Inl(a) => {
        sender2.send(true).unwrap();
        a.forward_to(sender2.to_opaque(), receiver1)
      }
      Sum::Inr(b) => {
        sender2.send(false).unwrap();
        b.forward_to(sender2.to_opaque(), receiver1)
      }
    }
  }

  fn forward_from(
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    let receiver2: ipc::IpcReceiver < bool > = receiver1.to();

    if receiver2.recv().unwrap() {
      Sum::Inl(
        A::forward_from(sender1, receiver2.to_opaque()))
    } else {
      Sum::Inr(
        B::forward_from(sender1, receiver2.to_opaque()))
    }
  }
}

impl ForwardChannel for Bottom
{
  fn forward_to(self,
    _: ipc::OpaqueIpcSender,
    _: ipc::OpaqueIpcReceiver,
  )
  {
    match self {}
  }

  fn forward_from(
    _: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    receiver1.to().recv().unwrap()
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
        (ipc::OpaqueIpcSender, ipc::OpaqueIpcReceiver)
      > ()
      .map_err(|err| ser::Error::custom(format!(
        "Failed to create IPC channel: {}", err)))?;

    task::spawn_blocking(move || {
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
          (ipc::OpaqueIpcSender, ipc::OpaqueIpcReceiver)
        >
      >::deserialize(deserializer)?;

    let (sender1, receiver1) = unbounded();

    task::spawn_blocking(move || {
      loop {
        let res = ipc_receiver.recv();
        match res {
          Ok((sender2, receiver2)) => {
            let payload = T::forward_from(sender2, receiver2);
            let sender3 = sender1.clone();
            task::spawn(async move {
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
