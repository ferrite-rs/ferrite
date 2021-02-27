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
    get_row,
    get_row_borrow,
    lift_sum,
    lift_sum_inject,
    AppSum,
    Bottom,
    ChoiceSelector,
    ElimConst,
    ElimField,
    ElimSum,
    HasRow,
    InjectLift,
    IntersectSum,
    Merge,
    Prism,
    SumApp,
    RowCon,
    SplitRow,
    Sum,
    SumFunctor,
    SumFunctorInject,
    UncloakRow,
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
