
use crate::base::{
  Nat,
  Z,
  S,
  mk_succ,
  Inactive,
  Processes,
  ProcessLens,
  ProcessNode,
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
  P : ProcessNode,
  R : Processes + NextSelector
{
  type Selector = S <
    < R as NextSelector >
    :: Selector
  >;

  fn make_selector () ->
    Self :: Selector
  {
    return mk_succ ();
  }
}

impl
  < P1, P2, R >
  ProcessLens <
    ( P1, R ),
    P1,
    P2
  > for
  Z
where
  P1 : ProcessNode + 'static,
  P2 : ProcessNode + 'static,
  R : Processes + 'static
{
  type Deleted = (Inactive, R);
  type Target = (P2, R);

  fn split_channels (
    (p, r) :
      < ( P1, R )
        as Processes
      > :: Values
  ) ->
    ( < P1 as ProcessNode > :: NodeValue,
      ( (),
        < R as Processes
        > :: Values
      )
    )
  {
    return (p, ((), r));
  }

  fn merge_channels
    ( p : < P2 as ProcessNode > :: NodeValue,
      ((), r) :
        ( (),
          < R as Processes
          > :: Values
        )
    ) ->
      < ( P2, R )
        as Processes
      > :: Values
  {
    return (p, r);
  }
}

impl
  < P, Q1, Q2, R, N >
  ProcessLens <
    ( P, R ),
    Q1,
    Q2
  > for
  S < N >
where
  P : ProcessNode + 'static,
  Q1 : ProcessNode + 'static,
  Q2 : ProcessNode + 'static,
  R : Processes + 'static,
  N : ProcessLens < R, Q1, Q2 >,
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
      < ( P, R ) as Processes >
      :: Values
  ) ->
    ( < Q1 as ProcessNode > :: NodeValue,
      < ( P,
          N :: Deleted
        ) as Processes
      > :: Values
    )
  {
    let (q, r2) =
      < N as ProcessLens < R, Q1, Q2 >
      > :: split_channels ( r1 );

    return ( q, ( p, r2 ) );
  }

  fn merge_channels (
    q : < Q2 as ProcessNode > :: NodeValue,
    (p, r1) :
      < ( P,
          N ::Deleted
        ) as Processes
      > :: Values
  ) ->
    < ( P,
        N :: Target
      ) as Processes
    > :: Values
  {
    let r2 =
      < N as ProcessLens < R, Q1, Q2 >
      > :: merge_channels ( q, r1 );

    return ( p, r2 );
  }
}

pub type Selector1 = S < Z >;
pub type Selector2 = S < Selector1 >;

pub fn select_0 () -> Z {
  Z {}
}

pub fn select_1 () -> Selector1 { mk_succ () }
pub fn select_2 () -> Selector2 { mk_succ () }