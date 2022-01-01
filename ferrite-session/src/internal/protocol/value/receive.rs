use core::{
  future::Future,
  pin::Pin,
};

use crate::internal::base::*;

pub struct ReceiveValue<T, A>(pub(crate) SenderOnce<(Value<T>, SenderOnce<A>)>);

impl<T, A> Protocol for ReceiveValue<T, A>
where
  T: Send + 'static,
  A: Protocol,
{
  type ConsumerEndpoint = (SenderOnce<Value<T>>, A::ConsumerEndpoint);
  type ProviderEndpoint = (ReceiverOnce<Value<T>>, A::ProviderEndpoint);

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    let (val_sender, val_receiver) = once_channel();
    let (provider_end, consumer_end) = A::create_endpoints();

    ((val_receiver, provider_end), (val_sender, consumer_end))
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    let (val_receiver, provider_end_a) = provider_end;
    let (val_sender, consumer_end_a) = consumer_end;

    Box::pin(async {
      let payload = val_receiver.recv().await.unwrap();
      val_sender.send(payload).unwrap();

      A::forward(consumer_end_a, provider_end_a).await;
    })
  }
}

impl<X, T, A> RecApp<X> for ReceiveValue<T, A>
where
  X: Send + 'static,
  T: Send + 'static,
  A: RecApp<X>,
{
  type Applied = ReceiveValue<T, A::Applied>;
}

impl<T, A, X> SharedRecApp<X> for ReceiveValue<T, A>
where
  T: Send + 'static,
  A: SharedRecApp<X>,
{
  type Applied = ReceiveValue<T, A::Applied>;
}

impl<T, A> ForwardChannel for ReceiveValue<T, A>
where
  A: ForwardChannel,
  T: Send + 'static,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn forward_to(
    self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    self.0.forward_to(sender, receiver)
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    ReceiveValue(<SenderOnce<(Value<T>, SenderOnce<A>)>>::forward_from(
      sender, receiver,
    ))
  }
}
