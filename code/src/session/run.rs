
use crate::process::{ End };
use crate::base::{ EmptyList, PartialSession };
use async_std::task;
use async_std::sync::{ channel };
use async_macros::join;

pub type RunnableSession = PartialSession < (), End >;

pub fn run_session
  < Ins >
  (session : PartialSession < Ins, End >)
where
  Ins : EmptyList + 'static
{
  let (sender, receiver) = channel(1);
  let ins = < Ins as EmptyList > :: make_empty_list ();

  task::block_on(async {
    let child1 = task::spawn(async {
      (session.builder)(ins, sender).await;
    });

    let child2 = task::spawn(async move {
      receiver.recv().await.unwrap();
    });

    join!(child1, child2).await;
  });
}
