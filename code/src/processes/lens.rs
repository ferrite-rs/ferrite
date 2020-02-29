
use crate::base::{
  Nat,
  Z,
  S,
  Context,
  Slot,
};

pub trait NextSelector {
  type Selector : Nat;

  fn make_selector () ->
    Self :: Selector;
}

impl NextSelector for () {
  type Selector = Z;

  fn make_selector () ->
    Self :: Selector
  {
    return Z {};
  }
}

impl
  < P, R >
  NextSelector for
  ( P, R )
where
  P : Slot,
  R : Context + NextSelector
{
  type Selector = S <
    < R as NextSelector >
    :: Selector
  >;

  fn make_selector () ->
    Self :: Selector
  {
    Self :: Selector :: nat ()
  }
}

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
