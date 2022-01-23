use core::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
};

use crate::internal::base::*;

pub struct SendChannel<A, B>(PhantomData<(A, B)>);

impl<A, B> SealedProtocol for SendChannel<A, B> {}

impl<A, B> Protocol for SendChannel<A, B>
where
  A: Protocol,
  B: Protocol,
{
  type ClientEndpoint = (ReceiverOnce<A::ClientEndpoint>, B::ClientEndpoint);
  type ProviderEndpoint = (SenderOnce<A::ClientEndpoint>, B::ProviderEndpoint);

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    let (chan_sender, chan_receiver) = once_channel();
    let (provider_end, client_end) = B::create_endpoints();

    ((chan_sender, provider_end), (chan_receiver, client_end))
  }

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    let (chan_receiver, client_end_b) = client_end;
    let (chan_sender, provider_end_b) = provider_end;

    Box::pin(async {
      let chan = chan_receiver.recv().await.unwrap();
      chan_sender.send(chan).unwrap();

      B::forward(client_end_b, provider_end_b).await;
    })
  }
}

impl<A, P, Q> RecApp<A> for SendChannel<P, Q>
where
  P: RecApp<A>,
  Q: RecApp<A>,
{
  type Applied = SendChannel<P::Applied, Q::Applied>;
}

impl<P, Q, R> SharedRecApp<R> for SendChannel<P, Q>
where
  P: Protocol,
  Q: SharedRecApp<R>,
{
  type Applied = SendChannel<P, Q::Applied>;
}
