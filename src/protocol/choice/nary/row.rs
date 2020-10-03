use std::any::Any;
use std::marker::PhantomData;

use crate::base::*;

pub trait RowCon
  : Sized + 'static
{}

pub trait SumRow < F >
  : RowCon
where
  F: TyCon,
{
  type Field: Send + 'static;
}

pub trait HasRow < Row, F >
  : Send
{
  fn get_row
    ( self: Box < Self > )
    -> Box < Row::Field >
  where
    F: TyCon,
    Row: SumRow < F >,
  ;
}

pub trait RowWitnessCont < Row, F, K >
{
  fn on_row_witness
    ( self: Box < Self >,
      row: Box < Row::Field >
    ) -> K
  where
    F: TyCon,
    Row: SumRow < F >,
  ;
}

pub trait HasRowWitness < Row, F, K >
  : HasRow < Row, F >
{
  fn with_witness
    ( self: Box < Self >,
      cont: Box < dyn RowWitnessCont < Row, F, K > >
    ) -> K
  ;
}

impl < S, Row, F >
  HasRow < Row, F >
  for S
where
  F: TyCon,
  S: Send + 'static,
  Row: SumRow < F, Field=S >,
{
  fn get_row
    ( self: Box < Self > )
    -> Box < Row::Field >
  where
    F: TyCon,
    Row: SumRow < F >,
  {
    self
  }
}

impl < S, Row, F, K >
  HasRowWitness < Row, F, K >
  for S
where
  F: TyCon,
  S: Send + 'static,
  Row: SumRow < F, Field=S >,
{
  fn with_witness
    ( self: Box < Self >,
      cont: Box < dyn RowWitnessCont < Row, F, K > >
    ) -> K
  {
    cont.on_row_witness(self)
  }
}

pub struct AppliedSum < Row, F >
{
  row: Box < dyn HasRowWitness <
    Row, F, Box < dyn Any > > >
}

impl < Row, F >
  AppliedSum < Row, F >
where
  F: TyCon,
  Row: SumRow < F >,
{
  pub fn get_row (self)
    -> Box < Row::Field >
  {
    self.row.get_row()
  }
}

pub fn wrap_row < Row, F >
  ( row: Row::Field )
  -> AppliedSum < Row, F >
where
  F: TyCon,
  Row: SumRow < F >,
{
  AppliedSum {
    row: Box::new( row )
  }
}

pub enum Bottom {}

impl < A, R >
  RowCon
  for (A, R)
where
  A: 'static,
  R: RowCon,
{ }

impl RowCon for () {}
impl RowCon for Bottom {}

impl < F, A, R >
  SumRow < F > for
  ( A, R )
where
  A: 'static,
  F: TyCon,
  R: RowCon,
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

pub struct Merge < T1, T2 >
  ( PhantomData <( T1, T2 )> );

#[derive(Copy, Clone)]
pub enum Sum < A, B >
{
  Inl ( A ),
  Inr ( B ),
}

pub fn absurd < F, A >
  ( row1: AppliedSum < (), F > )
  -> A
where
  F: TyCon,
{
  let row2 = row1.get_row();
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
    A: 'static,
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
where
  X: 'static,
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

impl < T1, T2 > TyCon
  for Merge < T1, T2 >
where
  T1: 'static,
  T2: 'static,
{}

impl < T1, T2, A >
  TypeApp < A >
  for Merge < T1, T2 >
where
  A: 'static,
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
  A: 'static,
  R: SplitRow,
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
    let row2 = *row1.get_row();

    match row2 {
      Sum::Inl ( row3 ) => {
        let ( row3a, row3b ) = *row3.get_applied();
        ( wrap_row( Sum::Inl(row3a) ),
          wrap_row( Sum::Inl(row3b) )
        )
      },
      Sum::Inr ( row3 ) => {
        let (row3a, row3b) = R::split_row (row3);
        ( wrap_row( Sum::Inr(row3a) ),
          wrap_row( Sum::Inr(row3b) )
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
  A: 'static,
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
    let row1a = *row1.get_row();
    let row2a = *row2.get_row();

    match (row1a, row2a) {
      ( Sum::Inl(a1), Sum::Inl(a2) ) => {
        Some ( wrap_row (
          Sum::Inl (
            wrap_applied(
              ( a1, a2 ) ) ) ) )
      }
      ( Sum::Inr(r1), Sum::Inr(r2) ) => {
        R :: intersect_sum ( r1, r2 )
          .map(| x | {
            wrap_row ( Sum::Inr(x) )
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
    let row2 = row1.get_row();
    match row2 {}
  }
}

impl < A, R >
  SumFunctor for
  (A, R)
where
  A: 'static,
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
    let row2 = *row1.get_row();
    match row2 {
      Sum::Inl(fa1) => {
        let fa2 = T::lift(fa1);
        wrap_row ( Sum::Inl ( fa2 ) )
      },
      Sum::Inr(b) => {
        wrap_row ( Sum::Inr (
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
  A: 'static,
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
    let row2 = *row1.get_row();
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
              wrap_row (
                Sum::Inl (
                  wrap_applied ( b )
                ) ) )
          };

        wrap_row (
          Sum :: Inl(
            wrap_applied(
              L::lift_field( ctx, inject2, *a.get_applied() )
            ) ) )
      },
      Sum::Inr(b) => {
        let inject2 =
          move | r : AppliedSum < R, L::TargetF > |
            -> Root
          {
            inject ( wrap_row ( Sum::Inr (r) ) )
          };

        wrap_row (
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
  A: 'static,
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
    let row2 = *row1.get_row();
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
  A: 'static,
  R: RowCon,
{
  type Elem = A;

  fn inject_elem < F >
    ( t: Applied < F, A > )
    -> AppliedSum < (A, R), F >
  where
    F: TyCon,
  {
    wrap_row ( Sum::Inl(t) )
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
    match *row.get_row() {
      Sum::Inl(e) => Some(e),
      Sum::Inr(_) => None,
    }
  }
}

impl < N, A, R >
  Prism < (A, R) >
  for S < N >
where
  R: RowCon,
  A: 'static,
  N : Prism < R >,
{
  type Elem = N::Elem;

  fn inject_elem < F >
    ( elem : Applied < F, Self::Elem > )
    -> AppliedSum < (A, R), F >
  where
    F: TyCon,
  {
    wrap_row (
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
    match *row.get_row() {
      Sum::Inl(_) => None,
      Sum::Inr(rest) => N::extract_elem(rest),
    }
  }
}
