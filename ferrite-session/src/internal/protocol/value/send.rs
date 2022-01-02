use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use crate::internal::base::*;

pub struct SendValue<T, A>(PhantomData<(T, A)>);

impl<T, A> Protocol for SendValue<T, A>
where
  T: Send + 'static,
  A: Protocol,
{
  type ConsumerEndpoint = (ReceiverOnce<Value<T>>, A::ConsumerEndpoint);
  type ProviderEndpoint = (SenderOnce<Value<T>>, A::ProviderEndpoint);

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    let (val_sender, val_receiver) = once_channel();
    let (provider_end, consumer_end) = A::create_endpoints();

    ((val_sender, provider_end), (val_receiver, consumer_end))
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    let (val_sender, provider_end_a) = provider_end;
    let (val_receiver, consumer_end_a) = consumer_end;

    Box::pin(async {
      let payload = val_receiver.recv().await.unwrap();
      val_sender.send(payload).unwrap();

      A::forward(consumer_end_a, provider_end_a).await;
    })
  }
}

impl<X, T, A> RecApp<X> for SendValue<T, A>
where
  T: Send + 'static,
  A: RecApp<X>,
{
  type Applied = SendValue<T, A::Applied>;
}

impl<T, A, X> SharedRecApp<X> for SendValue<T, A>
where
  T: Send + 'static,
  A: SharedRecApp<X>,
{
  type Applied = SendValue<T, A::Applied>;
}
