use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TypeApp, Protocol };

pub struct SendChannel < A, B >
( pub (crate) Receiver < A >,
  pub (crate) Receiver < B >
);

impl < P, Q >
  Protocol for
  SendChannel < P, Q >
where
  P: Protocol,
  Q: Protocol
{ }

impl < A, P, Q >
  TypeApp < A > for
  SendChannel < P, Q >
where
  P : TypeApp < A >,
  Q : TypeApp < A >,
{
  type Applied =
  SendChannel <
      P :: Applied,
      Q :: Applied
    >;
}
