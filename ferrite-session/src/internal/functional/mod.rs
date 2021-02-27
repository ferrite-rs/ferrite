pub mod base;
pub mod identity;
pub mod nat;
pub mod row;
pub mod type_app;

#[macro_use]
pub mod macros;

#[doc(inline)]
pub use self::{
  base::{
    Applicative,
    Functor,
    Monad,
    NaturalTransformation,
  },
  identity::{
    Identity,
    IdentityF,
  },
  nat::{
    succ,
    Nat,
    S,
    Z,
  },
  row::{
    absurd,
    cloak_row,
    extract,
    get_sum,
    get_sum_borrow,
    lift_sum,
    lift_sum_inject,
    AppSum,
    Bottom,
    ChoiceSelector,
    ElimConst,
    ElimField,
    ElimSum,
    FlattenSumApp,
    HasSumApp,
    InjectLift,
    IntersectSum,
    Merge,
    Prism,
    RowCon,
    SplitRow,
    Sum,
    SumApp,
    SumFunctor,
    SumFunctorInject,
  },
  type_app::{
    cloak_applied,
    get_applied,
    with_applied,
    App,
    Const,
    HasTypeApp,
    TyCon,
    TypeApp,
  },
};
