use crate::protocol::choice::nary::*;
use crate::protocol::choice::nary::either::*;
use crate::session::choice::nary;
use crate::session::choice::nary::external_choice_offer as choice;

use crate::base::{
  Protocol,
  Context,
  ContextLens,
  PartialSession,
};

use crate::protocol::choice::binary:: {
  ExternalChoice
};

pub type ContSum < C, A, B > =
  AppliedSum <
    Either < A, B >,
    SessionApp < C >
  >
;

pub type InjectCont < C, A, B > =
  < Either < A, B >
    as WrapRow <
      choice::RootCont <
        C,
        Either < A, B >
      >
    >
  > :: Unwrapped
;

pub fn offer_choice < C, A, B >
  ( cont : impl FnOnce
      ( InjectCont < C, A, B > )
      -> ContSum < C, A, B >
      + Send + 'static
  ) ->
    PartialSession <
      C,
      ExternalChoice < A, B >
    >
where
  A : Protocol,
  B : Protocol,
  C : Context
{
  nary::offer_choice ( cont )
}

pub fn choose_left
  < N, C, A1, A2, B >
  ( n : N,
    cont:
      PartialSession <
        N :: Target,
        B
      >
  ) ->
    PartialSession <
      C, B
    >
where
  C : Context,
  A1 : Protocol,
  A2 : Protocol,
  B : Protocol,
  N :
    ContextLens <
      C,
      ExternalChoice < A1, A2 >,
      A1
    >
{
  nary::choose ( n, LeftChoice, cont )
}

/*
           cont ::  Δ, Q, Δ'  ⊢ S
  =========================================
    choose_right(cont) :: Δ, P & Q, Δ' ⊢ S
 */
pub fn choose_right
  < N, C, A1, A2, B >
  ( n : N,
    cont:
      PartialSession <
        N :: Target,
        B
      >
  ) ->
    PartialSession < C, B >
where
  C : Context,
  A1 : Protocol,
  A2 : Protocol,
  B : Protocol,
  N :
    ContextLens <
      C,
      ExternalChoice < A1, A2 >,
      A2
    >
{
  nary::choose ( n, RightChoice, cont )
}
