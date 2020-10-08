
use super::utils::*;
use super::traits::*;
use super::structs::*;
use crate::functional::nat::*;
use crate::functional::base::*;
use crate::functional::type_app::*;

impl < S, Row, F >
  HasRow < Row, F >
  for S
where
  F: TyCon,
  S: Send + 'static,
  Row: RowApp < F, Applied=S >,
{
  fn get_row
    ( self: Box < Self > )
    -> Box < Row::Applied >
  where
    F: TyCon,
    Row: RowApp < F >,
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
  Row: RowApp < F, Applied=S >,
{
  fn with_witness
    ( self: Box < Self >,
      cont: Box < dyn RowWitnessCont < Row, F, K > >
    ) -> K
  {
    cont.on_row_witness(self)
  }
}

impl < A, R >
  RowCon
  for (A, R)
where
  A: Send + 'static,
  R: RowCon,
{ }

impl RowCon for () {}
impl RowCon for Bottom {}

impl < F, A, R >
  RowApp < F > for
  ( A, R )
where
  A: Send + 'static,
  F: TyCon,
  R: RowCon,
{
  type Applied =
    Sum <
      Applied < F, A >,
      AppliedSum < R, F >,
    >
  ;
}

impl < F >
  RowApp < F > for
  ()
where
  F: TyCon
{
  type Applied = Bottom;
}

impl < F, A, R >
  WrapRow < F > for
  ( A, R )
where
  A: Send + 'static,
  R: WrapRow < F >,
  F: TypeApp < A >,
{
  type Unwrapped =
    Sum <
      F::Applied,
      R::Unwrapped
    >
  ;

  fn wrap_row
    ( row1: Self::Unwrapped )
    -> Self::Applied
  {
    match row1 {
      Sum::Inl(field) => {
        Sum::Inl( wrap_applied( field ) )
      }
      Sum::Inr(row2) => {
        let row3 = R::wrap_row( row2 );
        Sum::Inr( wrap_row( row3 ) )
      }
    }
  }

  fn unwrap_row
    ( row1: AppliedSum < Self, F > )
    -> Self::Unwrapped
  {
    match *row1.get_row() {
      Sum::Inl(field1) => {
        let field2 = *field1.get_applied();
        Sum::Inl( field2 )
      }
      Sum::Inr(row2) => {
        let row3 = R::unwrap_row( row2 );
        Sum::Inr( row3 )
      }
    }
  }
}

impl < F >
  WrapRow < F >
  for ()
where
  F: TyCon
{
  type Unwrapped = Bottom;

  fn wrap_row
    ( row: Self::Unwrapped )
    -> Self::Applied
  {
    row
  }

  fn unwrap_row
    ( row: AppliedSum < Self, F > )
    -> Self::Unwrapped
  {
    *row.get_row()
  }
}

impl < X >
  ElimApplied <
    Const < X >,
    X
  >
  for ElimConst
where
  X: Send + 'static,
{
  fn elim_field < A >
    ( self,
      x : Applied < Const < X >, A >
    ) -> X
  where
    A: 'static
  {
    *get_applied(x)
  }
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
    absurd(row1)
  }
}

impl < A, R >
  SplitRow
  for ( A, R )
where
  A: Send + 'static,
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
      _row2: AppliedSum < (), F2 >,
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
  A: Send + 'static,
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
    absurd(row1)
  }
}

impl < A, R >
  SumFunctor for
  (A, R)
where
  A: Send + 'static,
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

impl
  SumFunctorInject
  for ()
{
  fn lift_sum_inject < L, Root, Inject >
    ( _ctx: L,
      _inject: Inject,
      sum: AppliedSum < Self, L::SourceF >,
    ) ->
      AppliedSum < Self, L::InjectF >
  where
    L: AppliedLifter < Root >,
    Inject:
      Fn ( AppliedSum < Self, L::TargetF > )
        -> Root
        + Send + 'static,
  {
    absurd(sum)
  }
}

impl < A, R >
  SumFunctorInject
  for (A, R)
where
  A: Send + 'static,
  R: SumFunctorInject,
{
  fn lift_sum_inject < L, Root, Inject >
    ( ctx: L,
      inject: Inject,
      row1: AppliedSum < Self, L::SourceF >,
    ) ->
      AppliedSum < Self, L::InjectF >
  where
    L: AppliedLifter < Root >,
    Inject:
      Fn ( AppliedSum < Self, L::TargetF > )
        -> Root
        + Send + 'static,
  {
    let row2 = *row1.get_row();
    match row2 {
      Sum::Inl(a) => {
        let inject2 =
          move |
            b: Applied < L::TargetF, A >
          | -> Root
          {
            inject (
              wrap_row (
                Sum::Inl (
                  b
                ) ) )
          };

        wrap_row (
          Sum :: Inl(
            L::lift_field( ctx, inject2, a )
          ) )
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
    ( _elim_field: E,
      row: AppliedSum < Self, F >
    ) ->
      R
  where
    F: TyCon,
    E: ElimApplied < F, R >,
  {
    absurd(row)
  }
}

impl < A, R >
  ElimSum for
  (A, R)
where
  A: Send + 'static,
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
    E: ElimApplied < F, K >,
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
  for ChoiceSelector < Z >
where
  A: Send + 'static,
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
  for ChoiceSelector < S < N > >
where
  R: RowCon,
  A: Send + 'static,
  ChoiceSelector < N > : Prism < R >,
{
  type Elem =
    < ChoiceSelector < N >
      as Prism < R >
    >::Elem;

  fn inject_elem < F >
    ( elem : Applied < F, Self::Elem > )
    -> AppliedSum < (A, R), F >
  where
    F: TyCon,
  {
    wrap_row (
      Sum::Inr(
        < ChoiceSelector < N >
          as Prism < R >
        >::inject_elem( elem ) ) )
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
      Sum::Inr(rest) =>
        < ChoiceSelector < N >
          as Prism < R >
        >::extract_elem(rest),
    }
  }
}
