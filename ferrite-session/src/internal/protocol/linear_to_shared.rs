use super::shared_to_linear::SharedToLinear;
use crate::internal::base::*;

pub trait HasSharedRecApp<F, A>: Send + 'static
{
  fn get_applied(self: Box<Self>) -> <F::Applied as Protocol>::ClientEndpoint
  where
    F: SharedRecApp<A>,
    F::Applied: Protocol;
}

impl<F, A, FA, E> HasSharedRecApp<F, A> for E
where
  F: 'static,
  A: 'static,
  E: Send + 'static,
  FA: Protocol<ClientEndpoint = E>,
  F: SharedRecApp<A, Applied = FA>,
{
  fn get_applied(self: Box<Self>) -> <F::Applied as Protocol>::ClientEndpoint
  {
    *self
  }
}

pub struct LinearToShared<F>
{
  pub(crate) linear:
    Box<dyn HasSharedRecApp<F, SharedToLinear<LinearToShared<F>>>>,
}

impl<F> SharedProtocol for LinearToShared<F>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
}

impl<F, T, E> ForwardChannel for LinearToShared<F>
where
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>, Applied = T>,
  F: Send + 'static,
  T: Protocol<ClientEndpoint = E>,
  E: ForwardChannel,
{
  fn forward_to(
    self,
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  )
  {
    self.linear.get_applied().forward_to(sender, receiver)
  }

  fn forward_from(
    sender: OpaqueSender,
    receiver: OpaqueReceiver,
  ) -> Self
  {
    LinearToShared {
      linear: Box::new(E::forward_from(sender, receiver)),
    }
  }
}
