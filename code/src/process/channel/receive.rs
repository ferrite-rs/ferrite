use async_std::sync::{ Sender, Receiver };

use crate::base as base;

use base::{ TyApp, Protocol };

pub struct ReceiveChannel
  < P, Q >
( pub (crate)
  Sender < (
    Receiver < P >,
    Sender < Q >
  ) >
);

impl
  < P, Q >
  Protocol for
  ReceiveChannel < P, Q >
where
  P: Protocol,
  Q: Protocol
{ }

impl < A, P, Q >
  TyApp < A > for
  ReceiveChannel < P, Q >
where
  P : TyApp < A >,
  Q : TyApp < A >,
{
  type Applied =
    ReceiveChannel <
      P :: Applied,
      Q :: Applied
    >;
}
