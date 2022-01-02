use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use crate::internal::base::*;

pub struct ReceiveChannel<A, B>(PhantomData<(A, B)>);

impl<A, B> Protocol for ReceiveChannel<A, B>
where
  A: Protocol,
  B: Protocol,
{
  type ConsumerEndpoint =
    (SenderOnce<A::ConsumerEndpoint>, B::ConsumerEndpoint);
  type ProviderEndpoint =
    (ReceiverOnce<A::ConsumerEndpoint>, B::ProviderEndpoint);

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ConsumerEndpoint)
  {
    let (chan_sender, chan_receiver) = once_channel();
    let (provider_end, consumer_end) = B::create_endpoints();

    ((chan_receiver, provider_end), (chan_sender, consumer_end))
  }

  fn forward(
    consumer_end: Self::ConsumerEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    let (chan_sender, consumer_end_b) = consumer_end;
    let (chan_receiver, provider_end_b) = provider_end;

    Box::pin(async {
      let chan = chan_receiver.recv().await.unwrap();
      chan_sender.send(chan).unwrap();

      B::forward(consumer_end_b, provider_end_b).await;
    })
  }
}

impl<A, P, Q> RecApp<A> for ReceiveChannel<P, Q>
where
  P: RecApp<A>,
  Q: RecApp<A>,
{
  type Applied = ReceiveChannel<P::Applied, Q::Applied>;
}

impl<A, B, X> SharedRecApp<X> for ReceiveChannel<A, B>
where
  B: SharedRecApp<X>,
{
  type Applied = ReceiveChannel<A, B::Applied>;
}
