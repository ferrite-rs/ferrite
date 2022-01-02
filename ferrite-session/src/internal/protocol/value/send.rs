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
  type ConsumerEndpoint = ReceiverOnce<(Value<T>, A::ConsumerEndpoint)>;
  type ProviderEndpoint = SenderOnce<(Value<T>, A::ConsumerEndpoint)>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    once_channel()
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let payload = consumer_end.recv().await.unwrap();
      provider_end.send(payload).unwrap();
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
