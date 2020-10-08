use async_std::sync::{ Sender };

use crate::base as base;

use base::{ RecApp, Protocol };

pub struct ReceiveValue
  < T, P >
( pub (crate)
  Sender < (
    T,
    Sender < P >
  ) >
);

impl
  < T, P >
  Protocol for
  ReceiveValue < T, P >
where
  T : Send + 'static,
  P : Protocol
{ }

impl < A, T, P >
  RecApp < A > for
  ReceiveValue < T, P >
where
  A : Send + 'static,
  T : Send + 'static,
  P : RecApp < A >,
{
  type Applied =
    ReceiveValue <
      T,
      P :: Applied
    >;
}
