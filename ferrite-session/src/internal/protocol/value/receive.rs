use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use crate::internal::base::*;

pub struct ReceiveValue<T, A>(PhantomData<(T, A)>);

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
