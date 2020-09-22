use std::mem::transmute;
use std::marker::PhantomData;

use crate::base::*;
use async_std::sync::{ Sender, Receiver };

pub trait RowCon {}

pub trait SumRow < F: TyCon > : RowCon
{ type Field; }

pub enum Bottom {}

impl < A, R >
  RowCon
  for (A, R)
where
  R: RowCon
{}

impl RowCon for () {}
impl RowCon for Bottom {}

impl < F, A, R >
  SumRow < F > for
  ( A, R )
where
  F: TyCon,
{
  type Field =
    Sum <
      Applied < F, A >,
      AppliedSum < R, F >,
    >;
}

impl < F >
  SumRow < F > for
  ()
where
  F: TyCon
{
  type Field = Bottom;
}

impl < F >
  SumRow < F > for
  Bottom
where
  F: TyCon
{
  type Field = Bottom;
}

pub struct Merge < T1, T2 >
  ( PhantomData <( T1, T2 )> );

pub struct ReceiverApp {}
pub struct SenderApp {}

impl TyCon for ReceiverApp {}
impl TyCon for SenderApp {}

impl < P > TypeApp < P > for ReceiverApp
{ type Applied = Receiver < P >; }

impl < P > TypeApp < P > for SenderApp
{ type Applied = Sender < P >; }

pub enum Sum < A, B >
{
  Inl ( A ),
  Inr ( B ),
}

pub struct AppliedSum < Row, F >
where
  Row: RowCon,
  F: TyCon,
{ wrapped: Box < (Row, F) > }

impl < Row, F >
  AppliedSum < Row, F >
where
  F: TyCon,
  Row: SumRow < F >,
{ pub fn unwrap
    ( self )
    -> Row::Field
  { unsafe {
      let unwrapped : Box < Row::Field > =
        transmute( self.wrapped );
      *unwrapped
    } }
}

pub fn unwrap_sum
  < Row, F >
  ( wrapped: AppliedSum < Row, F > )
  -> Row::Field
where
  F: TyCon,
  Row: SumRow < F >,
{
  unsafe {
    let unwrapped : Box < Row::Field > =
      transmute( wrapped.wrapped );
    *unwrapped
  }
}

pub fn wrap_sum < Row, F >
  ( applied: Row::Field )
  -> AppliedSum < Row, F >
where
  F: TyCon,
  Row: SumRow < F >,
{ unsafe {
    let wrapped : Box < ( Row, F ) > =
      transmute( Box::new( applied ) );
    AppliedSum { wrapped: wrapped }
  } }

pub fn absurd < F, A >
  ( row1: AppliedSum < (), F > )
  -> A
where
  F: TyCon,
{
  let row2 = row1.unwrap();
  match row2 {}
}

pub trait SplitRow : Sized + RowCon
{
  fn split_row
    < F1, F2 >
    ( row:
        AppliedSum <
          Self,
          Merge < F1, F2 >
        >
    ) ->
      ( AppliedSum < Self, F1 >,
        AppliedSum < Self, F2 >
      )
  where
    F1: TyCon,
    F2: TyCon,
  ;
}

pub trait SumFunctorBorrow
  : RowCon
{
  fn lift_sum_borrow
     < T, F1, F2 >
    ( sum: &AppliedSum < Self, F1 > )
    -> AppliedSum < Self, F2 >
  where
    T: NaturalTransformationBorrow < F1, F2 >
  ;
}

pub trait FieldLifter < Root >
{
  type SourceF: TyCon;
  type TargetF: TyCon;
  type InjectF: TyCon;

  fn lift_field < A >
    ( self,
      inject:
        impl Fn
          ( Applied < Self::TargetF, A > )
          -> Root
        + Send + 'static,
      row:
        Applied < Self::SourceF, A >
    ) ->
      Applied < Self::InjectF, A >
    ;
}

pub trait SumFunctorInject
  : RowCon
{
  fn lift_sum_inject
    < L, Root >
    ( ctx: L,
      inject:
        impl Fn
          ( AppliedSum < Self, L::TargetF > )
          -> Root
          + Send + 'static,
      sum: AppliedSum < Self, L::SourceF >,
    )
  where
    L: FieldLifter < Root >
  ;
}

pub trait IntersectSum : RowCon
{
  fn intersect_sum
    < F1, F2 >
    ( row1: AppliedSum < Self, F1 >,
      row2: AppliedSum < Self, F2 >,
    ) ->
      Option <
        AppliedSum <
          Self,
          Merge < F1, F2 >
        >
      >
  where
    F1: TyCon,
    F2: TyCon,
  ;
}

pub trait ElimField < F, R >
where
  F : TyCon
{
  fn elim_field < A >
    ( self,
      a : Applied < F, A >
    ) ->
      R
  ;
}

pub struct ElimConst {}

impl < X >
  ElimField <
    Const < X >,
    X
  >
  for ElimConst
{
  fn elim_field < A >
    ( self,
      x : X
    ) -> X
  { x }
}

pub trait ElimSum : RowCon
{
  fn elim_sum
    < F, E, R >
    ( elim_field: E,
      row: AppliedSum < Self, F >
    ) ->
      R
  where
    F: TyCon,
    E: ElimField < F, R >,
  ;
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

  fn extract_elem (
    row : R::Field
  ) ->
    Option < Self::Elem >
  ;
}

impl < T1, T2 > TyCon for Merge < T1, T2 > {}

impl < T1, T2, A >
  TypeApp < A >
  for Merge < T1, T2 >
where
  T1 : TyCon,
  T2 : TyCon,
{
  type Applied =
    ( Applied < T1, A >,
      Applied < T2, A >,
    );
}

impl
  SplitRow
  for ()
where
{
  fn split_row
    < F1, F2 >
    ( row1:
        AppliedSum <
          Self,
          Merge < F1, F2 >
        >
    ) ->
      ( AppliedSum < Self, F1 >,
        AppliedSum < Self, F2 >
      )
  where
    F1: TyCon,
    F2: TyCon,
  {
    match row1 {}
  }
}

impl < A, R >
  SplitRow
  for ( A, R )
where
  R : SplitRow,
{
  fn split_row
    < F1, F2 >
    ( row1:
        AppliedSum <
          Self,
          Merge < F1, F2 >
        >
    ) ->
      ( AppliedSum < Self, F1 >,
        AppliedSum < Self, F2 >
      )
  where
    F1: TyCon,
    F2: TyCon,
  {
    let row2 = row1.unwrap();

    match row2 {
      Sum::Inl ( row3 ) => {
        let ( row3a, row3b ) = row3.unwrap();
        ( wrap_sum( Sum::Inl(row3a) ),
          wrap_sum( Sum::Inl(row3b) )
        )
      },
      Sum::Inr ( row3 ) => {
        let (row3a, row3b) = R::split_row (row3);
        ( wrap_sum( Sum::Inr(row3a) ),
          wrap_sum( Sum::Inr(row3b) )
        )
      }
    }
  }
}

impl IntersectSum for ()
{
  fn intersect_sum
    < F1, F2 >
    ( row1: AppliedSum < (), F1 >,
      row2: AppliedSum < (), F2 >,
    ) ->
      Option <
        AppliedSum <
          (),
          Merge < F1, F2 >
        >
      >
  where
    F1: TyCon,
    F2: TyCon,
  {
    absurd(row1)
  }
}

impl < A, R >
  IntersectSum for
  ( A, R )
where
  R: IntersectSum
{
  fn intersect_sum
    < F1, F2 >
    ( row1: AppliedSum < Self, F1 >,
      row2: AppliedSum < Self, F2 >,
    ) ->
      Option <
        AppliedSum <
          Self,
          Merge < F1, F2 >
        >
      >
  where
    F1: TyCon,
    F2: TyCon,
  {
    let row1a = row1.unwrap();
    let row2a = row2.unwrap();

    match (row1a, row2a) {
      ( Sum::Inl(a1), Sum::Inl(a2) ) => {
        Some ( wrap_sum (
          Sum::Inl (
            wrap_applied(
              ( a1, a2 ) ) ) ) )
      }
      ( Sum::Inr(r1), Sum::Inr(r2) ) => {
        R :: intersect_sum ( r1, r2 )
          .map(| x | {
            wrap_sum ( Sum::Inr(x) )
          })
      },
      _ => {
        None
      }
    }
  }
}

impl SumFunctorBorrow for ()
{
  fn lift_sum_borrow
     < T, F1, F2 >
    ( sum: &AppliedSum < Self, F1 > )
    -> AppliedSum < Self, F2 >
  where
    T: NaturalTransformationBorrow < F1, F2 >
  { match *sum {} }
}

impl < A, R >
  SumFunctorBorrow for
  (A, R)
where
  R: SumFunctorBorrow
{
  fn lift_sum_borrow
     < T, F1, F2 >
    ( sum: &AppliedSum < Self, F1 > )
    -> AppliedSum < Self, F2 >
  where
    T: NaturalTransformationBorrow < F1, F2 >
  {
    match sum {
      Sum::Inl(a) => {
        Sum::Inl (
          T :: lift_field_borrow ( a )
        )
      },
      Sum::Inr(b) => {
        Sum::Inr (
          R :: lift_sum_borrow ( b )
        )
      }
    }
  }
}

impl SumFunctorInject for ()
{
  fn lift_sum_inject
    < L, Root >
    ( ctx: L,
      inject:
        impl Fn
          ( AppliedSum < Self, L::Target > )
          -> Root
          + Send + 'static,
      sum: AppliedSum < Self, L::Source >,
    )
  where
    L: FieldLifter < Root >
  {
    match sum {}
  }
}

impl < A, R >
  SumFunctorInject for
  (A, R)
where
  R: SumFunctorInject,
{
  fn lift_sum_inject
    < L, Root >
    ( ctx: L,
      inject:
        impl Fn
          ( AppliedSum < Self, L::Target > )
          -> Root
          + Send + 'static,
      sum: AppliedSum < Self, L::Source >,
    )
  where
    L: FieldLifter < Root >
  {
    match sum {
      Sum::Inl(a) => {
        let inject2 =
          move | b: Applied < L::Target, A > |
            -> Root
          {
            inject ( Sum::Inl (b) )
          };

        Sum :: Inl(
          L :: lift_field ( ctx, inject2, a )
        )
      },
      Sum::Inr(b) => {
        let inject2 =
          move | r : AppliedSum < R, L::Target > |
            -> Root
          {
            inject ( Sum::Inr (b) )
          };

        Sum::Inr (
          R :: lift_sum ( ctx, inject2, b )
        )
      }
    }
  }
}

impl
  ElimSum for
  ()
{
  fn elim_sum
    < F, E, R >
    ( elim_field: E,
      row: AppliedSum < Self, F >
    ) ->
      R
  where
    F: TyCon,
    E: ElimField < F, R >,
  {
    match row {}
  }
}

impl < A, R >
  ElimSum for
  (A, R)
where
  R: ElimSum,
{
  fn elim_sum
    < F, E, K >
    ( e: E,
      row: AppliedSum < Self, F >
    ) ->
      K
  where
    F: TyCon,
    E: ElimField < F, K >,
  {
    match row {
      Sum::Inl(a) => {
        e.elim_field ( a )
      },
      Sum::Inr(b) => {
        R :: elim_sum ( e, b )
      }
    }
  }
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

  fn extract_elem (
    row : Sum < T::Applied, R::Field >
  ) ->
    Option < T::Applied >
  {
    match row {
      Sum::Inl(e) => Some(e),
      Sum::Inr(_) => None,
    }
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

  fn extract_elem (
    row : Sum < T::Applied, R::Field >
  ) ->
    Option < N::Elem >
  {
    match row {
      Sum::Inl(_) => None,
      Sum::Inr(rest) => N::extract_elem(rest),
    }
  }
}
