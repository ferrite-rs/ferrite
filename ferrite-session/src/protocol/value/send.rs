use serde;

use crate::base::*;

pub struct SendValue < T, A >
(
  pub (crate) T,
  pub (crate) ReceiverOnce < A >
);

impl < T, P > Protocol for SendValue < T, P >
where
  T : Send + 'static,
  P : Protocol
{ }

impl < X, T, A >
  RecApp < X > for
  SendValue < T, A >
where
  T : Send + 'static,
  A : RecApp < X >,
{
  type Applied =
    SendValue <
      T,
      A :: Applied
    >;
}

impl < T, A > serde::Serialize
  for SendValue < T, A >
where
  A: Send + 'static,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
  A: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    (&self.0, &self.1).serialize(serializer)
  }
}

impl < 'a, T, A > serde::Deserialize <'a>
  for SendValue < T, A >
where
  A: Send + 'static,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
  A: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>
  {
    let (val, receiver) =
      < (T, ReceiverOnce<A>) >::deserialize(deserializer)?;

    Ok(SendValue(val, receiver))

    // let (val, ipc_receiver) =
    //   < (T, IpcReceiverOnce<A>) >::deserialize(deserializer)?;

    // let (sender, receiver) = async_std::sync::channel::<A>(1);

    // task::spawn(async move {
    //   loop {
    //     let res = ipc_receiver.recv();
    //     match res {
    //       Ok(x) => sender.send(x).await,
    //       Err(_) => return
    //     }
    //   }
    // });

    // Ok(SendValue(val, receiver))
  }
}
