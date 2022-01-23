use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use crate::internal::base::*;

pub struct ReceiveChannel<A, B>(PhantomData<(A, B)>);

impl<A, B> SealedProtocol for ReceiveChannel<A, B> {}

impl<A, B> Protocol for ReceiveChannel<A, B>
where
  A: Protocol,
  B: Protocol,
{
  type ClientEndpoint = (SenderOnce<A::ClientEndpoint>, B::ClientEndpoint);
  type ProviderEndpoint =
    (ReceiverOnce<A::ClientEndpoint>, B::ProviderEndpoint);

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    let (chan_sender, chan_receiver) = once_channel();
    let (provider_end, client_end) = B::create_endpoints();

    ((chan_receiver, provider_end), (chan_sender, client_end))
  }

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    let (chan_sender, client_end_b) = client_end;
    let (chan_receiver, provider_end_b) = provider_end;

    Box::pin(async {
      let chan = chan_receiver.recv().await.unwrap();
      chan_sender.send(chan).unwrap();

      B::forward(client_end_b, provider_end_b).await;
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
