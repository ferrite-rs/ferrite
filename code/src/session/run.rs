
use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };

use crate::process::{ End };

use crate::base::{
  EmptyContext,
  PartialSession,
  unsafe_run_session
};

pub fn
  run_session
  < C >
  (session : PartialSession < C, End >)
where
  C : EmptyContext
{
  let (sender, receiver) = channel(1);
  let ctx = < C as EmptyContext > :: empty_values ();

  task::block_on(async {
    let child1 = task::spawn(async {
      unsafe_run_session
        ( session, ctx, sender
        ).await;
    });

    let child2 = task::spawn(async move {
      receiver.recv().await.unwrap();
    });

    join!(child1, child2).await;
  });
}
