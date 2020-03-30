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
  pub field1 : T1 :: Type,
  pub field2 : T2 :: Type
}

pub enum Sum < A, B >
{
  Inl ( A ),
  Inr ( B ),
}

/*
  class
    (forall t . Send (Field self t))
    => SumRow self where
      type family ToRow self t :: Type
 */
pub trait SumRow < T >
{
  type Field : Send;
}

pub trait Iso {
  type Canon;
}

/*
  class
    (SumRow self)
    => IsoRow self where
      type family Canon self t :: Type

      toCanon
        :: forall t
         . ToRow self t
        -> Canon self t

      fromCanon
        :: forall t
         . Canon self t
        -> ToRow self t
 */
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

/*
  class
    ( TyApp t1, TyApp t2 )
    => LiftField t1 t2 where
      liftField
        :: forall a
         . Apply t1 a
        -> Apply t2 a
 */
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

pub trait LiftFieldBorrow < T1, T2, A >
where
  T1 : TyApp < A >,
  T2 : TyApp < A >
{
  fn lift_field_borrow (
    field : &T1 :: Type
  ) ->
    T2 :: Type;
}

pub trait LiftSumBorrow < T1, T2, F >
  : SumRow < T1 > + SumRow < T2 >
{
  fn lift_sum_borrow (
    sum :
      & < Self as
          SumRow < T1 >
        > :: Field
  ) ->
    < Self as
      SumRow < T2 >
    > :: Field;
}

/*
  class FieldLifter f where
    type family Source f root
    type family Target f root
    type family Root f root

    liftField
      :: forall root a
       . (Apply (Target f root) a -> root)
      -> Apply (Source f root) a
      -> Apply (Root f root) a
 */
pub trait FieldLifterType < Root >
{
  type Source;
  type Target;

  type Injected;
}

pub trait FieldLifter < Root, A >
  : FieldLifterType < Root >
where
  Self :: Source : TyApp < A >,
  Self :: Target : TyApp < A >,
  Self :: Injected : TyApp < A >,
{
  fn lift_field (
    inject :
      impl Fn
        ( < Self :: Target
            as TyApp < A >
          >:: Type)
        -> Root
      + Send + 'static,
    field :
      < Self :: Source
        as TyApp < A >
      > :: Type
  ) ->
    < Self :: Injected
      as TyApp < A >
    >:: Type;
}

/*
  class SumRow row
    => LiftSum row where
      liftSum
        :: forall f root
         . (FieldLifter f)
        => (ToRow row (Target f root) -> root)
        -> ToRow row (Source f root)
        -> ToRow row (Root f root)
 */
pub trait LiftSum2 < F, Root >
  : SumRow < F :: Source >
  + SumRow < F :: Target >
  + SumRow < F :: Injected >
where
  F : FieldLifterType < Root >
{
  fn lift_sum (
    inject: impl Fn
      ( < Self as
          SumRow < F :: Target >
        >:: Field
      ) ->
        Root
        + Send + 'static,
    sum :
      < Self as
        SumRow < F :: Source >
      > :: Field
  ) ->
    < Self as
      SumRow < F :: Injected >
    > :: Field;
}

pub trait LiftSum3 < F, Target >
  : SumRow < Target >
  + LiftSum2 < F,
      < Self as
        SumRow < Target >
      > :: Field
    >
  + Sized
where
  F :
    FieldLifterType <
      < Self as
        SumRow < Target >
      > :: Field,
      Target = Target
    >
{
  fn lift_sum3 (
    sum :
      < Self as
        SumRow < F :: Source >
      > :: Field
  ) ->
    < Self as
      SumRow < F :: Injected >
    > :: Field;
}

impl < A, F, Target >
  LiftSum3 < F, Target >
  for A
where
  A : Sized,
  A : SumRow < Target >,
  A : LiftSum2 < F,
        < A as
          SumRow < Target >
        > :: Field
      >,
  F :
    FieldLifterType <
      < Self as
        SumRow < Target >
      > :: Field,
      Target = Target
    >,
{
  fn lift_sum3 (
    sum :
      < Self as
        SumRow < F :: Source >
      > :: Field
  ) ->
    < Self as
      SumRow < F :: Injected >
    > :: Field
  {
    A::lift_sum (
      |x| { x },
      sum
    )
  }
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

impl Iso for () {
  type Canon = ();
}

impl < A, R >
  Iso for
  ( A, R )
where
  R : Iso < Canon = R >
{
  type Canon = ( A, R :: Canon );
}

impl < T >
  IsoRow < T >
  for ()
{
  fn to_canon (
    row : Bottom
  ) -> Bottom
  { row }

  fn from_canon (
    row : Bottom
  ) -> Bottom
  { row }
}

impl < T, A, R >
  IsoRow < T >
  for ( A, R )
where
  R : Iso < Canon = R >,
  R : SumRow < T >,
  T : TyApp < A >,
  T::Type : Send,
{
  fn to_canon (
    row :
      Sum < T :: Type, R :: Field >
  ) ->
      Sum < T :: Type, R :: Field >
  {
    row
  }

  fn from_canon (
    row :
      Sum < T :: Type, R :: Field >
  ) ->
    Sum < T :: Type, R :: Field >
  {
    row
  }
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
  type Type = Receiver < P :: Payload >;
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
  R : SumRow < T >,
  T::Type : Send
{
  type Field =
    Sum <
      T :: Type,
      R :: Field
    >;
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
  R : IntersectSum < T1, T2 >,
  T1::Type : Send,
  T2::Type : Send,
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
  LiftSumBorrow < T1, T2, F > for
  ()
{
  fn lift_sum_borrow ( bot : &Bottom ) -> Bottom
  { match *bot {} }
}

impl < T1, T2, F, A, B >
  LiftSumBorrow < T1, T2, F > for
  (A, B)
where
  T1 : TyApp < A >,
  T2 : TyApp < A >,
  F : LiftFieldBorrow < T1, T2, A >,
  B : LiftSumBorrow < T1, T2, F >,
  T1::Type : Send,
  T2::Type : Send,
{
  fn lift_sum_borrow (
    sum :
      &Sum <
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
          F :: lift_field_borrow ( a )
        )
      },
      Sum::Inr(b) => {
        Sum::Inr (
          B :: lift_sum_borrow ( b )
        )
      }
    }
  }
}

impl < F, Root >
  LiftSum2 < F, Root > for
  ()
where
  F : FieldLifterType < Root >
{
  fn lift_sum (
    _ : impl Fn ( Bottom ) -> Root,
    bot : Bottom
  ) -> Bottom
  {
    match bot {}
  }
}

impl < F, Root, A, B >
  LiftSum2 < F, Root > for
  (A, B)
where
  F : FieldLifter < Root, A >,
  B : LiftSum2 < F, Root >,
  F :: Source : TyApp < A >,
  F :: Target : TyApp < A >,
  F :: Injected : TyApp < A >,
  < F :: Source
    as TyApp < A >
  > :: Type : Send,
  < F :: Target
    as TyApp < A >
  > :: Type : Send,
  < F :: Injected
    as TyApp < A >
  > :: Type : Send,
{
  fn lift_sum (
    inject1 :
      impl Fn
        ( Sum <
            < F :: Target
              as TyApp < A >
            > :: Type,
            < B as
              SumRow < F :: Target >
            > :: Field
          >
        ) ->
          Root
          + Send + 'static,
    sum :
      Sum <
        < F :: Source
          as TyApp < A >
        > :: Type,
        < B as
          SumRow < F :: Source >
        > :: Field
      >
  ) ->
    < Self as
      SumRow < F :: Injected >
    > :: Field
  {
    match sum {
      Sum::Inl(a) => {
        let inject2 =
          move |
            b :
              < F :: Target
                as TyApp < A >
              > :: Type
          | ->
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
                SumRow < F :: Target >
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
  T::Type : Send,
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
