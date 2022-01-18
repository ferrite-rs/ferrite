pub mod base;
pub mod nat;
pub mod public;
pub mod row;
pub mod type_app;

#[doc(inline)]
pub use self::{
  base::NaturalTransformation,
  nat::{
    succ,
    Nat,
    S,
    Z,
  },
  row::{
    absurd,
    extract_choice,
    get_sum,
    get_sum_borrow,
    lift_sum,
    wrap_sum_app,
    AppSum,
    Bottom,
    ChoiceSelector,
    FlattenSumApp,
    HasSumApp,
    Prism,
    RowCon,
    Sum,
    SumApp,
    SumFunctor,
    ToRow,
  },
  type_app::{
    get_applied,
    wrap_type_app,
    App,
    HasTypeApp,
    TyCon,
    TypeApp,
  },
};
