pub use super::{
  absurd,
  extract_choice,
  get_applied,
  get_sum,
  get_sum_borrow,
  lift_sum,
  succ,
  wrap_sum_app,
  wrap_type_app,
  App,
  AppSum,
  Bottom,
  ChoiceSelector,
  FlattenSumApp,
  HasSumApp,
  HasTypeApp,
  NaturalTransformation,
  Prism,
  RowCon,
  Sum,
  SumApp,
  SumFunctor,
  ToRow,
  TyCon,
  TypeApp,
  S,
  Z,
};

pub trait Nat: super::Nat {}

impl<N> Nat for N where N: super::Nat {}
