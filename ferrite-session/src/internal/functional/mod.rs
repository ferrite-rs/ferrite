pub mod base;
pub mod identity;
pub mod nat;
pub mod public;
pub mod row;
pub mod type_app;

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
    extract,
    get_sum,
    get_sum_borrow,
    lift_sum,
    lift_sum_inject,
    wrap_sum_app,
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
    ToRow,
  },
  type_app::{
    get_applied,
    wrap_type_app,
    App,
    Const,
    HasTypeApp,
    TyCon,
    TypeApp,
  },
};
