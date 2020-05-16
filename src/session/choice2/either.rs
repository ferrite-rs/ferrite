pub use crate::base::{
  TypeApp
};

pub use crate::context::*;
pub use crate::protocol::choice2::*;

pub type EitherField < A, B, T > =
  Either <
    < T as TypeApp<A> > :: Applied,
    < T as TypeApp<B> > :: Applied
  >;

pub enum Either < A, B > {
  Left ( A ),
  Right ( B ),
}

impl < T, A, B >
  SumRow < T > for
  Either < A, B >
where
  T : TypeApp < A >,
  T : TypeApp < B >,
  < T as TypeApp<A> > :: Applied : Send,
  < T as TypeApp<B> > :: Applied : Send,
{
  type Field = Either <
    < T as TypeApp<A> > :: Applied,
    < T as TypeApp<B> > :: Applied
  >;
}

impl < A, B >
  Iso
  for Either < A, B >
{
  type Canon = ( A, ( B, () ) );
}

impl < A, B, T >
  IsoRow < T >
  for Either < A, B >
where
  T : TypeApp < A >,
  T : TypeApp < B >,
  < T as TypeApp<A> > :: Applied : Send,
  < T as TypeApp<B> > :: Applied : Send,
{
  fn to_canon (
    row : EitherField < A, B, T >
  ) ->
    < Self :: Canon
      as SumRow < T >
    > :: Field
  {
    match row {
      Either::Left ( a ) => {
        Sum::Inl ( a )
      },
      Either::Right ( a ) => {
        Sum::Inr (
          Sum::Inl ( a ) )
      }
    }
  }

  fn from_canon (
    row :
      < Self :: Canon
        as SumRow < T >
      > :: Field
  ) ->
    EitherField < A, B, T >
  {
    match row {
      Sum::Inl ( a ) => {
        Either::Left( a )
      },
      Sum::Inr ( row2 ) => {
        match row2 {
          Sum::Inl ( a ) => {
            Either::Right( a )
          },
          Sum::Inr ( bot ) => {
            match bot {}
          }
        }
      }
    }
  }
}
