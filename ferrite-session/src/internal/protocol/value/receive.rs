use serde;

use crate::internal::base::*;

pub struct ReceiveValue<T, A>(pub(crate) SenderOnce<(Value<T>, SenderOnce<A>)>);

impl<T, A> Protocol for ReceiveValue<T, A>
where
  T : Send + 'static,
  A : Protocol,
{
}

impl<X, T, A> RecApp<X> for ReceiveValue<T, A>
where
  X : Send + 'static,
  T : Send + 'static,
  A : RecApp<X>,
{
  type Applied = ReceiveValue<T, A::Applied>;
}

impl<T, A, X> SharedRecApp<X> for ReceiveValue<T, A>
where
  T : Send + 'static,
  A : SharedRecApp<X>,
{
  type Applied = ReceiveValue<T, A::Applied>;
}

impl<T, A> ForwardChannel for ReceiveValue<T, A>
where
  A : ForwardChannel,
  T : Send + 'static,
  T : serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn forward_to(
    self,
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  )
  {
    self.0.forward_to(sender, receiver)
  }

  fn forward_from(
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  ) -> Self
  {
    ReceiveValue(<SenderOnce<(Value<T>, SenderOnce<A>)>>::forward_from(
      sender, receiver,
    ))
  }
}
