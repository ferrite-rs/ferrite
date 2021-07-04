use std::marker::PhantomData;

use tokio::{
  task,
  try_join,
};

use crate::internal::{
  base::*,
  functional::nat::*,
  protocol::{
    End,
    LinearToShared,
    Lock,
    SendValue,
    SharedToLinear,
  },
};

pub fn accept_shared_session<F>(
  cont: PartialSession<(Lock<F>, ()), F::Applied>
) -> SharedSession<LinearToShared<F>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  unsafe_create_shared_session(
    move |receiver1: Receiver<(
      SenderOnce<()>,
      SenderOnce<LinearToShared<F>>,
    )>| async move {
      let (sender2, receiver2): (SenderOnce<Lock<F>>, _) = once_channel();

      let (sender4, receiver4): (SenderOnce<F::Applied>, _) = once_channel();

      let m_sender1 = receiver1.recv().await;

      if let Some((sender5, sender6)) = m_sender1 {
        let child1 = task::spawn(async move {
          debug!("[accept_shared_session] calling cont");

          unsafe_run_session(cont, (receiver2, ()), sender4).await;

          debug!("[accept_shared_session] returned from cont");
        });

        let child2 = task::spawn(async move {
          let linear = receiver4.recv().await.unwrap();

          debug!("[accept_shared_session] received from receiver4");

          sender6
            .send(LinearToShared {
              linear: Box::new(linear),
            })
            .unwrap();
        });

        let child3 = task::spawn(async move {
          sender5.send(()).unwrap();
        });

        let child4 = task::spawn(async move {
          debug!("[accept_shared_session] sending sender12");

          sender2.send(Lock { unlock: receiver1 }).unwrap();

          debug!("[accept_shared_session] sent sender12");
        });

        let _ = try_join!(child1, child2, child3, child4).unwrap();
      } else {
        // shared session is terminated with all references to it
        // being dropped
      }
    },
  )
}

pub fn detach_shared_session<F, C>(
  cont: SharedSession<LinearToShared<F>>
) -> PartialSession<(Lock<F>, C), SharedToLinear<LinearToShared<F>>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
  C: EmptyContext,
{
  unsafe_create_session(
    move |(receiver1, _): (ReceiverOnce<Lock<F>>, C::Endpoints), sender1| async move {
      let (sender3, receiver3) = once_channel::<()>();

      let child1 = task::spawn(async move {
        debug!("[detach_shared_session] receiving sender2");

        let Lock { unlock: receiver2 } = receiver1.recv().await.unwrap();

        receiver3.recv().await.unwrap();

        debug!("[detach_shared_session] received sender2");

        unsafe_run_shared_session(cont, receiver2).await;

        debug!("[detach_shared_session] ran cont");
      });

      let child2 = task::spawn(async move {
        debug!("[detach_shared_session] sending sender1");

        sender1
          .send(SharedToLinear {
            unlock: sender3,
            phantom: PhantomData,
          })
          .unwrap();

        debug!("[detach_shared_session] sent sender1");
      });

      try_join!(child1, child2).unwrap();
    },
  )
}

pub fn async_acquire_shared_session<F>(
  shared: SharedChannel<LinearToShared<F>>,
  cont1: impl FnOnce(Z) -> PartialSession<(F::Applied, ()), End> + Send + 'static,
) -> task::JoinHandle<()>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  debug!("[async_acquire_shared_session] acquiring shared session");

  let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

  task::spawn(async move {
    let (sender1, receiver1) = once_channel();

    let (sender2, receiver2) = once_channel();

    let cont2 = cont1(Z);

    let ctx = (receiver2, ());

    let child1 = task::spawn(async move {
      let LinearToShared { linear } = receiver4.recv().await.unwrap();

      sender2.send(*linear.get_applied()).unwrap();
    });

    let child2 = task::spawn(async move {
      unsafe_run_session(cont2, ctx, sender1).await;
    });

    let child3 = task::spawn(async move {
      receiver1.recv().await.unwrap();
    });

    let child4 = task::spawn(async move {
      receiver3.recv().await.unwrap();

      debug!("[async_acquire_shared_session] acquired shared session");
    });

    try_join!(child1, child2, child3, child4).unwrap();
  })
}

pub fn async_acquire_shared_session_with_result<T, F>(
  shared: SharedChannel<LinearToShared<F>>,
  cont1: impl FnOnce(Z) -> PartialSession<(F::Applied, ()), SendValue<T, End>>
    + Send
    + 'static,
) -> task::JoinHandle<T>
where
  F: Protocol,
  T: Send + 'static,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  debug!("[async_acquire_shared_session_with_result] acquiring shared session");

  let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

  task::spawn(async move {
    let (sender1, receiver1) = once_channel();

    let (sender2, receiver2) = once_channel();

    let cont2 = cont1(Z);

    let ctx = (receiver2, ());

    let child1 = task::spawn(async move {
      let LinearToShared { linear } = receiver4.recv().await.unwrap();

      sender2.send(*linear.get_applied()).unwrap();
    });

    let child2 = task::spawn(async move {
      unsafe_run_session(cont2, ctx, sender1).await;
    });

    let child3 = task::spawn(async move {
      let SendValue((Value(val), receiver3)) = receiver1.recv().await.unwrap();

      receiver3.recv().await.unwrap();

      val
    });

    let child4 = task::spawn(async move {
      receiver3.recv().await.unwrap();

      debug!(
        "[async_acquire_shared_session_with_result] acquired shared session"
      );
    });

    let (_, _, val, _) = try_join!(child1, child2, child3, child4).unwrap();

    val
  })
}

pub fn acquire_shared_session<N, C1, C2, A1, A2, B>(
  shared: SharedChannel<LinearToShared<A1>>,
  cont1: impl FnOnce(N) -> PartialSession<C2, B> + Send + 'static,
) -> PartialSession<C1, B>
where
  N: Nat,
  C1: Context<Length = N>,
  C2: Context,
  A1: Protocol,
  A2: Protocol,
  B: Protocol,
  A1: SharedRecApp<SharedToLinear<LinearToShared<A1>>, Applied = A2>,
  C1: AppendContext<(A2, ()), Appended = C2>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let cont2 = cont1(N::nat());

    let (sender2, receiver2) = once_channel();

    let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

    debug!("[acquire_shared_session] acquiring shared endpoint");

    receiver3.recv().await.unwrap();

    debug!("[acquire_shared_session] acquired shared endpoint");

    let ctx2 = C1::append_context(ctx1, (receiver2, ()));

    let child1 = task::spawn(async move {
      let LinearToShared { linear } = receiver4.recv().await.unwrap();

      sender2.send(*linear.get_applied()).unwrap();
    });

    let child2 = task::spawn(async move {
      unsafe_run_session(cont2, ctx2, sender1).await;
    });

    try_join!(child1, child2).unwrap();

    // debug!("[acquire_shared_session] ran cont");
  })
}

pub fn release_shared_session<N, C1, C2, A, B>(
  _n: N,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, B>
where
  A: Protocol,
  B: Protocol,
  C1: Context,
  C2: Context,
  N: ContextLens<C1, SharedToLinear<LinearToShared<A>>, Empty, Target = C2>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let (receiver2, ctx2) = N::extract_source(ctx1);

    let ctx3 = N::insert_target((), ctx2);

    debug!("[release_shared_session] waiting receiver2");

    let lock: SharedToLinear<LinearToShared<A>> =
      receiver2.recv().await.unwrap();

    lock.unlock.send(()).unwrap();

    debug!("[release_shared_session] received receiver2");

    unsafe_run_session(cont, ctx3, sender1).await;

    debug!("[release_shared_session] ran cont");
  })
}

pub fn shared_forward<A1, A2, C>(
  channel: SharedChannel<LinearToShared<A1>>
) -> PartialSession<(Lock<A1>, C), SharedToLinear<A1>>
where
  A1: Protocol,
  A2: Protocol,
  A1: SharedRecApp<SharedToLinear<LinearToShared<A1>>, Applied = A2>,
  C: EmptyContext,
{
  unsafe_create_session(
    move |(receiver1, _): (ReceiverOnce<Lock<A1>>, C::Endpoints), sender1| async move {
      let (sender3, receiver3) = once_channel::<()>();

      sender1
        .send(SharedToLinear {
          unlock: sender3,
          phantom: PhantomData,
        })
        .unwrap();

      let Lock { unlock: receiver2 } = receiver1.recv().await.unwrap();
      receiver3.recv().await.unwrap();

      task::spawn(async move {
        unsafe_forward_shared_channel(channel, receiver2).await;
      });
    },
  )
}

impl<F> SharedChannel<LinearToShared<F>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
{
  pub fn acquire<C, A>(
    &self,
    cont: impl FnOnce(C::Length) -> PartialSession<C::Appended, A> + Send + 'static,
  ) -> PartialSession<C, A>
  where
    C: Context,
    A: Protocol,
    C: AppendContext<(F::Applied, ())>,
  {
    acquire_shared_session(self.clone(), cont)
  }
}
