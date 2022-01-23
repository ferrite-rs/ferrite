use core::{
  future::Future,
  pin::Pin,
};

use crate::internal::base::*;

pub struct End();

impl SealedProtocol for End {}

impl Protocol for End
{
  type ClientEndpoint = ReceiverOnce<()>;
  type ProviderEndpoint = SenderOnce<()>;

  fn create_endpoints() -> (Self::ProviderEndpoint, Self::ClientEndpoint)
  {
    once_channel()
  }

  fn forward(
    client_end: Self::ClientEndpoint,
    provider_end: Self::ProviderEndpoint,
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
  {
    Box::pin(async {
      let payload = client_end.recv().await.unwrap();
      provider_end.send(payload).unwrap();
    })
  }
}

impl<A> RecApp<A> for End
{
  type Applied = End;
}
