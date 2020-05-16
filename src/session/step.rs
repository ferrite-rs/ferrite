use std::future::Future;

use crate::base::*;

pub fn step < C, A, Fut >
  ( cont1 : impl
      FnOnce () -> Fut
      + Send + 'static
  ) ->
    PartialSession < C, A >
where
  C : Context,
  A : Protocol,
  Fut :
    Future <
      Output = PartialSession < C, A >
    >
    + Send
{
  unsafe_create_session(
    async move | ins, sender | {
      let cont2 = cont1().await;

      unsafe_run_session( cont2, ins, sender ).await;
    })
}