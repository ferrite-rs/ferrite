pub use super::{
  accept_shared_session,
  acquire_shared_session,
  async_acquire_shared_session,
  async_acquire_shared_session_with_result,
  detach_shared_session,
  release_shared_session,
  run_shared_session,
  run_shared_session_with_join_handle,
  LinearToShared,
  SharedChannel,
  SharedSession,
  SharedToLinear,
};

pub trait SharedRecApp<X>: super::SharedRecApp<X>
{
}

impl<X, S> SharedRecApp<X> for S where S : super::SharedRecApp<X> {}

pub trait SharedProtocol: super::SharedProtocol
{
}

impl<A> SharedProtocol for A where A : super::SharedProtocol {}
