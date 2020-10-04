use crate::protocol::choice::nary::*;
use crate::protocol::choice::nary::either::*;
use crate::session::choice::nary;
use crate::session::choice::nary::external_choice_offer as choice;

use crate::base::{
  Applied,
  Protocol,
  Context,
  ContextLens,
  PartialSession,
};

use crate::protocol::choice::binary:: {
  ExternalChoice
};

pub type ContSum < C, A, B > =
  Sum <
    PartialSession < C, A >,
    Sum <
      PartialSession < C, B >,
      Bottom
    >
  >;

pub type InjectCont < C, A, B > =
  EitherRow <
    choice::InjectSession <
      ContSum < C, A, B >,
      C,
      A
    >,
    choice::InjectSession <
      ContSum < C, A, B >,
      C,
      B
    >,
  >;

type ContSumWrapped < C, A, B > =
  Sum <
    Applied < SessionApp < C >, A >,
    Sum <
    Applied < SessionApp < C >, B >,
      Bottom
    >
  >
;

// fn wrap_cont_sum < C, A, B >
//   ( row1: ContSum < C, A, B > )
//   -> ContSumWrapped < C, A, B >
// where
//   A: Protocol,
//   B: Protocol,
//   C: Context,
// {
//   match row1 {
//     Sum::Inl( row2 ) => {
//       todo!()
//     }
//     Sum::Inr( row2 ) => {
//       match row2 {
//         Sum::Inl( row3 ) => {
//           todo!()
//         }
//         Sum::Inr( row3 ) => {
//           todo!()
//         }
//       }
//     }
//   }
// }

pub fn offer_choice < C, A, B >
  ( cont : impl FnOnce
      ( InjectCont < C, A, B > )
      -> ContSumWrapped < C, A, B >
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
  // nary::offer_choice ( cont )
  todo!()
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
