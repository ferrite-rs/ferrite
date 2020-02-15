
use async_std::task;
use async_macros::join;
use async_std::sync::{ channel };

use crate::process::{ End };

use crate::base::{
  EmptyContext,
  PartialSession,
  run_partial_session
};

pub fn
  run_session
  < Ins >
  (session : PartialSession < Ins, End >)
where
  Ins : EmptyContext + 'static
{
  let (sender, receiver) = channel(1);
  let ins = < Ins as EmptyContext > :: make_empty_list ();

  task::block_on(async {
    let child1 = task::spawn(async {
      run_partial_session
        ( session, ins, sender
        ).await;
    });

    let child2 = task::spawn(async move {
      receiver.recv().await.unwrap();
    });

    join!(child1, child2).await;
  });
}
