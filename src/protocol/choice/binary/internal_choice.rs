use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TypeApp, Protocol };

use super::data::{ Either };

/*
  data InternalChoice p q = InternalChoice
 */
pub struct InternalChoice < A, B >
( pub (crate)
  Either <
    Receiver < A >,
    Receiver < B >
  >
);

impl
  < P, Q >
  Protocol for
  InternalChoice < P, Q >
where
  P: Protocol,
  Q: Protocol
{ }

impl < A, X, Y >
  TypeApp < A > for
  InternalChoice < X, Y >
where
  X : TypeApp < A >,
  Y : TypeApp < A >,
{
  type Applied =
    InternalChoice <
      X :: Applied,
      Y :: Applied
    >;
}
