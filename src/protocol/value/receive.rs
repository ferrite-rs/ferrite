use async_std::sync::{ Sender };

use crate::base as base;

use base::{ TypeApp, Protocol };

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
  TypeApp < A > for
  ReceiveValue < T, P >
where
  P : TypeApp < A >,
{
  type Applied =
    ReceiveValue <
      T,
      P :: Applied
    >;
}
