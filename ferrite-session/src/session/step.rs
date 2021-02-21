use std::future::Future;

use crate::base::*;

pub fn step < C, A >
  ( cont1 : impl
      Future <
        Output = PartialSession < C, A >
      >
      + Send + 'static
  ) ->
    PartialSession < C, A >
where
  C : Context,
  A : Protocol,
{
  unsafe_create_session(
    move | ins, sender | async move {
      let cont2 = cont1.await;

      unsafe_run_session( cont2, ins, sender ).await;
    })
}
