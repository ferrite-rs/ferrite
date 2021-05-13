use serde;

use super::{
  structs::*,
  traits::*,
  utils::*,
};
use crate::internal::functional::{
  base::*,
  nat::*,
  type_app::*,
};

impl<Row, F> serde::Serialize for AppSum<Row, F>
where
  F: TyCon,
  Row: SumApp<F>,
  Row::Applied:
    Send + 'static + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let row: &Row::Applied = get_sum_borrow(self);

    row.serialize(serializer)
  }
}

impl<'a, Row, F, T> serde::Deserialize<'a> for AppSum<Row, F>
where
  F: TyCon,
  T: Send + 'static,
  Row: SumApp<F, Applied = T>,
  T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>,
  {
    let row = T::deserialize(deserializer)?;

    Ok(wrap_sum_app(row))
  }
}

impl<S, Row, F> HasSumApp<Row, F> for S
where
  F: TyCon,
  S: Send + 'static,
  Row: SumApp<F, Applied = S>,
{
  fn get_sum(self: Box<Self>) -> Box<Row::Applied>
  where
    F: TyCon,
    Row: SumApp<F>,
  {
    self
  }

  fn get_sum_borrow<'a>(&'a self) -> &'a Row::Applied
  where
    F: TyCon,
    Row: SumApp<F>,
  {
    self
  }
}

impl<A, R> RowCon for (A, R)
where
  A: Send + 'static,
  R: RowCon,
{
}

impl RowCon for () {}

impl RowCon for Bottom {}

impl<F, A, R> SumApp<F> for (A, R)
where
  A: Send + 'static,
  F: TyCon,
  R: RowCon,
{
  type Applied = Sum<App<F, A>, AppSum<R, F>>;
}

impl<F> SumApp<F> for ()
where
  F: TyCon,
{
  type Applied = Bottom;
}

impl<F, A, R> FlattenSumApp<F> for (A, R)
where
  A: Send + 'static,
  R: FlattenSumApp<F>,
  F: TypeApp<A>,
{
  type FlattenApplied = Sum<F::Applied, R::FlattenApplied>;

  fn unflatten_sum(row1: Self::FlattenApplied) -> Self::Applied
  {
    match row1 {
      Sum::Inl(field) => Sum::Inl(wrap_type_app(field)),
      Sum::Inr(row2) => {
        let row3 = R::unflatten_sum(row2);

        Sum::Inr(wrap_sum_app(row3))
      }
    }
  }

  fn flatten_sum(row1: AppSum<Self, F>) -> Self::FlattenApplied
  {
    match row1.get_sum() {
      Sum::Inl(field1) => {
        let field2 = field1.get_applied();

        Sum::Inl(field2)
      }
      Sum::Inr(row2) => {
        let row3 = R::flatten_sum(row2);

        Sum::Inr(row3)
      }
    }
  }
}

impl<F> FlattenSumApp<F> for ()
where
  F: TyCon,
{
  type FlattenApplied = Bottom;

  fn unflatten_sum(row: Self::FlattenApplied) -> Self::Applied
  {
    row
  }

  fn flatten_sum(row: AppSum<Self, F>) -> Self::FlattenApplied
  {
    row.get_sum()
  }
}

impl<X> ElimField<Const<X>, X> for ElimConst
where
  X: Send + 'static,
{
  fn elim_field<A>(
    self,
    x: App<Const<X>, A>,
  ) -> X
  where
    A: 'static,
  {
    get_applied(x)
  }
}

impl<T1, T2> TyCon for Merge<T1, T2>
where
  T1: 'static,
  T2: 'static,
{
}

impl<T1, T2, A> TypeApp<A> for Merge<T1, T2>
where
  A: 'static,
  T1: TyCon,
  T2: TyCon,
{
  type Applied = (App<T1, A>, App<T2, A>);
}

impl SplitRow for ()
{
  fn split_row<F1, F2>(
    row1: AppSum<Self, Merge<F1, F2>>
  ) -> (AppSum<Self, F1>, AppSum<Self, F2>)
  where
    F1: TyCon,
    F2: TyCon,
  {
    absurd(row1)
  }
}

impl<A, R> SplitRow for (A, R)
where
  A: Send + 'static,
  R: SplitRow,
{
  fn split_row<F1, F2>(
    row1: AppSum<Self, Merge<F1, F2>>
  ) -> (AppSum<Self, F1>, AppSum<Self, F2>)
  where
    F1: TyCon,
    F2: TyCon,
  {
    let row2 = row1.get_sum();

    match row2 {
      Sum::Inl(row3) => {
        let (row3a, row3b) = row3.get_applied();

        (wrap_sum_app(Sum::Inl(row3a)), wrap_sum_app(Sum::Inl(row3b)))
      }
      Sum::Inr(row3) => {
        let (row3a, row3b) = R::split_row(row3);

        (wrap_sum_app(Sum::Inr(row3a)), wrap_sum_app(Sum::Inr(row3b)))
      }
    }
  }
}

impl IntersectSum for ()
{
  fn intersect_sum<F1, F2>(
    row1: AppSum<(), F1>,
    _row2: AppSum<(), F2>,
  ) -> Option<AppSum<(), Merge<F1, F2>>>
  where
    F1: TyCon,
    F2: TyCon,
  {
    absurd(row1)
  }
}

impl<A, R> IntersectSum for (A, R)
where
  A: Send + 'static,
  R: IntersectSum,
{
  fn intersect_sum<F1, F2>(
    row1: AppSum<Self, F1>,
    row2: AppSum<Self, F2>,
  ) -> Option<AppSum<Self, Merge<F1, F2>>>
  where
    F1: TyCon,
    F2: TyCon,
  {
    let row1a = row1.get_sum();

    let row2a = row2.get_sum();

    match (row1a, row2a) {
      (Sum::Inl(a1), Sum::Inl(a2)) => {
        Some(wrap_sum_app(Sum::Inl(wrap_type_app((a1, a2)))))
      }
      (Sum::Inr(r1), Sum::Inr(r2)) => {
        R::intersect_sum(r1, r2).map(|x| wrap_sum_app(Sum::Inr(x)))
      }
      _ => None,
    }
  }
}

impl SumFunctor for ()
{
  fn lift_sum<T, F1, F2>(
    _lift: &T,
    row1: AppSum<Self, F1>,
  ) -> AppSum<Self, F2>
  where
    F1: TyCon,
    F2: TyCon,
    T: NaturalTransformation<F1, F2>,
  {
    absurd(row1)
  }
}

impl<A, R> SumFunctor for (A, R)
where
  A: Send + 'static,
  R: SumFunctor,
{
  fn lift_sum<T, F1, F2>(
    lift: &T,
    row1: AppSum<Self, F1>,
  ) -> AppSum<Self, F2>
  where
    F1: TyCon,
    F2: TyCon,
    T: NaturalTransformation<F1, F2>,
  {
    let row2 = row1.get_sum();

    match row2 {
      Sum::Inl(fa1) => {
        let fa2 = lift.lift(fa1);

        wrap_sum_app(Sum::Inl(fa2))
      }
      Sum::Inr(b) => wrap_sum_app(Sum::Inr(R::lift_sum::<T, F1, F2>(lift, b))),
    }
  }
}

impl SumFunctorInject for ()
{
  fn lift_sum_inject<L, Root, Inject>(
    _ctx: L,
    _inject: Inject,
    sum: AppSum<Self, L::SourceF>,
  ) -> AppSum<Self, L::InjectF>
  where
    L: InjectLift<Root>,
    Inject: Fn(AppSum<Self, L::TargetF>) -> Root + Send + 'static,
  {
    absurd(sum)
  }
}

impl<A, R> SumFunctorInject for (A, R)
where
  A: Send + 'static,
  R: SumFunctorInject,
{
  fn lift_sum_inject<L, Root, Inject>(
    ctx: L,
    inject: Inject,
    row1: AppSum<Self, L::SourceF>,
  ) -> AppSum<Self, L::InjectF>
  where
    L: InjectLift<Root>,
    Inject: Fn(AppSum<Self, L::TargetF>) -> Root + Send + 'static,
  {
    let row2 = row1.get_sum();

    match row2 {
      Sum::Inl(a) => {
        let inject2 = move |b: App<L::TargetF, A>| -> Root {
          inject(wrap_sum_app(Sum::Inl(b)))
        };

        wrap_sum_app(Sum::Inl(L::lift_field(ctx, inject2, a)))
      }
      Sum::Inr(b) => {
        let inject2 = move |r: AppSum<R, L::TargetF>| -> Root {
          inject(wrap_sum_app(Sum::Inr(r)))
        };

        wrap_sum_app(Sum::Inr(R::lift_sum_inject(ctx, inject2, b)))
      }
    }
  }
}

impl ElimSum for ()
{
  fn elim_sum<F, E, R>(
    _elim_field: E,
    row: AppSum<Self, F>,
  ) -> R
  where
    F: TyCon,
    E: ElimField<F, R>,
  {
    absurd(row)
  }
}

impl<A, R> ElimSum for (A, R)
where
  A: Send + 'static,
  R: ElimSum,
{
  fn elim_sum<F, E, K>(
    e: E,
    row1: AppSum<Self, F>,
  ) -> K
  where
    F: TyCon,
    E: ElimField<F, K>,
  {
    let row2 = row1.get_sum();

    match row2 {
      Sum::Inl(a) => e.elim_field(a),
      Sum::Inr(b) => R::elim_sum(e, b),
    }
  }
}

impl<A, R> Prism<(A, R)> for ChoiceSelector<Z>
where
  A: Send + 'static,
  R: RowCon,
{
  type Elem = A;

  fn inject_elem<F>(t: App<F, A>) -> AppSum<(A, R), F>
  where
    F: TyCon,
  {
    wrap_sum_app(Sum::Inl(t))
  }

  fn extract_elem<F>(row: AppSum<(A, R), F>) -> Option<App<F, A>>
  where
    F: TyCon,
  {
    match row.get_sum() {
      Sum::Inl(e) => Some(e),
      Sum::Inr(_) => None,
    }
  }
}

impl<N, A, R> Prism<(A, R)> for ChoiceSelector<S<N>>
where
  R: RowCon,
  A: Send + 'static,
  ChoiceSelector<N>: Prism<R>,
{
  type Elem = <ChoiceSelector<N> as Prism<R>>::Elem;

  fn inject_elem<F>(elem: App<F, Self::Elem>) -> AppSum<(A, R), F>
  where
    F: TyCon,
  {
    wrap_sum_app(Sum::Inr(<ChoiceSelector<N> as Prism<R>>::inject_elem(elem)))
  }

  fn extract_elem<F>(row: AppSum<(A, R), F>) -> Option<App<F, Self::Elem>>
  where
    F: TyCon,
  {
    match row.get_sum() {
      Sum::Inl(_) => None,
      Sum::Inr(rest) => <ChoiceSelector<N> as Prism<R>>::extract_elem(rest),
    }
  }
}
