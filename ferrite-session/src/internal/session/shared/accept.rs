use core::future::Future;

use crate::internal::{
  base::*,
  functional::*,
  protocol::{
    LinearToShared,
    Lock,
    SharedToLinear,
  },
};

pub fn accept_shared_session<F>(
  cont: impl Future<Output = PartialSession<(Lock<F>, ()), F::Applied>>
    + Send
    + 'static
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
      let (lock_producer_end, lock_client_end) = <Lock<F>>::create_endpoints();

      let (producer_end, client_end) = F::Applied::create_endpoints();

      let m_sender1 = receiver1.recv().await;

      if let Some((sender5, sender6)) = m_sender1 {
        sender6
          .send(LinearToShared {
            linear: Box::new(client_end),
          })
          .unwrap();

        sender5.send(()).unwrap();

        lock_producer_end.send(Lock { unlock: receiver1 }).unwrap();

        debug!("[accept_shared_session] calling cont");

        unsafe_run_session(
          cont.await,
          (App::new(lock_client_end), ()),
          producer_end,
        )
        .await;

        debug!("[accept_shared_session] returned from cont");
      } else {
        // shared session is terminated with all references to it
        // being dropped
      }
    },
  )
}
