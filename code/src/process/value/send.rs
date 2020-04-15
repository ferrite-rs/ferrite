use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TyApp, Protocol };

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
  TyApp < X > for
  SendValue < T, A >
where
  A : TyApp < X >,
{
  type Applied =
    SendValue <
      T,
      A :: Applied
    >;
}
