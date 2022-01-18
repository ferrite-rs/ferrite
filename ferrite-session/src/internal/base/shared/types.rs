use core::{
  future::Future,
  pin::Pin,
};

use crate::internal::base::{
  channel::*,
  protocol::*,
};

pub struct SharedSession<S>
where
  S: SharedProtocol,
{
  pub(crate) executor: Box<
    dyn FnOnce(
        Receiver<(SenderOnce<()>, SenderOnce<S>)>,
      ) -> Pin<Box<dyn Future<Output = ()> + Send>>
      + Send,
  >,
}

pub struct SharedChannel<S>
where
  S: SharedProtocol,
{
  pub(crate) endpoint: Sender<(SenderOnce<()>, SenderOnce<S>)>,
}

impl<S> Clone for SharedChannel<S>
where
  S: SharedProtocol,
{
  fn clone(&self) -> Self
  {
    SharedChannel {
      endpoint: self.endpoint.clone(),
    }
  }
}
