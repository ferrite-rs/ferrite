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
  type ConsumerEndpoint = SenderOnce<(Value<T>, A::ProviderEndpoint)>;
  type ProviderEndpoint = ReceiverOnce<(Value<T>, A::ProviderEndpoint)>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    let (sender, receiver) = once_channel();
    (receiver, sender)
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let payload = provider_end.recv().await.unwrap();
      consumer_end.send(payload).unwrap();
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
