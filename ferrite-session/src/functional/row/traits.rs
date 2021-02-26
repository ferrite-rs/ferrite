use super::structs::*;
use crate::functional::{base::*, type_app::*};

pub trait RowCon: Sized + Send + 'static
{
}

pub trait RowApp<F>: RowCon
where
  F : TyCon,
{
  type Applied: Send + 'static;
}

pub trait UncloakRow<F>: RowApp<F>
where
  F : TyCon,
{
  type Uncloaked: Send + 'static;

  fn full_cloak_row(row : Self::Uncloaked) -> Self::Applied;

  fn full_uncloak_row(row : AppliedSum<Self, F>) -> Self::Uncloaked;
}

pub trait HasRow<Row, F>: Send
{
  fn get_row(self: Box<Self>) -> Box<Row::Applied>
  where
    F : TyCon,
    Row : RowApp<F>;

  fn get_row_borrow<'a>(&'a self) -> &'a Row::Applied
  where
    F : TyCon,
    Row : RowApp<F>;

  fn get_row_borrow_mut<'a>(&'a mut self) -> &'a mut Row::Applied
  where
    F : TyCon,
    Row : RowApp<F>;
}

pub trait RowWitnessCont<Row, F, K>
{
  fn on_row_witness(
    self: Box<Self>,
    row : Box<Row::Applied>,
  ) -> K
  where
    F : TyCon,
    Row : RowApp<F>;
}

pub trait HasRowWitness<Row, F, K>: HasRow<Row, F>
{
  fn with_witness(
    self: Box<Self>,
    cont : Box<dyn RowWitnessCont<Row, F, K>>,
  ) -> K;
}

pub trait SplitRow: Sized + RowCon
{
  fn split_row<F1, F2>(
    row : AppliedSum<Self, Merge<F1, F2>>
  ) -> (AppliedSum<Self, F1>, AppliedSum<Self, F2>)
  where
    F1 : TyCon,
    F2 : TyCon;
}

pub trait SumFunctor: RowCon
{
  fn lift_sum<T, F1, F2>(
    lift : &T,
    sum : AppliedSum<Self, F1>,
  ) -> AppliedSum<Self, F2>
  where
    F1 : TyCon,
    F2 : TyCon,
    T : NaturalTransformation<F1, F2>;
}

pub trait InjectLift<Root>
{
  type SourceF: TyCon;

  type TargetF: TyCon;

  type InjectF: TyCon;

  fn lift_field<A>(
    self,
    inject : impl Fn(Applied<Self::TargetF, A>) -> Root + Send + 'static,
    row : Applied<Self::SourceF, A>,
  ) -> Applied<Self::InjectF, A>
  where
    A : Send + 'static;
}

pub trait SumFunctorInject: RowCon
{
  fn lift_sum_inject<L, Root, Inject>(
    ctx : L,
    inject : Inject,
    sum : AppliedSum<Self, L::SourceF>,
  ) -> AppliedSum<Self, L::InjectF>
  where
    L : InjectLift<Root>,
    Inject : Fn(AppliedSum<Self, L::TargetF>) -> Root + Send + 'static;
}

pub trait IntersectSum: RowCon
{
  fn intersect_sum<F1, F2>(
    row1 : AppliedSum<Self, F1>,
    row2 : AppliedSum<Self, F2>,
  ) -> Option<AppliedSum<Self, Merge<F1, F2>>>
  where
    F1 : TyCon,
    F2 : TyCon;
}

pub trait ElimField<F, R>
where
  F : TyCon,
{
  fn elim_field<A>(
    self,
    a : Applied<F, A>,
  ) -> R
  where
    A : Send + 'static;
}

pub trait ElimSum: RowCon
{
  fn elim_sum<F, E, R>(
    elim_field : E,
    row : AppliedSum<Self, F>,
  ) -> R
  where
    F : TyCon,
    E : ElimField<F, R>;
}

pub trait Prism<Row>
where
  Row : RowCon,
{
  type Elem;

  fn inject_elem<F>(elem : Applied<F, Self::Elem>) -> AppliedSum<Row, F>
  where
    F : TyCon;

  fn extract_elem<F>(
    row : AppliedSum<Row, F>
  ) -> Option<Applied<F, Self::Elem>>
  where
    F : TyCon;
}
