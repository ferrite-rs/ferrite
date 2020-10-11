use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };

use crate::protocol::{ SendValue, End };

use crate::base::{
  Session,
  unsafe_run_session,
};

pub async fn run_session
  ( session : Session < End > )
{
  let (sender, receiver) = channel(1);

  let child1 = task::spawn ( async move {
    unsafe_run_session
      ( session, (), sender
      ).await;
  });

  let child2 = task::spawn ( async move {
    receiver.recv().await.unwrap();
  });

  join!(child1, child2).await;
}


pub async fn run_session_with_result < T >
  ( session :
      Session <
        SendValue < T, End >
      >
  ) -> T
where
  T: Send + 'static
{
  let (sender, receiver1) = channel(1);

  let child1 = task::spawn ( async move {
    unsafe_run_session
      ( session, (), sender
      ).await;
  });

  let SendValue ( val, receiver2 ) =
    receiver1.recv().await.unwrap();

  receiver2.recv().await.unwrap();

  child1.await;

  val
}