use super::{
  linear_to_shared::LinearToShared,
  shared_to_linear::SharedToLinear,
};
use crate::internal::base::*;

pub struct Lock<F>
where
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
{
  pub(crate) unlock: Receiver<(SenderOnce<()>, SenderOnce<LinearToShared<F>>)>,
}

impl<F> Protocol for Lock<F>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
}
