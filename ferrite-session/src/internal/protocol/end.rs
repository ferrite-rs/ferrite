use core::{
  future::Future,
  pin::Pin,
};

use crate::internal::base::*;

pub struct End();

impl Protocol for End
{
  type ConsumerEndpoint = ReceiverOnce<()>;
  type ProviderEndpoint = SenderOnce<()>;

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

impl<A> RecApp<A> for End
{
  type Applied = End;
}

impl ForwardChannel for End
{
  fn forward_to(
    self,
    sender: OpaqueSender,
    _: OpaqueReceiver,
  )
  {
    sender.send(())
  }

  fn forward_from(
    _: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    let () = receiver.recv().unwrap();

    End()
  }
}
