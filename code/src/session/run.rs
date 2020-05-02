
use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };

use crate::process::{ End };

use crate::base::{
  PartialSession,
  unsafe_run_session
};

pub async fn run_session
  ( session : PartialSession < (), End > )
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
