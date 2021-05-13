use std::marker::PhantomData;

use async_macros::join;
use tokio::task;

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
  cont: impl FnOnce() -> PartialSession<(Lock<F>, ()), F::Applied> + Send + 'static
) -> SharedSession<LinearToShared<F>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<F>>,
  F::Applied: Protocol,
{
  unsafe_create_shared_session(
    move |receiver1: Receiver<(
      SenderOnce<()>,
      SenderOnce<LinearToShared<F>>,
    )>| async move {
      let cont2 = cont();

      let (sender2, receiver2): (SenderOnce<Lock<F>>, _) = once_channel();

      let (sender4, receiver4): (SenderOnce<F::Applied>, _) = once_channel();

      let m_sender1 = receiver1.recv().await;

      match m_sender1 {
        Some((sender5, sender6)) => {
          let child1 = task::spawn(async move {
            debug!("[accept_shared_session] calling cont");

            unsafe_run_session(cont2, (receiver2, ()), sender4).await;

            debug!("[accept_shared_session] returned from cont");
          });

          let child2 = task::spawn(async move {
            let linear = receiver4.recv().await.unwrap();

            debug!("[accept_shared_session] received from receiver4");

            sender6.send(LinearToShared { linear }).unwrap();
          });

          let child3 = task::spawn(async move {
            sender5.send(()).unwrap();
          });

          let child4 = task::spawn(async move {
            debug!("[accept_shared_session] sending sender12");

            sender2.send(Lock { unlock: receiver1 }).unwrap();

            debug!("[accept_shared_session] sent sender12");
          });

          let _ = join!(child1, child2, child3, child4).await;
        }
        None => {
          // shared session is terminated with all references to it
          // being dropped
        }
      }
    },
  )
}

pub fn detach_shared_session<F, C>(
  cont: SharedSession<LinearToShared<F>>
) -> PartialSession<(Lock<F>, C), SharedToLinear<F>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<F>>,
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

      let _ = join!(child1, child2).await;
    },
  )
}

pub fn async_acquire_shared_session<F>(
  shared: SharedChannel<LinearToShared<F>>,
  cont_builder: impl FnOnce(Z) -> PartialSession<(F::Applied, ()), End>
    + Send
    + 'static,
) -> task::JoinHandle<()>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<F>>,
  F::Applied: Protocol,
{
  debug!("[async_acquire_shared_session] acquiring shared session");

  let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

  task::spawn(async move {
    let (sender1, receiver1) = once_channel();

    let (sender2, receiver2) = once_channel();

    let cont = cont_builder(Z);

    let ctx = (receiver2, ());

    let child1 = task::spawn(async move {
      let LinearToShared { linear } = receiver4.recv().await.unwrap();

      sender2.send(linear).unwrap();
    });

    let child2 = task::spawn(async move {
      unsafe_run_session(cont, ctx, sender1).await;
    });

    let child3 = task::spawn(async move {
      receiver1.recv().await.unwrap();
    });

    let child4 = task::spawn(async move {
      receiver3.recv().await.unwrap();

      debug!("[async_acquire_shared_session] acquired shared session");
    });

    let _ = join!(child1, child2, child3, child4).await;
  })
}

pub fn async_acquire_shared_session_with_result<T, F>(
  shared: SharedChannel<LinearToShared<F>>,
  cont_builder: impl FnOnce(Z) -> PartialSession<(F::Applied, ()), SendValue<T, End>>
    + Send
    + 'static,
) -> task::JoinHandle<T>
where
  F: Protocol,
  T: Send + 'static,
  F: SharedRecApp<SharedToLinear<F>>,
  F::Applied: Protocol,
{
  debug!("[async_acquire_shared_session_with_result] acquiring shared session");

  let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

  task::spawn(async move {
    let (sender1, receiver1) = once_channel();

    let (sender2, receiver2) = once_channel();

    let cont = cont_builder(Z);

    let ctx = (receiver2, ());

    let child1 = task::spawn(async move {
      let LinearToShared { linear } = receiver4.recv().await.unwrap();

      sender2.send(linear).unwrap();
    });

    let child2 = task::spawn(async move {
      unsafe_run_session(cont, ctx, sender1).await;
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

    let (_, _, val, _) = join!(child1, child2, child3, child4).await;

    val.unwrap()
  })
}

pub fn acquire_shared_session<F, C, A>(
  shared: SharedChannel<LinearToShared<F>>,
  cont1: impl FnOnce(C::Length) -> PartialSession<C::Appended, A> + Send + 'static,
) -> PartialSession<C, A>
where
  C: Context,
  A: Protocol,
  F: Protocol,
  F: SharedRecApp<SharedToLinear<F>>,
  C: AppendContext<(F::Applied, ())>,
  F::Applied: Protocol,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let cont2 = cont1(<C::Length as Nat>::nat());

    let (sender2, receiver2) = once_channel();

    let (receiver3, receiver4) = unsafe_receive_shared_channel(shared);

    debug!("[acquire_shared_session] acquiring shared endpoint");

    receiver3.recv().await.unwrap();

    debug!("[acquire_shared_session] acquired shared endpoint");

    let ctx2 = C::append_context(ctx1, (receiver2, ()));

    let child1 = task::spawn(async move {
      let LinearToShared { linear } = receiver4.recv().await.unwrap();

      sender2.send(linear).unwrap();
    });

    let child2 = task::spawn(async move {
      unsafe_run_session(cont2, ctx2, sender1).await;
    });

    let _ = join!(child1, child2).await;

    // debug!("[acquire_shared_session] ran cont");
  })
}

pub fn release_shared_session<F, C, A, N>(
  _: N,
  cont: PartialSession<N::Target, A>,
) -> PartialSession<C, A>
where
  A: Protocol,
  C: Context,
  F: Protocol,
  N: ContextLens<C, SharedToLinear<F>, Empty>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let (receiver2, ctx2) = N::extract_source(ctx1);

    let ctx3 = N::insert_target((), ctx2);

    debug!("[release_shared_session] waiting receiver2");

    let lock: SharedToLinear<F> = receiver2.recv().await.unwrap();

    lock.unlock.send(()).unwrap();

    debug!("[release_shared_session] received receiver2");

    unsafe_run_session(cont, ctx3, sender1).await;

    debug!("[release_shared_session] ran cont");
  })
}

impl<F> SharedChannel<LinearToShared<F>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<F>>,
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
