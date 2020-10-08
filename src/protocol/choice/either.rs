
use crate::functional::nat::*;
use crate::functional::row::*;

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

pub enum EitherChoice < A, B > {
  Left ( A ),
  Right ( B )
}

pub use EitherChoice::{
  Left, Right
};

impl < A, B >
  ExtractRow < EitherChoice < A, B > >
  for Sum < A,
    Sum < B, Bottom >
  >
{
  fn extract (self)
    -> EitherChoice < A, B >
  {
    match self {
      Sum::Inl ( a ) => { Left ( a ) }
      Sum::Inr ( Sum::Inl ( b ) ) => { Right ( b ) }
      Sum::Inr ( Sum::Inr ( bot ) ) => { match bot {} }
    }
  }
}
