use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TypeApp, Protocol };

use super::data::{ Choice, Either };

pub struct ExternalChoice
  < A, B >
( pub (crate)
  Box < dyn
    FnOnce(Choice) ->
      Either <
        Receiver < A >,
        Receiver < B >
      >
    + Send >
);

impl
  < P, Q >
  Protocol for
  ExternalChoice < P, Q >
where
  P: Protocol,
  Q: Protocol
{ }

impl < A, T, X, Y >
  TypeApp < A > for
  Box <
    dyn FnOnce (T) ->
      Either < X, Y >
    + Send
  >
where
  X : TypeApp < A >,
  Y : TypeApp < A >,
{
  type Applied =
    Box <
      dyn FnOnce (T) ->
        Either <
          X :: Applied,
          Y :: Applied
        >
      + Send
    >;
}

impl < A, P, Q >
  TypeApp < A > for
  ExternalChoice < P, Q >
where
  P : TypeApp < A >,
  Q : TypeApp < A >,
{
  type Applied =
  ExternalChoice <
    P :: Applied,
    Q :: Applied
  >;
}
