use async_macros::join;
use tokio::task;

use crate::internal::{
  base::*,
  functional::nat::*,
};

pub fn fix_session<F, A, C>(
  cont : PartialSession<C, A>
) -> PartialSession<C, Rec<F>>
where
  C : Context,
  F : Protocol,
  A : Protocol,
  F : RecApp<Unfix<Rec<F>>, Applied = A>,
{
  unsafe_create_session(move |ctx, sender1| async move {
    let (sender2, receiver) : (SenderOnce<A>, _) = once_channel();

    let child1 = task::spawn(async move {
      let val = receiver.recv().await.unwrap();

      sender1.send(fix(val)).unwrap();
    });

    let child2 = task::spawn(unsafe_run_session(cont, ctx, sender2));

    let _ = join!(child1, child2).await;
  })
}

pub fn unfix_session<C, F, A>(
  cont : PartialSession<C, Rec<F>>
) -> PartialSession<C, A>
where
  C : Context,
  F : Protocol,
  A : Protocol,
  F : RecApp<Unfix<Rec<F>>, Applied = A>,
{
  unsafe_create_session(move |ctx, sender1| async move {
    let (sender2, receiver) = once_channel();

    let child1 = task::spawn(async move {
      let val = receiver.recv().await.unwrap();

      sender1.send(unfix(val)).unwrap();
    });

    let child2 = task::spawn(unsafe_run_session(cont, ctx, sender2));

    let _ = join!(child1, child2).await;
  })
}

pub fn succ_session<I, P>(
  cont : PartialSession<I, P>
) -> PartialSession<I, S<P>>
where
  P : Protocol,
  I : Context,
{
  unsafe_create_session(move |ctx, sender| async move {
    let (sender2, receiver) = once_channel();

    let child1 = task::spawn(async move {
      let val = receiver.recv().await.unwrap();

      sender.send(succ(val)).unwrap();
    });

    let child2 = task::spawn(unsafe_run_session(cont, ctx, sender2));

    let _ = join!(child1, child2).await;
  })
}

pub fn unfix_session_for<N, C, A, B, F>(
  _ : N,
  cont : PartialSession<N::Target, B>,
) -> PartialSession<C, B>
where
  B : Protocol,
  C : Context,
  F : Protocol,
  F : RecApp<Unfix<Rec<F>>, Applied = A>,
  A : Protocol,
  N : ContextLens<C, Rec<F>, A>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let (receiver1, ctx2) = N::extract_source(ctx1);

    let (sender2, receiver2) = once_channel();

    let ctx3 = N::insert_target(receiver2, ctx2);

    let child1 = task::spawn(async move {
      let val = receiver1.recv().await.unwrap();

      sender2.send(unfix(val)).unwrap();
    });

    let child2 = task::spawn(unsafe_run_session(cont, ctx3, sender1));

    let _ = join!(child1, child2).await;
  })
}
