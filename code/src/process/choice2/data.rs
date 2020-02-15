use std::marker::PhantomData;

use crate::base::*;
use async_std::sync::{ Receiver };

pub enum Bottom {}

pub struct Merge < T1, T2 >
  ( PhantomData <( T1, T2 )> );

pub struct ReceiverCon {}

pub struct MergeField < T1, T2, A >
where
  T1 : TyApp < A >,
  T2 : TyApp < A >,
{
  field1 : T1 :: Type,
  field2 : T2 :: Type
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

pub trait Iso {
  type Canon;
}

pub trait Inject < A > {
  fn inject ( a : A ) -> Self;
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
  T1 : TyApp < A >,
  T2 : TyApp < A >
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

pub trait LiftField2 < T1, T2, A, T3, Root >
where
  T1 : TyApp < A >,
  T2 : TyApp < A >,
  T3 : TyApp < A >,
{
  fn lift_field (
    inject : impl Fn (T3 :: Type) -> Root + Send + 'static,
    field : T1 :: Type
  ) ->
    T2 :: Type;
}

pub trait LiftSum2 < T1, T2, F, T3, Root >
  : SumRow < T1 >
  + SumRow < T2 >
  + SumRow < T3 >
{
  fn lift_sum (
    inject:
      impl Fn
        ( < Self as SumRow < T3 > >:: Field ) ->
          Root
          + Send + 'static,
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
  T : TyApp < A >
{
  fn elim_field (
    self,
    a : T :: Type
  ) ->
    R
  ;
}

pub trait ElimSum < T, F, R >
  : SumRow < T >
{
  fn elim_sum (
    f : F,
    row : Self :: Field
  ) ->
    R;
}

pub trait IntroField < T, A >
where
  T : TyApp < A >
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

impl < T1, T2, A >
  TyApp < A >
  for Merge < T1, T2 >
where
  T1 : TyApp < A >,
  T2 : TyApp < A >,
{
  type Type = MergeField < T1, T2, A >;
}

impl < P >
  TyApp < P > for
  ReceiverCon
where
  P : Protocol
{
  type Type = Receiver < P :: Value >;
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
  T : TyApp < A >,
  R : SumRow < T >
{
  type Field =
    Sum <
      T :: Type,
      R :: Field
    >;
}

impl Inject < Bottom > for Bottom {
  fn inject ( a : Bottom ) -> Bottom {
    a
  }
}

impl < A, B, C >
  Inject < C > for
  Sum < A, B >
where
  B : Inject < C >
{
  fn inject ( c : C ) -> Sum < A, B >
  {
    Sum::Inr ( B::inject(c) )
  }
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
  T1 : TyApp < A >,
  T2 : TyApp < A >,
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
  T1 : TyApp < A >,
  T2 : TyApp < A >,
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


impl < T1, T2, F, T3, Root >
  LiftSum2 < T1, T2, F, T3, Root > for
  ()
{
  fn lift_sum (
    inject : impl Fn ( Bottom ) -> Root,
    bot : Bottom
  ) -> Bottom
  {
    match bot {}
  }
}

impl < T1, T2, F, T3, Root, A, B >
  LiftSum2 < T1, T2, F, T3, Root > for
  (A, B)
where
  T1 : TyApp < A >,
  T2 : TyApp < A >,
  T3 : TyApp < A >,
  F : LiftField2 < T1, T2, A, T3, Root >,
  B : LiftSum2 < T1, T2, F, T3, Root >,
{
  fn lift_sum (
    inject1 :
      impl Fn
        ( Sum <
            T3 :: Type,
            < B as
              SumRow < T3 >
            > :: Field
          >
        ) ->
          Root
          + Send + 'static,
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
        let inject2 =
          move | b : T3 :: Type | ->
            Root
          {
            inject1 ( Sum::Inl (b) )
          };

        Sum :: Inl(
          F :: lift_field ( inject2, a )
        )
      },
      Sum::Inr(b) => {
        let inject2 =
          move |
            b :
              < B as
                SumRow < T3 >
              > :: Field
          | ->
            Root
          {
            inject1 ( Sum::Inr (b) )
          };

        Sum::Inr (
          B :: lift_sum ( inject2, b )
        )
      }
    }
  }
}


impl < T, F, R >
  ElimSum < T, F, R > for
  ()
{
  fn elim_sum ( _ : F, row : Bottom ) -> R {
    match row {}
  }
}

impl < A, B, T, F, R >
  ElimSum < T, F, R > for
  (A, B)
where
  T : TyApp < A >,
  B : ElimSum < T, F, R >,
  F : ElimField < T, A, R >,
{
  fn elim_sum (
    f : F,
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
        f.elim_field ( a )
      },
      Sum::Inr(b) => {
        B :: elim_sum ( f, b )
      }
    }
  }
}

impl < T, F, A, B >
  IntroSum < Z, T, F > for
  ( A, B )
where
  T : TyApp < A >,
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
  IntroSum < S < N >, T, F > for
  ( A, B )
where
  N : Nat,
  T : TyApp < A >,
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
  TyApp < A > for
  Bottom
{
  type Type = Bottom;
}

impl < X , A, B >
  TyApp < X > for
  Sum < A, B >
where
  A : TyApp < X >,
  B : TyApp < X >,
{
  type Type =
    Sum <
      A :: Type,
      B :: Type
    >;
}
