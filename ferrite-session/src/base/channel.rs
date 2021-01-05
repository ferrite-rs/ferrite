use std::any::type_name;
use async_std::{channel, task};
use ipc_channel::ipc;
use log::debug;
use serde::{ser, Serialize, Deserialize, Serializer, Deserializer};

pub struct Sender<T>(pub channel::Sender<T>);

pub struct Receiver<T>(pub channel::Receiver<T>);

pub fn bounded<T>(cap: usize)
  -> (Sender<T>, Receiver<T>)
{
  let (sender, receiver) = channel::bounded(cap);
  (Sender(sender), Receiver(receiver))
}

pub fn unbounded<T>()
  -> (Sender<T>, Receiver<T>)
{
  let (sender, receiver) = channel::unbounded();
  (Sender(sender), Receiver(receiver))
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
    -> Result<(), channel::SendError<T>>
  {
    self.0.send(msg).await
  }

  pub fn close(&self) -> bool
  {
    self.0.close()
  }
}

impl <T> Receiver <T> {
  pub async fn recv(&self)
    -> Result<T, channel::RecvError>
  {
    self.0.recv().await
  }

  pub fn close(&self) -> bool
  {
    self.0.close()
  }
}

impl < T > Serialize
  for Receiver < T >
where
  T: Send + 'static,
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    debug!("Serializing {}", type_name::<Self>());
    let receiver = self.0.clone();

    let (ipc_sender, ipc_receiver) = ipc::channel::<T>()
      .map_err(|err| ser::Error::custom(format!(
        "Failed to create IPC channel: {}", err)))?;

    task::spawn(async move {
      loop {
        match receiver.recv().await {
          Ok(x) => {
            debug!("[SerializeReceiver] forwarding message from {} to {} for serialized {}",
              type_name::<Receiver<T>>(),
              type_name::<ipc::IpcSender<T>>(),
              type_name::<Receiver<T>>());

            let ipc_sender = ipc_sender.clone();
            task::spawn_blocking(move || {
              ipc_sender.send(x).unwrap()
            }).await;
          },
          Err(_) => break
        }
      }
      debug!("Ending Serialize forwarding for {}", type_name::<Self>());
    });

    ipc_receiver.serialize(serializer)
  }
}

impl < 'a, T > Deserialize <'a>
  for Receiver < T >
where
  T: Send + 'static,
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'a>
  {
    debug!("Deserializing {}", type_name::<Self>());

    let ipc_receiver =
      < ipc::IpcReceiver<T> >::deserialize(deserializer)?;

    let (sender, receiver) = channel::unbounded::<T>();

    task::spawn_blocking(move || {
      loop {
        let res = ipc_receiver.recv();
        match res {
          Ok(x) => {
            debug!("[DeserializeReceiver] forwarding message from {} to {} for deserialized {}",
              type_name::<ipc::IpcReceiver<T>>(),
              type_name::<Sender<T>>(),
              type_name::<Receiver<T>>());

            let sender = sender.clone();
            task::block_on(async move {
              sender.send(x).await.unwrap()
            });
          },
          Err(_) => break
        }
      }
      debug!("Ending Deserialize forwarding for {}", type_name::<Self>());
    });

    Ok(Receiver(receiver))
  }
}

impl < T > Serialize
  for Sender < T >
where
  T: Send + 'static,
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    debug!("Serializing {}", type_name::<Self>());

    let sender = self.0.clone();

    let (ipc_sender, ipc_receiver) = ipc::channel::<T>()
      .map_err(|err| ser::Error::custom(format!(
        "Failed to create IPC channel: {}", err)))?;

    task::spawn_blocking(move || {
      loop {
        let res = ipc_receiver.recv();
        match res {
          Ok(x) => {
            debug!("[SerializeSender] forwarding message from {} to {} for serialized {}",
              type_name::<ipc::IpcReceiver<T>>(),
              type_name::<Sender<T>>(),
              type_name::<Sender<T>>());

            let sender = sender.clone();
            task::block_on(async move {
              sender.send(x).await.unwrap()
            });
          },
          Err(_) => break
        }
      }
      debug!("Ending Serialize forwarding for {}", type_name::<Self>());
    });

    ipc_sender.serialize(serializer)
  }
}

impl < 'a, T > Deserialize <'a>
  for Sender < T >
where
  T: Send + 'static,
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'a>
  {
    debug!("Deserializing {}", type_name::<Self>());
    let ipc_sender =
      < ipc::IpcSender<T> >::deserialize(deserializer)?;

    let (sender, receiver) = channel::unbounded::<T>();

    task::spawn(async move {
      loop {
        let res = receiver.recv().await;
        match res {
          Ok(x) => {
            debug!("[DeserializeSender] forwarding message from {} to {} for deserialized {}",
              type_name::<Receiver<T>>(),
              type_name::<ipc::IpcSender<T>>(),
              type_name::<Sender<T>>());

            let ipc_sender = ipc_sender.clone();
            task::spawn_blocking(move || {
              ipc_sender.send(x).unwrap()
            }).await;
          },
          Err(_) => break
        }
      }
      debug!("Ending Deserialize forwarding for {}", type_name::<Self>());
    });

    Ok(Sender(sender))
  }
}
