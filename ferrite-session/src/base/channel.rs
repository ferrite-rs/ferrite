use async_std::{channel, task};
use ipc_channel::ipc;
use serde::{ser, Serialize, Deserialize, Serializer, Deserializer};

pub struct Payload<T>(pub T);

pub struct Forward<T>(pub T);

pub struct Sender<T>(pub channel::Sender<T>);

pub struct Receiver<T>(pub channel::Receiver<T>);

pub struct SenderOnce<T>(channel::Sender<T>);

pub struct ReceiverOnce<T>(channel::Receiver<T>);

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

pub fn once_channel<T>() -> (SenderOnce<T>, ReceiverOnce<T>)
{
  let (sender, receiver) = channel::bounded(1);
  (SenderOnce(sender), ReceiverOnce(receiver))
}

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

impl <T> SenderOnce <T> {
  pub async fn send (self, msg: T)
    -> Result<(), channel::SendError<T>>
  {
    self.0.send(msg).await
  }

  pub fn close(self) -> bool
  {
    self.0.close()
  }
}

impl <T> ReceiverOnce <T> {
  pub async fn recv(self)
    -> Result<T, channel::RecvError>
  {
    self.0.recv().await
  }

  pub fn close(self) -> bool
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

impl <T, C> ForwardChannel
  for SenderOnce < (Payload<T>, C) >
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
    let receiver2: ipc::IpcReceiver<T> = receiver1.to();

    task::spawn_blocking(move || {
      let res = receiver2.recv();
      match res {
        Ok(payload) => {
          let channel = C::forward_from(sender1, receiver2.to_opaque());
          task::spawn(async move {
            self.send((Payload(payload), channel)).await.unwrap()
          });
        },
        Err(_) => ()
      }
    });
  }

  fn forward_from(
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();
    let sender3: ipc::IpcSender<T> = sender1.to();

    task::spawn(async move {
      let (Payload(payload), channel): (Payload<T>, C) =
        receiver2.recv().await.unwrap();

      task::spawn_blocking(move || {
        sender3.send(payload).unwrap();
        channel.forward_to(sender3.to_opaque(), receiver1);
      });
    });

    sender2
  }
}

impl < T, C > ForwardChannel
  for ReceiverOnce < ( Payload<T>, C ) >
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
    let sender2: ipc::IpcSender<T> = sender1.to();

    task::spawn(async move {
      let (Payload(payload), channel) = self.recv().await.unwrap();
      task::spawn_blocking(move || {
        sender2.send(payload).unwrap();
        channel.forward_to(sender2.to_opaque(), receiver1)
      });
    });
  }

  fn forward_from(
    sender1: ipc::OpaqueIpcSender,
    receiver1: ipc::OpaqueIpcReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();
    let receiver3: ipc::IpcReceiver<T> = receiver1.to();

    task::spawn_blocking(move || {
      let payload = receiver3.recv().unwrap();
      let channel = C::forward_from(sender1, receiver3.to_opaque());
      task::spawn(async move {
        sender2.send((Payload(payload), channel)).await.unwrap();
      });
    });

    receiver2
  }
}

impl < T > Serialize
  for Sender < Forward<T> >
where
  T: ForwardChannel,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let sender = self.0.clone();

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
            task::spawn(async move {
              sender.send(Forward(payload)).await.unwrap();
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
  for Sender < Forward<T> >
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
        match task::block_on(receiver1.recv()) {
          Ok(Forward(payload)) => {
            let channel = serialize_channel(payload);
            ipc_sender.send(channel).unwrap();
          },
          Err(_) => break
        }
      }
    });

    Ok(sender1)
  }
}


// impl < T > Serialize
//   for Receiver < Forward<T> >
// where
//   T: ForwardChannel
// {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     let receiver = self.0.clone();

//     let (ipc_sender, ipc_receiver) =
//       ipc::channel::<
//         (ipc::OpaqueIpcSender, ipc::OpaqueIpcReceiver)
//       > ()
//       .map_err(|err| ser::Error::custom(format!(
//         "Failed to create IPC channel: {}", err)))?;

//     task::spawn_blocking(move || {
//       loop {
//         match task::block_on(receiver.recv()) {
//           Ok(Forward(payload)) => {
//             let channel = serialize_channel(payload);
//             ipc_sender.send(channel).unwrap();
//           },
//           Err(_) => break
//         }
//       }
//     });

//     ipc_receiver.serialize(serializer)
//   }
// }

// impl < 'a, T > Deserialize <'a>
//   for Receiver < Forward<T> >
// where
//   T: ForwardChannel
// {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'a>
//   {
//     let ipc_receiver =
//       < ipc::IpcReceiver <
//           (ipc::OpaqueIpcSender, ipc::OpaqueIpcReceiver)
//         >
//       >::deserialize(deserializer)?;

//     let (sender1, receiver1) = unbounded();

//     task::spawn_blocking(move || {
//       loop {
//         let res = ipc_receiver.recv();
//         match res {
//           Ok((sender2, receiver2)) => {
//             let payload = T::forward_from(sender2, receiver2);
//             task::spawn(async move {
//               sender1.send(Forward(payload)).await.unwrap();
//             });
//           },
//           Err(_) => break
//         }
//       }
//     });

//     Ok(receiver1)
//   }
// }

// impl < T > Serialize
//   for Receiver < T >
// where
//   T: Send + 'static,
//   T: Serialize + for<'de> Deserialize<'de>,
// {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     let receiver = self.0.clone();

//     let (ipc_sender, ipc_receiver) = ipc::channel::<T>()
//       .map_err(|err| ser::Error::custom(format!(
//         "Failed to create IPC channel: {}", err)))?;

//     task::spawn_blocking(move || {
//       loop {
//         match task::block_on(receiver.recv()) {
//           Ok(x) => {
//             ipc_sender.send(x).unwrap();
//           },
//           Err(_) => break
//         }
//       }
//     });

//     ipc_receiver.serialize(serializer)
//   }
// }

// impl < 'a, T > Deserialize <'a>
//   for Receiver < T >
// where
//   T: Send + 'static,
//   T: Serialize + for<'de> Deserialize<'de>,
// {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'a>
//   {
//     let ipc_receiver =
//       < ipc::IpcReceiver<T> >::deserialize(deserializer)?;

//     let (sender, receiver) = channel::unbounded::<T>();

//     task::spawn_blocking(move || {
//       loop {
//         let res = ipc_receiver.recv();
//         match res {
//           Ok(x) => {
//             let sender = sender.clone();
//             task::block_on(async move {
//               sender.send(x).await.unwrap()
//             });
//           },
//           Err(_) => break
//         }
//       }
//     });

//     Ok(Receiver(receiver))
//   }
// }

// impl < T > Serialize
//   for Sender < T >
// where
//   T: Send + 'static,
//   T: Serialize + for<'de> Deserialize<'de>,
// {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     let sender = self.0.clone();

//     let (ipc_sender, ipc_receiver) = ipc::channel::<T>()
//       .map_err(|err| ser::Error::custom(format!(
//         "Failed to create IPC channel: {}", err)))?;

//     task::spawn_blocking(move || {
//       loop {
//         let res = ipc_receiver.recv();
//         match res {
//           Ok(x) => {
//             let sender = sender.clone();
//             task::block_on(async move {
//               sender.send(x).await.unwrap()
//             });
//           },
//           Err(_) => break
//         }
//       }
//     });

//     ipc_sender.serialize(serializer)
//   }
// }

// impl < 'a, T > Deserialize <'a>
//   for Sender < T >
// where
//   T: Send + 'static,
//   T: Serialize + for<'de> Deserialize<'de>,
// {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'a>
//   {
//     let ipc_sender =
//       < ipc::IpcSender<T> >::deserialize(deserializer)?;

//     let (sender, receiver) = channel::unbounded::<T>();

//     task::spawn_blocking(move || {
//       loop {
//         match task::block_on(receiver.recv()) {
//           Ok(x) => {
//             ipc_sender.send(x).unwrap();
//           },
//           Err(_) => break
//         }
//       }
//     });

//     Ok(Sender(sender))
//   }
// }

// impl < T > Serialize
//   for ReceiverOnce < T >
// where
//   T: Send + 'static,
//   T: Serialize + for<'de> Deserialize<'de>,
// {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     // debug!("Serializing {}", type_name::<Self>());
//     let receiver = self.0.clone();

//     let (ipc_sender, ipc_receiver) = ipc::channel::<T>()
//       .map_err(|err| ser::Error::custom(format!(
//         "Failed to create IPC channel: {}", err)))?;

//     task::spawn_blocking(move || {
//       match task::block_on(receiver.recv()) {
//         Ok(x) => {
//           // debug!("[SerializeReceiverOnce] forwarding message from {} to {} for serialized {}",
//           //   type_name::<ReceiverOnce<T>>(),
//           //   type_name::<ipc::IpcSender<T>>(),
//           //   type_name::<ReceiverOnce<T>>());

//           ipc_sender.send(x).unwrap();
//         },
//         Err(_) => ()
//       }
//       debug!("Ending Serialize forwarding for {}", type_name::<Self>());
//     });

//     ipc_receiver.serialize(serializer)
//   }
// }

// impl < 'a, T > Deserialize <'a>
//   for ReceiverOnce < T >
// where
//   T: Send + 'static,
//   T: Serialize + for<'de> Deserialize<'de>,
// {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'a>
//   {
//     // debug!("Deserializing {}", type_name::<Self>());

//     let ipc_receiver =
//       < ipc::IpcReceiver<T> >::deserialize(deserializer)?;

//     let (sender, receiver) = channel::unbounded::<T>();

//     task::spawn_blocking(move || {
//       let res = ipc_receiver.recv();
//       match res {
//         Ok(x) => {
//           // debug!("[DeserializeReceiverOnce] forwarding message from {} to {} for deserialized {}",
//           //   type_name::<ipc::IpcReceiver<T>>(),
//           //   type_name::<Sender<T>>(),
//           //   type_name::<ReceiverOnce<T>>());

//           let sender = sender.clone();
//           task::block_on(async move {
//             sender.send(x).await.unwrap()
//           });
//         },
//         Err(_) => ()
//       }
//       debug!("Ending Deserialize forwarding for {}", type_name::<Self>());
//     });

//     Ok(ReceiverOnce(receiver))
//   }
// }

// impl < T > Serialize
//   for SenderOnce < T >
// where
//   T: Send + 'static,
//   T: Serialize + for<'de> Deserialize<'de>,
// {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     // debug!("Serializing {}", type_name::<Self>());

//     let sender = self.0.clone();

//     let (ipc_sender, ipc_receiver) = ipc::channel::<T>()
//       .map_err(|err| ser::Error::custom(format!(
//         "Failed to create IPC channel: {}", err)))?;

//     task::spawn_blocking(move || {
//       let res = ipc_receiver.recv();
//       match res {
//         Ok(x) => {
//           // debug!("[SerializeSenderOnce] forwarding message from {} to {} for serialized {}",
//           //   type_name::<ipc::IpcReceiver<T>>(),
//           //   type_name::<SenderOnce<T>>(),
//           //   type_name::<SenderOnce<T>>());

//           let sender = sender.clone();
//           task::block_on(async move {
//             sender.send(x).await.unwrap()
//           });
//         },
//         Err(_) => ()
//       }
//       debug!("Ending Serialize forwarding for {}", type_name::<Self>());
//     });

//     ipc_sender.serialize(serializer)
//   }
// }

// impl < 'a, T > Deserialize <'a>
//   for SenderOnce < T >
// where
//   T: Send + 'static,
//   T: Serialize + for<'de> Deserialize<'de>,
// {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'a>
//   {
//     // debug!("Deserializing {}", type_name::<Self>());
//     let ipc_sender =
//       < ipc::IpcSender<T> >::deserialize(deserializer)?;

//     let (sender, receiver) = channel::unbounded::<T>();

//     task::spawn_blocking(move || {
//       match task::block_on(receiver.recv()) {
//         Ok(x) => {
//           // debug!("[DeserializeSenderOnce] forwarding message from {} to {} for deserialized {}",
//           //   type_name::<ReceiverOnce<T>>(),
//           //   type_name::<ipc::IpcSender<T>>(),
//           //   type_name::<SenderOnce<T>>());

//           ipc_sender.send(x).unwrap();
//         },
//         Err(_) => ()
//       }
//       debug!("Ending Deserialize forwarding for {}", type_name::<Self>());
//     });

//     Ok(SenderOnce(sender))
//   }
// }


// impl <T> ForwardChannel
//   for SenderOnce <T>
// where
//   T: ForwardChannel + Send + 'static
// {
//   fn forward_to(self,
//     sender1: ipc::OpaqueIpcSender,
//     receiver1: ipc::OpaqueIpcReceiver,
//   ) {
//     let receiver2: ipc::IpcReceiver<()> = receiver1.to();

//     task::spawn_blocking(move || {
//       receiver2.recv().unwrap();
//       let payload = T::forward_from(sender1, receiver2.to_opaque());

//       task::block_on(async move {
//         self.send(payload).await.unwrap();
//       });
//     });
//   }

//   fn forward_from(
//     sender1: ipc::OpaqueIpcSender,
//     receiver1: ipc::OpaqueIpcReceiver,
//   ) -> Self
//   {
//     let (sender2, receiver2) = once_channel();
//     let sender3: ipc::IpcSender<()> = sender1.to();

//     task::spawn(async move {
//       let payload: T = receiver2.recv().await.unwrap();
//       task::spawn_blocking(move || {
//         sender3.send(()).unwrap();
//         payload.forward_to(sender3.to_opaque(), receiver1);
//       });
//     });

//     sender2
//   }
// }

// impl <T> ForwardChannel
//   for ReceiverOnce <T>
// where
//   T: ForwardChannel + Send + 'static
// {
//   fn forward_to(self,
//     sender1: ipc::OpaqueIpcSender,
//     receiver1: ipc::OpaqueIpcReceiver,
//   ) {
//     let sender2: ipc::IpcSender<()> = sender1.to();

//     task::spawn(async move {
//       let channel = self.recv().await.unwrap();

//       task::spawn_blocking(move || {
//         sender2.send(()).unwrap();
//         channel.forward_to(sender2.to_opaque(), receiver1);
//       });
//     });
//   }

//   fn forward_from(
//     sender1: ipc::OpaqueIpcSender,
//     receiver1: ipc::OpaqueIpcReceiver,
//   ) -> Self
//   {
//     let (sender2, receiver2) = once_channel();
//     let receiver3: ipc::IpcReceiver<()> = receiver1.to();

//     task::spawn_blocking(move || {
//       receiver3.recv().unwrap();
//       let channel = T::forward_from(sender1, receiver3.to_opaque());
//       task::spawn(async move {
//         sender2.send(channel).await.unwrap();
//       });
//     });

//     receiver2
//   }
// }

// impl < T > ForwardChannel
//   for Sender < T >
// where
//   T: ForwardChannel
// {
//   fn forward_to(self,
//     sender1: ipc::OpaqueIpcSender,
//     receiver1: ipc::OpaqueIpcReceiver,
//   )
//   {
//     let receiver2
//       : ipc::IpcReceiver <
//           ( ipc::OpaqueIpcSender,
//             ipc::OpaqueIpcReceiver
//           ) >
//       = receiver1.to();

//     task::spawn_blocking(move || {
//       loop {
//         let res = receiver2.recv();
//         match res {
//           Ok((sender3, receiver3)) => {
//             let payload = T::forward_from(sender3, receiver3);
//             task::spawn(async move {
//               self.send(payload).await.unwrap();
//             });
//           },
//           Err(_) => break
//         }
//       }
//     });
//   }

//   fn forward_from(
//     sender1: ipc::OpaqueIpcSender,
//     receiver1: ipc::OpaqueIpcReceiver,
//   ) -> Self
//   {
//     let (sender2, receiver2) = unbounded::<T>();

//     let sender3
//       : ipc::IpcSender <
//           ( ipc::OpaqueIpcSender,
//             ipc::OpaqueIpcReceiver
//           ) >
//       = sender1.to();

//     task::spawn(async move {
//       loop {
//         let res = receiver2.recv().await;
//         match res {
//           Ok(payload) => {
//             task::spawn_blocking(move || {
//               let (sender4, receiver4) = ipc::channel::<()>().unwrap();
//               let (sender5, receiver5) = ipc::channel::<()>().unwrap();

//               sender3.send(
//                 (sender4.to_opaque(), receiver5.to_opaque())
//               ).unwrap();

//               payload.forward_to(
//                 sender5.to_opaque(), receiver4.to_opaque()
//               );
//             });
//           },
//           Err(_) => break
//         }
//       }
//     });

//     sender2
//   }
// }
