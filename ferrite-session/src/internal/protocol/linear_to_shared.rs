use super::shared_to_linear::SharedToLinear;
use crate::internal::base::*;

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

impl<F, T> ForwardChannel for LinearToShared<F>
where
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>, Applied = T>,
  F: Send + 'static,
  T: Send + 'static + ForwardChannel,
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
      linear: Box::new(T::forward_from(sender, receiver)),
    }
  }
}
