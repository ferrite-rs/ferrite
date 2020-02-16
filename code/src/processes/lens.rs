
use crate::base::{
  Nat,
  Z,
  S,
  Empty,
  Context,
  ContextLens,
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

impl
  < P1, P2, R >
  ContextLens <
    ( P1, R ),
    P1,
    P2
  > for
  Z
where
  P1 : Slot + 'static,
  P2 : Slot + 'static,
  R : Context + 'static
{
  type Deleted = (Empty, R);
  type Target = (P2, R);

  fn split_channels (
    (p, r) :
      < ( P1, R )
        as Context
      > :: Values
  ) ->
    ( < P1 as Slot > :: SlotValue,
      ( (),
        < R as Context
        > :: Values
      )
    )
  {
    return (p, ((), r));
  }

  fn merge_channels
    ( p : < P2 as Slot > :: SlotValue,
      ((), r) :
        ( (),
          < R as Context
          > :: Values
        )
    ) ->
      < ( P2, R )
        as Context
      > :: Values
  {
    return (p, r);
  }
}

impl
  < P, Q1, Q2, R, N >
  ContextLens <
    ( P, R ),
    Q1,
    Q2
  > for
  S < N >
where
  P : Slot + 'static,
  Q1 : Slot + 'static,
  Q2 : Slot + 'static,
  R : Context + 'static,
  N : ContextLens < R, Q1, Q2 >,
{
  type Deleted =
    ( P,
      N :: Deleted
    );

  type Target =
    ( P,
      N :: Target
    );

  fn split_channels (
    (p, r1) :
      < ( P, R ) as Context >
      :: Values
  ) ->
    ( < Q1 as Slot > :: SlotValue,
      < ( P,
          N :: Deleted
        ) as Context
      > :: Values
    )
  {
    let (q, r2) =
      < N as ContextLens < R, Q1, Q2 >
      > :: split_channels ( r1 );

    return ( q, ( p, r2 ) );
  }

  fn merge_channels (
    q : < Q2 as Slot > :: SlotValue,
    (p, r1) :
      < ( P,
          N ::Deleted
        ) as Context
      > :: Values
  ) ->
    < ( P,
        N :: Target
      ) as Context
    > :: Values
  {
    let r2 =
      < N as ContextLens < R, Q1, Q2 >
      > :: merge_channels ( q, r1 );

    return ( p, r2 );
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
