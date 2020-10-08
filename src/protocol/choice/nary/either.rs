use crate::base::{
  Z,
  S,
};

use crate::protocol::choice::nary::*;

pub type Either < A, B > = ( A, ( B, () ) );

pub type EitherRow < A, B > =
  Sum < A,
    Sum < B, Bottom >
  >;

#[allow(non_upper_case_globals)]
pub const LeftLabel : ChoiceSelector < Z > =
  < ChoiceSelector < Z > >::new();

#[allow(non_upper_case_globals)]
pub const RightLabel : ChoiceSelector < S<Z> > =
  < ChoiceSelector < S < Z > > >::new();

pub enum EitherField < A, B > {
  Left ( A ),
  Right ( B )
}

pub use EitherField::Left as Left;
pub use EitherField::Right as Right;

impl < A, B >
  ExtractRow < EitherField < A, B > >
  for Sum < A,
    Sum < B, Bottom >
  >
{
  fn extract (self)
    -> EitherField < A, B >
  {
    extract_either(self)
  }
}

pub fn extract_either < A, B >
  ( row :
      Sum < A,
        Sum < B, Bottom >
      >
  ) -> EitherField < A, B >
{
  match row {
    Sum::Inl ( a ) => { Left ( a ) }
    Sum::Inr ( Sum::Inl ( b ) ) => { Right ( b ) }
    Sum::Inr ( Sum::Inr ( bot ) ) => { match bot {} }
  }
}
