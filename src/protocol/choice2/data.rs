use std::marker::PhantomData;

use crate::base::*;
use async_std::sync::{ Sender, Receiver };

pub enum Bottom {}

pub struct Const < X >
  ( PhantomData<X> );

impl
  < X, A >
  TypeApp < A >
  for Const < X >
{
  type Applied = X;
}

pub struct Merge < T1, T2 >
  ( PhantomData <( T1, T2 )> );

pub struct ReceiverApp {}

pub struct SenderApp {}

impl < P >
  TypeApp < P > for
  ReceiverApp
where
  P : Protocol
{
  type Applied = Receiver < P >;
}

impl < P >
  TypeApp < P > for
  SenderApp
where
  P : Protocol
{
  type Applied = Sender < P >;
}

impl < T >
  SumRow < T > for
  ()
{
  type Field = Bottom;
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
      type family ToRow self t :: Applied
 */
pub trait SumRow < T >
{
  type Field : Send;
}

pub trait Iso {
  type Canon;
}

pub trait SplitRow < T1, T2 >
  : SumRow < T1 >
  + SumRow < T2 >
  + SumRow < Merge < T1, T2 > >
{
  fn split_row
    ( row:
        < Self as
          SumRow <
            Merge < T1, T2 >
          >
        >::Field
    ) ->
      ( < Self
          as SumRow < T1 >
        > :: Field,
        < Self
          as SumRow < T2 >
        > :: Field,
      )
  ;
}

/*
  class
    (SumRow self)
    => IsoRow self where
      type family Canon self t :: Applied

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
    ( TypeApp t1, TypeApp t2 )
    => LiftField t1 t2 where
      liftField
        :: forall a
         . Apply t1 a
        -> Apply t2 a
 */
pub trait LiftField < T1, T2, A >
where
  T1 : TypeApp < A >,
  T2 : TypeApp < A >
{
  fn lift_field (
    field : T1 :: Applied
  ) ->
    T2 :: Applied;
}

pub trait LiftFieldBorrow < T1, T2, A >
where
  T1 : TypeApp < A >,
  T2 : TypeApp < A >
{
  fn lift_field_borrow (
    field : &T1 :: Applied
  ) ->
    T2 :: Applied;
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
pub trait FieldLifterApplied < Root >
{
  type Source;

  type Target;

  type Injected;
}

pub trait FieldLifter < Root, A >
  : FieldLifterApplied < Root >
where
  Self :: Source : TypeApp < A >,
  Self :: Target : TypeApp < A >,
  Self :: Injected : TypeApp < A >,
{
  fn lift_field (
    self,
    inject :
      impl Fn
        ( < Self :: Target
            as TypeApp < A >
          >:: Applied)
        -> Root
      + Send + 'static,
    field :
      < Self :: Source
        as TypeApp < A >
      > :: Applied
  ) ->
    < Self :: Injected
      as TypeApp < A >
    >:: Applied;
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
  F : FieldLifterApplied < Root >
{
  fn lift_sum (
    ctx: F,
    inject: impl Fn
      ( < Self as
          SumRow < F :: Target >
        > :: Field
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
    FieldLifterApplied <
      < Self as
        SumRow < Target >
      > :: Field,
      Target = Target
    >
{
  fn lift_sum3 (
    ctx: F,
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
    FieldLifterApplied <
      < Self as
        SumRow < Target >
      > :: Field,
      Target = Target
    >,
{
  fn lift_sum3 (
    ctx: F,
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
      ctx,
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
  T : TypeApp < A >
{
  fn elim_field (
    self,
    a : T :: Applied
  ) ->
    R
  ;
}

pub struct ElimConst {}

impl < X, A >
  ElimField <
    Const < X >,
    A,
    X
  >
  for ElimConst
{
  fn elim_field (
    self,
    x : X
  ) -> X
  { x }
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

pub trait Prism < R, T >
where
  R : SumRow < T >,
{
  type Elem;

  fn inject_elem (
    elem : Self::Elem
  ) ->
    R :: Field
  ;
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
  T : TypeApp < A >,
  T::Applied : Send,
{
  fn to_canon (
    row :
      Sum < T :: Applied, R :: Field >
  ) ->
      Sum < T :: Applied, R :: Field >
  {
    row
  }

  fn from_canon (
    row :
      Sum < T :: Applied, R :: Field >
  ) ->
    Sum < T :: Applied, R :: Field >
  {
    row
  }
}

impl < T1, T2, A >
  TypeApp < A >
  for Merge < T1, T2 >
where
  T1 : TypeApp < A >,
  T2 : TypeApp < A >,
{
  type Applied = ( T1::Applied, T2::Applied );
}

impl < T, A, R >
  SumRow < T > for
  ( A, R )
where
  T : TypeApp < A >,
  R : SumRow < T >,
  T::Applied : Send
{
  type Field =
    Sum <
      T :: Applied,
      R :: Field
    >;
}

impl < T1, T2 >
  SplitRow < T1, T2 >
  for ()
where
{
  fn split_row ( bottom: Bottom ) -> (Bottom, Bottom)
  {
    match bottom {}
  }
}

impl < T1, T2, A, R >
  SplitRow < T1, T2 >
  for ( A, R )
where
  T1 : TypeApp < A >,
  T2 : TypeApp < A >,
  R : SumRow < Merge < T1, T2 > >,
  R : SplitRow < T1, T2 >,
  T1::Applied : Send,
  T2::Applied : Send,
{
  fn split_row (
    row1 :
      < ( A, R ) as
        SumRow <
          Merge < T1, T2 >
        >
      > :: Field
  ) ->
    ( < ( A, R ) as
        SumRow < T1 >
      > :: Field,
      < ( A, R ) as
        SumRow < T2 >
      > :: Field
    )
  {
    match row1 {
      Sum::Inl ( (row1a, row1b) ) => {
        ( Sum::Inl(row1a), Sum::Inl(row1b) )
      },
      Sum::Inr ( row2 ) => {
        let (row2a, row2b) = R::split_row (row2);
        ( Sum::Inr(row2a), Sum::Inr(row2b) )
      }
    }
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
  T1 : TypeApp < A >,
  T2 : TypeApp < A >,
  R : IntersectSum < T1, T2 >,
  T1::Applied : Send,
  T2::Applied : Send,
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
          ( a1, a2 ) ) )
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
  T1 : TypeApp < A >,
  T2 : TypeApp < A >,
  F : LiftFieldBorrow < T1, T2, A >,
  B : LiftSumBorrow < T1, T2, F >,
  T1::Applied : Send,
  T2::Applied : Send,
{
  fn lift_sum_borrow (
    sum :
      &Sum <
        T1 :: Applied,
        < B as
          SumRow < T1 >
        > :: Field
      >
  ) ->
    Sum <
      T2 :: Applied,
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
  F : FieldLifterApplied < Root >
{
  fn lift_sum (
    _: F,
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
  F :: Source : TypeApp < A >,
  F :: Target : TypeApp < A >,
  F :: Injected : TypeApp < A >,
  < F :: Source
    as TypeApp < A >
  > :: Applied : Send,
  < F :: Target
    as TypeApp < A >
  > :: Applied : Send,
  < F :: Injected
    as TypeApp < A >
  > :: Applied : Send,
{
  fn lift_sum (
    ctx: F,
    inject1 :
      impl Fn
        ( Sum <
            < F::Target
              as TypeApp < A >
            > :: Applied,
            < B as
              SumRow < F::Target >
            > :: Field
          >
        ) ->
          Root
          + Send + 'static,
    sum :
      Sum <
        < F :: Source
          as TypeApp < A >
        > :: Applied,
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
              < F::Target
                as TypeApp < A >
              > :: Applied
          | ->
            Root
          {
            inject1 ( Sum::Inl (b) )
          };

        Sum :: Inl(
          F :: lift_field ( ctx, inject2, a )
        )
      },
      Sum::Inr(b) => {
        let inject2 =
          move |
            b :
              < B as
                SumRow < F::Target >
              > :: Field
          | ->
            Root
          {
            inject1 ( Sum::Inr (b) )
          };

        Sum::Inr (
          B :: lift_sum ( ctx, inject2, b )
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
  T : TypeApp < A >,
  B : ElimSum < T, F, R >,
  F : ElimField < T, A, R >,
  T::Applied : Send,
{
  fn elim_sum (
    f : F,
    row :
      Sum <
        T :: Applied,
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
  TypeApp < A > for
  Bottom
{
  type Applied = Bottom;
}

impl < X , A, B >
  TypeApp < X > for
  Sum < A, B >
where
  A : TypeApp < X >,
  B : TypeApp < X >,
{
  type Applied =
    Sum <
      A :: Applied,
      B :: Applied
    >;
}

impl < T, A, R >
  Prism < (A, R), T >
  for Z
where
  T : TypeApp < A >,
  R : SumRow < T >,
  T::Applied : Send
{
  type Elem = T::Applied;

  fn inject_elem (
    t: T::Applied
  ) ->
    Sum < T::Applied, R::Field >
  {
    Sum::Inl(t)
  }
}

impl < N, T, A, R >
  Prism < (A, R), T >
  for S<N>
where
  N : Prism < R, T >,
  R : SumRow < T >,
  T : TypeApp < A >,
  T::Applied : Send,
{
  type Elem = N::Elem;

  fn inject_elem (
    t: N::Elem
  ) ->
    Sum < T::Applied, R::Field >
  {
    Sum::Inr( N::inject_elem(t) )
  }
}
