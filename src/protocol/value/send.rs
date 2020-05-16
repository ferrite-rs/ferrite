use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TypeApp, Protocol };

pub struct SendValue < T, A >
(
  pub (crate) T,
  pub (crate) Receiver < A >
);

impl < T, P > Protocol for SendValue < T, P >
where
  T : Send + 'static,
  P : Protocol
{ }

impl < X, T, A >
  TypeApp < X > for
  SendValue < T, A >
where
  A : TypeApp < X >,
{
  type Applied =
    SendValue <
      T,
      A :: Applied
    >;
}
