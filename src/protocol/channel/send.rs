use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ RecApp, Protocol };

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
  RecApp < A > for
  SendChannel < P, Q >
where
  P : RecApp < A >,
  Q : RecApp < A >,
{
  type Applied =
  SendChannel <
      P :: Applied,
      Q :: Applied
    >;
}
