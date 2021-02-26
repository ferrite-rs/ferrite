use super::{
  fix::SharedRecApp,
  protocol::SharedProtocol,
  shared_to_linear::SharedToLinear,
};
use crate::base::*;

pub struct LinearToShared<F>
where
  F : SharedRecApp<SharedToLinear<F>>,
{
  pub(crate) linear : F::Applied,
}

impl<F> SharedProtocol for LinearToShared<F>
where
  F : Protocol,
  F : SharedRecApp<SharedToLinear<F>>,
  F::Applied : Protocol,
{
}

impl<F, T> ForwardChannel for LinearToShared<F>
where
  F : Send + 'static + SharedRecApp<SharedToLinear<F>, Applied = T>,
  T : Send + 'static + ForwardChannel,
{
  fn forward_to(
    self,
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  )
  {

    self.linear.forward_to(sender, receiver)
  }

  fn forward_from(
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  ) -> Self
  {

    LinearToShared {
      linear : T::forward_from(sender, receiver),
    }
  }
}
