use tokio::task;

use crate::internal::{
  base::*,
  protocol::{
    LinearToShared,
    Lock,
    SharedToLinear,
  },
};

pub fn detach_shared_session<F, C>(
  cont: SharedSession<LinearToShared<F>>
) -> PartialSession<(Lock<F>, C), SharedToLinear<LinearToShared<F>>>
where
  F: Protocol,
  F: SharedRecApp<SharedToLinear<LinearToShared<F>>>,
  F::Applied: Protocol,
  C: EmptyContext,
{
  unsafe_create_session::<(Lock<F>, C), SharedToLinear<LinearToShared<F>>, _, _>(
    move |(lock_client_end, _), receiver| async move {
      debug!("[detach_shared_session] receiving sender2");

      let lock_receiver = lock_client_end.get_applied();

      let Lock { unlock: receiver2 } = lock_receiver.recv().await.unwrap();

      receiver.recv().await.unwrap();

      debug!("[detach_shared_session] received sender2");

      // Run the continuation as a separate task *without* awaiting to
      // avoice stack overflow in the async worker thread.
      task::spawn(async move {
        unsafe_run_shared_session(cont, receiver2).await;
      })
      .await
      .unwrap();

      debug!("[detach_shared_session] ran cont");
    },
  )
}
