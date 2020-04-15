use async_std::sync::{ Receiver };

use crate::base as base;

use base::{ TyApp, Protocol };

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
  TyApp < A > for
  Box <
    dyn FnOnce (T) ->
      Either < X, Y >
    + Send
  >
where
  X : TyApp < A >,
  Y : TyApp < A >,
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
  TyApp < A > for
  ExternalChoice < P, Q >
where
  P : TyApp < A >,
  Q : TyApp < A >,
{
  type Applied =
  ExternalChoice <
    P :: Applied,
    Q :: Applied
  >;
}
