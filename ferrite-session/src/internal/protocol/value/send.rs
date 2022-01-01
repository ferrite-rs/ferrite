use crate::internal::base::*;

pub struct SendValue<T, A>(pub(crate) (Value<T>, ReceiverOnce<A>));

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
    let (provider, consumer) = A::create_endpoints();

    ((val_sender, provider), (val_receiver, consumer))
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

impl<T, A> ForwardChannel for SendValue<T, A>
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
    SendValue(<(Value<T>, ReceiverOnce<A>)>::forward_from(
      sender, receiver,
    ))
  }
}
