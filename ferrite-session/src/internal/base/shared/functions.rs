use core::{
  future::Future,
  pin::Pin,
};

use tokio::task;

use super::types::*;
use crate::internal::base::{
  channel::*,
  protocol::*,
};

pub async fn unsafe_forward_shared_channel<S>(
  channel: SharedChannel<S>,
  receiver: Receiver<(SenderOnce<()>, SenderOnce<S>)>,
) where
  S: SharedProtocol,
{
  while let Some(senders) = receiver.recv().await {
    channel.endpoint.send(senders).unwrap();
  }
}

pub async fn unsafe_run_shared_session<S>(
  session: SharedSession<S>,
  receiver: Receiver<(SenderOnce<()>, SenderOnce<S>)>,
) where
  S: SharedProtocol,
{
  (session.executor)(receiver).await;
}

pub fn unsafe_create_shared_session<S, Fut>(
  executor1: impl FnOnce(Receiver<(SenderOnce<()>, SenderOnce<S>)>) -> Fut
    + Send
    + 'static
) -> SharedSession<S>
where
  S: SharedProtocol,
  Fut: Future<Output = ()> + Send,
{
  let executor: Box<
    dyn FnOnce(
        Receiver<(SenderOnce<()>, SenderOnce<S>)>,
      ) -> Pin<Box<dyn Future<Output = ()> + Send>>
      + Send,
  > = Box::new(move |receiver| {
    Box::pin(async {
      task::spawn(async move {
        executor1(receiver).await;
      })
      .await
      .unwrap();
    })
  });

  SharedSession { executor }
}

pub fn unsafe_create_shared_channel<S>(
) -> (SharedChannel<S>, Receiver<(SenderOnce<()>, SenderOnce<S>)>)
where
  S: SharedProtocol,
{
  let (sender, receiver) = unbounded();

  (SharedChannel { endpoint: sender }, receiver)
}

pub fn unsafe_receive_shared_channel<S>(
  session: SharedChannel<S>
) -> (ReceiverOnce<()>, ReceiverOnce<S>)
where
  S: SharedProtocol,
{
  let (sender1, receiver1) = once_channel::<()>();

  let (sender2, receiver2) = once_channel::<S>();

  session.endpoint.send((sender1, sender2)).unwrap();

  (receiver1, receiver2)
}
