use serde::{
  self,
  Deserialize,
  Serialize,
};
use tokio::task;

use super::{
  functions::*,
  traits::ForwardChannel,
  types::*,
};
use crate::internal::functional::*;

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
    msg: T,
  ) -> Result<(), SendError>
  {
    self
      .0
      .send(msg)
      .map_err(|_| SendError(String::from("failed to send")))
  }
}

impl ForwardChannel for ()
{
  fn forward_to(
    self,
    _: OpaqueSender,
    _: OpaqueReceiver,
  )
  {
  }

  fn forward_from(
    _: OpaqueSender,
    _: OpaqueReceiver,
  ) -> Self
  {
  }
}

impl<T> ForwardChannel for SenderOnce<T>
where
  T: ForwardChannel,
{
  fn forward_to(
    self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    task::spawn_blocking(move || {
      receiver.recv::<()>().unwrap();

      let payload = T::forward_from(sender, receiver);

      self.send(payload).unwrap();
    });
  }

  fn forward_from(
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  ) -> Self
  {
    let (sender2, receiver2) = once_channel();

    task::spawn(async move {
      let payload: T = receiver2.recv().await.unwrap();

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
  T: ForwardChannel,
{
  fn forward_to(
    self,
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
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
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
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

impl<T> ForwardChannel for Value<T>
where
  T: Send + 'static,
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn forward_to(
    self,
    sender1: OpaqueSender,
    _receiver1: OpaqueReceiver,
  )
  {
    let Value(payload) = self;

    sender1.send(payload);
  }

  fn forward_from(
    _sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  ) -> Self
  {
    let payload = receiver1.recv().unwrap();

    Value(payload)
  }
}

impl<T, C> ForwardChannel for (Value<T>, C)
where
  T: Send + 'static,
  T: Serialize + for<'de> Deserialize<'de>,
  C: ForwardChannel,
{
  fn forward_to(
    self,
    sender1: OpaqueSender,
    receiver1: OpaqueReceiver,
  )
  {
    let (Value(payload), channel) = self;

    task::spawn_blocking(move || {
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

impl<F, X, T> ForwardChannel for App<'static, F, X>
where
  X: 'static,
  F: 'static,
  F: TypeApp<'static, X, Applied = T>,
  T: ForwardChannel,
{
  fn forward_to(
    self,
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
    App::new(T::forward_from(sender, receiver))
  }
}

impl<Row, F, T> ForwardChannel for AppSum<'static, Row, F>
where
  F: TyCon,
  F: Send + 'static,
  Row: 'static,
  Row: SumApp<'static, F, Applied = T>,
  T: ForwardChannel,
{
  fn forward_to(
    self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    self.get_sum().forward_to(sender, receiver)
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    AppSum::new(T::forward_from(sender, receiver))
  }
}

impl<A, B> ForwardChannel for Sum<A, B>
where
  A: ForwardChannel,
  B: ForwardChannel,
{
  fn forward_to(
    self,
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
