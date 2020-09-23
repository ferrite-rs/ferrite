use std::mem::transmute;
use std::marker::PhantomData;

use crate::base::*;

pub trait RowCon : Send {}

pub trait SumRow < F: TyCon > : RowCon
{ type Field : Send;
}

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

// impl < 'a, Row > RowCon
//   for &'a Row
// where
//   Row: RowCon
// { }

// impl < 'a, Row, F >
//   SumRow < F >
//   for &'a Row
// where
//   F: TyCon,
//   Row: SumRow < F >,
// {
//   type Field = &'a Row::Field;
// }

pub struct Merge < T1, T2 >
  ( PhantomData <( T1, T2 )> );

#[derive(Copy, Clone)]
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

unsafe impl < Row, F >
  Send
  for AppliedSum < Row, F >
{}

impl < Row, F >
  AppliedSum < Row, F >
where
  F: TyCon,
  Row: SumRow < F >,
{ pub fn unwrap ( self )
    -> Row::Field
  { unwrap_sum(self) }
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

pub trait SumFunctor
  : RowCon
{
  fn lift_sum
     < T, F1, F2 >
    ( sum: AppliedSum < Self, F1 > )
    -> AppliedSum < Self, F2 >
  where
    F1: TyCon,
    F2: TyCon,
    T: NaturalTransformation < F1, F2 >
  ;
}

pub trait FieldConstraint < A >
  : TypeApp < A >
{
  fn witness () -> Self::Applied;
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
          ( < Self::TargetF
              as TypeApp < A >
            > :: Applied )
          -> Root
        + Send + 'static,
      row:
        < Self::SourceF
          as TypeApp < A >
        > :: Applied
    ) ->
      < Self::InjectF
        as TypeApp < A >
      > :: Applied
  where
    Self::SourceF : TypeApp < A >,
    Self::TargetF : TypeApp < A >,
    Self::InjectF : TypeApp < A >,
  ;
}

pub trait SumFunctorInject < L, Root >
  : RowCon
where
  L: FieldLifter < Root >
{
  fn lift_sum_inject
    ( ctx: L,
      inject:
        impl Fn
          ( AppliedSum < Self, L::TargetF > )
          -> Root
          + Send + 'static,
      sum: AppliedSum < Self, L::SourceF >,
    ) ->
      AppliedSum < Self, L::InjectF >
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

pub trait Prism < Row >
where
  Row : RowCon,
{
  type Elem;

  fn inject_elem < F >
    ( elem : Applied < F, Self::Elem > )
    -> AppliedSum < Row, F >
  where
    F: TyCon,
  ;

  fn extract_elem < F >
    ( row : AppliedSum < Row, F > )
    ->
      Option <
        Applied < F, Self::Elem >
      >
  where
    F: TyCon,
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

impl SumFunctor for ()
{
  fn lift_sum
     < T, F1, F2 >
    ( row1: AppliedSum < Self, F1 > )
    -> AppliedSum < Self, F2 >
  where
    F1: TyCon,
    F2: TyCon,
    T: NaturalTransformation < F1, F2 >
  {
    let row2: Bottom = row1.unwrap();
    match row2 {}
  }
}

impl < A, R >
  SumFunctor for
  (A, R)
where
  R: SumFunctor
{
  fn lift_sum
     < T, F1, F2 >
    ( row1: AppliedSum < Self, F1 > )
    -> AppliedSum < Self, F2 >
  where
    F1: TyCon,
    F2: TyCon,
    T: NaturalTransformation < F1, F2 >
  {
    let row2 = row1.unwrap();
    match row2 {
      Sum::Inl(fa1) => {
        let fa2 = T::lift(fa1);
        wrap_sum ( Sum::Inl ( fa2 ) )
      },
      Sum::Inr(b) => {
        wrap_sum ( Sum::Inr (
          R :: lift_sum::< T, F1, F2 >( b )
        ) )
      }
    }
  }
}

impl < L, Root >
  SumFunctorInject < L, Root >
  for ()
where
  L: FieldLifter < Root >
{
  fn lift_sum_inject

    ( ctx: L,
      inject:
        impl Fn
          ( AppliedSum < Self, L::TargetF > )
          -> Root
          + Send + 'static,
      sum: AppliedSum < Self, L::SourceF >,
    )
  {
    match sum {}
  }
}

impl < A, R, L, Root >
  SumFunctorInject < L, Root >
  for (A, R)
where
  L: FieldLifter < Root >,
  R: SumFunctorInject < L, Root >,
  L::SourceF : TypeApp < A >,
  L::TargetF : TypeApp < A >,
  L::InjectF : TypeApp < A >,
{
  fn lift_sum_inject
    ( ctx: L,
      inject:
        impl Fn
          ( AppliedSum < Self, L::TargetF > )
          -> Root
          + Send + 'static,
      row1: AppliedSum < Self, L::SourceF >,
    ) ->
      AppliedSum < Self, L::InjectF >
  {
    let row2 = row1.unwrap();
    match row2 {
      Sum::Inl(a) => {
        let inject2 =
          move |
            b:  < L::TargetF
                  as TypeApp < A >
                > :: Applied
          |
            -> Root
          {
            inject (
              wrap_sum (
                Sum::Inl (
                  wrap_applied ( b )
                ) ) )
          };

        wrap_sum (
          Sum :: Inl(
            wrap_applied(
              L::lift_field( ctx, inject2, a.unwrap() )
            ) ) )
      },
      Sum::Inr(b) => {
        let inject2 =
          move | r : AppliedSum < R, L::TargetF > |
            -> Root
          {
            inject ( wrap_sum ( Sum::Inr (r) ) )
          };

        wrap_sum (
          Sum::Inr (
            R :: lift_sum_inject ( ctx, inject2, b )
          ) )
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
      row1: AppliedSum < Self, F >
    ) ->
      K
  where
    F: TyCon,
    E: ElimField < F, K >,
  {
    let row2 = row1.unwrap();
    match row2 {
      Sum::Inl(a) => {
        e.elim_field ( a )
      },
      Sum::Inr(b) => {
        R :: elim_sum ( e, b )
      }
    }
  }
}

impl < A, R >
  Prism < (A, R) >
  for Z
where
  R : RowCon
{
  type Elem = A;

  fn inject_elem < F >
    ( t: Applied < F, A > )
    -> AppliedSum < (A, R), F >
  where
    F: TyCon,
  {
    wrap_sum ( Sum::Inl(t) )
  }

  fn extract_elem < F >
    ( row : AppliedSum < (A, R), F > )
    ->
      Option <
        Applied < F, A >
      >
  where
    F: TyCon,
  {
    match row.unwrap() {
      Sum::Inl(e) => Some(e),
      Sum::Inr(_) => None,
    }
  }
}

impl < N, A, R >
  Prism < (A, R) >
  for S < N >
where
  N : Prism < R >,
  R: RowCon,
{
  type Elem = N::Elem;

  fn inject_elem < F >
    ( elem : Applied < F, Self::Elem > )
    -> AppliedSum < (A, R), F >
  where
    F: TyCon,
  {
    wrap_sum (
      Sum::Inr( N::inject_elem( elem ) ) )
  }

  fn extract_elem < F >
    ( row : AppliedSum < (A, R), F > )
    ->
      Option <
        Applied < F, Self::Elem >
      >
  where
    F: TyCon,
  {
    match row.unwrap() {
      Sum::Inl(_) => None,
      Sum::Inr(rest) => N::extract_elem(rest),
    }
  }
}
