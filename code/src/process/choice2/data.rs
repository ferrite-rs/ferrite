use std::marker::PhantomData;

use crate::base::*;
use async_std::sync::{ Receiver };

pub enum Bottom {}

pub struct Merge < T1, T2 >
  ( PhantomData <( T1, T2 )> );

pub struct ReceiverCon {}

pub struct MergeField < T1, T2, A >
where
  T1 : TyCon < A >,
  T2 : TyCon < A >,
{
  field1 : T1 :: Type,
  field2 : T2 :: Type
}

impl < T1, T2, A >
  TyCon < A >
  for Merge < T1, T2 >
where
  T1 : TyCon < A >,
  T2 : TyCon < A >,
{
  type Type = MergeField < T1, T2, A >;
}

impl < P >
  TyCon < P > for
  ReceiverCon
where
  P : Process
{
  type Type = Receiver < P :: Value >;
}

pub enum Sum < A, B >
{
  Inl ( A ),
  Inr ( B ),
}

pub trait SumRow < T >
{
  type Field;
}

impl < T >
  SumRow < T > for
  ()
{
  type Field = Bottom;
}

impl < T, A, R >
  SumRow < T > for
  ( A, R )
where
  T : TyCon < A >,
  R : SumRow < T >
{
  type Field =
    Sum <
      T :: Type,
      R :: Field
    >;
}

pub trait Iso {
  type Canon;
}

pub trait IsoRow < T >
  : Iso + SumRow < T >
where
  Self :: Canon : SumRow < T >
{
  fn to_canon (
    row : Self :: Field
  ) ->
    < Self :: Canon
      as SumRow < T >
    > :: Field
  ;

  fn from_canon (
    row :
      < Self :: Canon
        as SumRow < T >
      > :: Field
  ) ->
    Self :: Field
  ;
}

pub trait LiftField < T1, T2, A >
where
  T1 : TyCon < A >,
  T2 : TyCon < A >
{
  fn lift_field (
    field : T1 :: Type
  ) ->
    T2 :: Type;
}

pub trait LiftSum < T1, T2, F >
  : SumRow < T1 > + SumRow < T2 >
{
  fn lift_sum (
    sum :
      < Self as
        SumRow < T1 >
      > :: Field
  ) ->
    < Self as
      SumRow < T2 >
    > :: Field;
}

pub trait IntersectSum < T1, T2 >
  : SumRow < T1 >
  + SumRow < T2 >
  + SumRow < Merge < T1, T2 > >
{
  fn intersect (
    row1 :
      < Self as
        SumRow < T1 >
      > :: Field,
    row2 :
      < Self as
        SumRow < T2 >
      > :: Field,
  ) ->
    Option <
      < Self as
        SumRow < Merge < T1, T2 > >
      > :: Field
    >
  ;
}

pub trait ElimField < T, A, R >
where
  T : TyCon < A >
{
  fn elim_field (
    a : T :: Type
  ) ->
    R
  ;
}

pub trait ElimSum < T, F, R >
  : SumRow < T >
{
  fn elim_sum (
    row : Self :: Field
  ) ->
    R;
}

pub trait IntroField < T, A >
where
  T : TyCon < A >
{
  fn intro_field () ->
    T :: Type
  ;
}

pub trait IntroSum < N, T, F >
  : SumRow < T >
where
  N : Nat
{
  fn intro_sum () ->
    Self :: Field
  ;
}

impl < T1, T2 >
  IntersectSum < T1, T2 > for
  ()
{
  fn intersect (
    row1 : Bottom,
    _ : Bottom,
  ) ->
    Option < Bottom >
  {
    match row1 {}
  }
}

impl < T1, T2, A, R >
  IntersectSum < T1, T2 > for
  ( A, R )
where
  T1 : TyCon < A >,
  T2 : TyCon < A >,
  R : IntersectSum < T1, T2 >
{
  fn intersect (
    row1 :
      < Self as
        SumRow < T1 >
      > :: Field,
    row2 :
      < Self as
        SumRow < T2 >
      > :: Field,
  ) ->
    Option <
      < Self as
        SumRow < Merge < T1, T2 > >
      > :: Field
    >
  {
    match (row1, row2) {
      ( Sum::Inl(a1), Sum::Inl(a2) ) => {
        Some ( Sum::Inl (
          MergeField {
            field1 : a1,
            field2 : a2
          }
        ) )
      },
      ( Sum::Inr(r1), Sum::Inr(r2) ) => {
        R :: intersect ( r1, r2 )
          .map(| x | { Sum::Inr(x) })
      },
      _ => {
        None
      }
    }
  }
}

impl < T1, T2, F >
  LiftSum < T1, T2, F > for
  ()
{
  fn lift_sum ( bot : Bottom ) -> Bottom
  {
    match bot {}
  }
}

impl < T1, T2, F, A, B >
  LiftSum < T1, T2, F > for
  (A, B)
where
  T1 : TyCon < A >,
  T2 : TyCon < A >,
  F : LiftField < T1, T2, A >,
  B : LiftSum < T1, T2, F >,
{
  fn lift_sum (
    sum :
      Sum <
        T1 :: Type,
        < B as
          SumRow < T1 >
        > :: Field
      >
  ) ->
    Sum <
      T2 :: Type,
      < B as
        SumRow < T2 >
      > :: Field
    >
  {
    match sum {
      Sum::Inl(a) => {
        Sum::Inl (
          F :: lift_field ( a )
        )
      },
      Sum::Inr(b) => {
        Sum::Inr (
          B :: lift_sum ( b )
        )
      }
    }
  }
}

impl < T, F, R >
  ElimSum < T, F, R > for
  ()
{
  fn elim_sum ( row : Bottom ) -> R {
    match row {}
  }
}

impl < A, B, T, F, R >
  ElimSum < T, F, R > for
  (A, B)
where
  T : TyCon < A >,
  B : ElimSum < T, F, R >,
  F : ElimField < T, A, R >,
{
  fn elim_sum (
    row :
      Sum <
        T :: Type,
        B :: Field
      >
  ) ->
    R
  {
    match row {
      Sum::Inl(a) => {
        F :: elim_field ( a )
      },
      Sum::Inr(b) => {
        B :: elim_sum ( b )
      }
    }
  }
}

impl < T, F, A, B >
  IntroSum < Z, T, F > for
  ( A, B )
where
  T : TyCon < A >,
  B : SumRow < T >,
  F : IntroField < T, A >
{
  fn intro_sum () ->
    Sum <
      T :: Type,
      B :: Field
    >
  {
    Sum::Inl( F :: intro_field () )
  }
}

impl < N, T, F, A, B >
  IntroSum < Succ < N >, T, F > for
  ( A, B )
where
  N : Nat,
  T : TyCon < A >,
  B : SumRow < T >,
  B : IntroSum < N, T, F >
{
  fn intro_sum () ->
    Sum <
      T :: Type,
      B :: Field
    >
  {
    Sum :: Inr ( B :: intro_sum () )
  }
}

impl < A >
  TyCon < A > for
  Bottom
{
  type Type = Bottom;
}

impl < X , A, B >
  TyCon < X > for
  Sum < A, B >
where
  A : TyCon < X >,
  B : TyCon < X >,
{
  type Type =
    Sum <
      A :: Type,
      B :: Type
    >;
}
