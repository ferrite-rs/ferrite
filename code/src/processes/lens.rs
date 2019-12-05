use std::marker::PhantomData;

use crate::base::{
  Processes,
  ProcessLens,
  ProcessNode
};

#[derive(Copy, Clone)]
pub struct Selector1 {}

#[derive(Copy, Clone)]
pub struct SelectorNext < Lens > {
  lens : PhantomData < Lens >
}

pub trait NextSelector {
  type Selector;

  fn make_selector () ->
    Self :: Selector;
}

impl NextSelector for () {
  type Selector = Selector1;

  fn make_selector () ->
    Self :: Selector
  {
    return Selector1 {};
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
  type Selector = SelectorNext <
    < R as NextSelector >
    :: Selector
  >;

  fn make_selector () ->
    Self :: Selector
  {
    return SelectorNext {
      lens : PhantomData
    };
  }
}

impl
  < P1, P2, R >
  ProcessLens <
    ( P1, R ),
    ( P2, R ),
    R,
    P1,
    P2
  > for
  Selector1
where
  P1 : ProcessNode + 'static,
  P2 : ProcessNode + 'static,
  R : Processes + 'static
{
  fn split_channels (
    channels :
      < ( P1, R )
        as Processes
      > :: Values
  ) ->
    ( < P1 as ProcessNode > :: NodeValue,
      < R as Processes
      > :: Values
    )
  {
    return channels;
  }

  fn merge_channels
    ( p : < P2 as ProcessNode > :: NodeValue,
      r : < R as Processes
      > :: Values
    ) ->
      < ( P2, R )
        as Processes
      > :: Values
  {
    return (p, r);
  }
}

impl
  < P, Q1, Q2, R1, R2, R3, Lens >
  ProcessLens <
    ( P, R1 ),
    ( P, R2 ),
    ( P, R3 ),
    Q1,
    Q2
  > for
  SelectorNext < Lens >
where
  P : ProcessNode + 'static,
  Q1 : ProcessNode + 'static,
  Q2 : ProcessNode + 'static,
  R1 : Processes + 'static,
  R2 : Processes + 'static,
  R3 : Processes + 'static,
  Lens : ProcessLens < R1, R2, R3, Q1, Q2 >,
{
  fn split_channels (
    (p, r1) :
      < ( P, R1 ) as Processes >
      :: Values
  ) ->
    ( < Q1 as ProcessNode > :: NodeValue,
      < ( P, R3 ) as Processes > :: Values
    )
  {
    let (q, r2) =
      < Lens as ProcessLens < R1, R2, R3, Q1, Q2 >
      > :: split_channels ( r1 );

    return ( q, ( p, r2 ) );
  }

  fn merge_channels (
    q : < Q2 as ProcessNode > :: NodeValue,
    (p, r1) :
      < ( P, R3 ) as Processes >
      :: Values
  ) ->
    < ( P, R2 ) as Processes > :: Values
  {
    let r2 =
      < Lens as ProcessLens < R1, R2, R3, Q1, Q2 >
      > :: merge_channels ( q, r1 );

    return ( p, r2 );
  }
}

pub type Selector2 = SelectorNext < Selector1 >;
pub type Selector3 = SelectorNext < Selector2 >;
pub type Selector4 = SelectorNext < Selector3 >;
pub type Selector5 = SelectorNext < Selector4 >;

pub static SELECT_1 : Selector1 = Selector1{};
pub static SELECT_2 : Selector2 = select_next();
pub static SELECT_3 : Selector3 = select_next();
pub static SELECT_4 : Selector4 = select_next();
pub static SELECT_5 : Selector5 = select_next();

pub const fn select_next
  < Lens >
  () ->
    SelectorNext < Lens >
{
  SelectorNext {
    lens: PhantomData
  }
}

pub fn make_slots_1 () ->
  Selector1
{
  SELECT_1
}

pub fn make_slots_2 () ->
  (Selector1, Selector2)
{
  (SELECT_1, SELECT_2)
}

pub fn make_slots_3 () ->
  (Selector1, Selector2, Selector3)
{
  (SELECT_1, SELECT_2, SELECT_3)
}

pub fn make_slots_4 () ->
  (Selector1, Selector2, Selector3, Selector4)
{
  (SELECT_1, SELECT_2, SELECT_3, SELECT_4)
}
