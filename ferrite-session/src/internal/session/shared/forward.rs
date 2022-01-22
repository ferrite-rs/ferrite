use crate::internal::{
  base::*,
  protocol::{
    LinearToShared,
    Lock,
    SharedToLinear,
  },
};

pub fn shared_forward<A1, A2, C>(
  channel: SharedChannel<LinearToShared<A1>>
) -> PartialSession<(Lock<A1>, C), SharedToLinear<LinearToShared<A1>>>
where
  A1: Protocol,
  A2: Protocol,
  A1: SharedRecApp<SharedToLinear<LinearToShared<A1>>, Applied = A2>,
  C: EmptyContext,
{
  unsafe_create_session::<(Lock<A1>, C), SharedToLinear<LinearToShared<A1>>, _, _>(
    move |(lock_client_end, _), receiver1| async move {
      let lock_receiver = lock_client_end.get_applied();

      let Lock { unlock } = lock_receiver.recv().await.unwrap();

      receiver1.recv().await.unwrap();

      unsafe_forward_shared_channel(channel, unlock).await;
    },
  )
}
