
use crate::base::{
  Nat,
  Z,
  S,
};

pub type Selector1 = S < Z >;
pub type Selector2 = S < Selector1 >;

pub fn select_0 () -> Z {
  Z {}
}

pub fn select_1 () -> Selector1 {
  Selector1 :: nat ()
}
pub fn select_2 () -> Selector2 {
  Selector2 :: nat ()
}
