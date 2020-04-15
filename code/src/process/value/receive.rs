use async_std::sync::{ Sender };

use crate::base as base;

use base::{ TyApp, Protocol };

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
  TyApp < A > for
  ReceiveValue < T, P >
where
  P : TyApp < A >,
{
  type Applied =
    ReceiveValue <
      T,
      P :: Applied
    >;
}
