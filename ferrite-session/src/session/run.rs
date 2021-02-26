use async_macros::join;
use tokio::task;

use crate::{
  base::*,
  protocol::{End, SendValue},
};

pub async fn run_session(session : Session<End>)
{

  let (sender, receiver) = once_channel();

  let child1 = task::spawn(async move {

    unsafe_run_session(session, (), sender).await;
  });

  let child2 = task::spawn(async move {

    receiver.recv().await.unwrap();
  });

  let _ = join!(child1, child2).await;
}

pub async fn run_session_with_result<T>(
  session : Session<SendValue<T, End>>
) -> T
where
  T : Send + 'static,
{

  let (sender, receiver1) = once_channel();

  let child1 = task::spawn(async move {

    unsafe_run_session(session, (), sender).await;
  });

  let SendValue((Value(val), receiver2)) = receiver1.recv().await.unwrap();

  receiver2.recv().await.unwrap();

  let _ = child1.await;

  val
}
