use crate::protocol::choice::nary::*;
use crate::protocol::choice::nary::either::*;
use crate::session::choice::nary;
use crate::session::choice::nary::internal_choice_case as choice;

use crate::protocol::choice::binary::{
  InternalChoice
};

use crate::base::{
  Nat,
  PartialSession,
  Protocol,
  Context,
  Empty,
  ContextLens,
};

/*
  Additive Disjuction / Internal Choice

  Right Rule (Session)

            cont :: Δ ⊢ P
  =================================
    offer_left(cont) :: Δ ⊢ P ⊕ Q

  offerLeft
    :: forall ctx p q
       ( Protocol p
       , Protocol q
       , Context ctx
       )
    =>  PartialSession ctx p
    ->  PartialSession ctx (InternalChoice p q)
 */
pub fn offer_left
  < C, A, B >
  ( cont:  PartialSession < C, A >
  ) ->
    PartialSession < C,
      InternalChoice < A, B >
    >
where
  A : Protocol,
  B : Protocol,
  C : Context
{
  nary::offer_case ( LeftChoice, cont )
}

pub fn offer_right
  < C, A, B >
  ( cont:  PartialSession < C, B > )
  -> PartialSession < C, InternalChoice < A, B > >
where
  A : Protocol,
  B : Protocol,
  C : Context,
{
  nary::offer_case ( RightChoice, cont )
}

/*
  Additive Disjuction / Internal Choice

  Left Rule (Client)

      cont_builder(Left)  :: Δ, P, Δ' ⊢ S
      cont_builder(Right) :: Δ, Q, Δ' ⊢ S
  ===========================================
    case(cont_builder) :: Δ, P ⊕ Q, Δ' ⊢ S
 */

pub type ContSum < N, C, A1, A2, B > =
  Sum <
    PartialSession <
      < N as
        ContextLens <
          C,
          InternalChoice < A1, A2 >,
          A1
        >
      > :: Target,
      B
    >,
    Sum <
      PartialSession <
        < N as
          ContextLens <
            C,
            InternalChoice < A1, A2 >,
            A2
          >
        > :: Target,
        B
      >,
      Bottom
    >
  >;

pub type InjectCont < N, C, A1, A2, B > =
  EitherRow <
    choice::InjectSession <
      N, C, A1, B,
      Either < A1, A2 >,
      ContSum < N, C, A1, A2, B >
    >,
    choice::InjectSession <
      N, C, A2, B,
      Either < A1, A2 >,
      ContSum < N, C, A1, A2, B >
    >,
  >;

pub fn case
  < N, C, D, A1, A2, B >
  ( n : N,
    cont : impl
      FnOnce (
        InjectCont < N, C, A1, A2, B >
      ) ->
        ContSum < N, C, A1, A2, B >
      + Send + 'static
  ) ->
    PartialSession < C, B >
where
  N : Nat,
  C : Context,
  D : Context,
  A1 : Protocol,
  A2 : Protocol,
  B : Protocol,
  N :
    ContextLens <
      C,
      InternalChoice < A1, A2 >,
      A1,
      Deleted = D
    >,
  N :
    ContextLens <
      C,
      InternalChoice < A1, A2 >,
      A2,
      Deleted = D
    >,
  N :
    ContextLens <
      C,
      InternalChoice < A1, A2 >,
      Empty,
      Deleted = D
    >
{
  nary::case ( n, cont )
}
