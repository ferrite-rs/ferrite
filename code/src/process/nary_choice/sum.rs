use std::marker::PhantomData;

use crate::base::*;
use crate::processes::lens::*;

pub enum Sum < P, Q > {
  Inl ( P ),
  Inr ( Q )
}

pub trait ProcessSum {
  type ValueSum;

  type SelectCurrent;
  type SelectorSum;
}

pub trait SelectSum < Lens > : ProcessSum {
  type SelectedProcess : Process;
}

pub struct InternalChoice < Choice >
{
  c : PhantomData < Choice >
}

pub struct ExternalChoice < Choice >
{
  c : PhantomData < Choice >
}

impl
  < Choice >
  Process for
  InternalChoice < Choice >
where
  Choice : ProcessSum
{
  type Value = ();
}

impl < P > ProcessSum for P
where
  P : Process
{
  type ValueSum = P;

  type SelectCurrent = SelectorZ;
  type SelectorSum = SelectorZ;
}

impl < P, R >
  ProcessSum
  for Sum < P, R >
where
  P : Process,
  R : ProcessSum
{
  type ValueSum =
    Sum <
      P :: Value,
      R :: ValueSum
    >;

  type SelectCurrent =
    SelectorSucc <
      R :: SelectCurrent
    >;

  type SelectorSum =
    Sum <
      Self::SelectCurrent,
      R :: SelectorSum
    >;
}

impl
  < P >
  SelectSum < SelectorZ >
  for P
where
  P : Process
{
  type SelectedProcess = P;
}

impl
  < P, R >
  SelectSum <
    SelectorZ
  >
  for Sum < P, R >
where
  P : Process,
  R : ProcessSum
{
  type SelectedProcess = P;
}

impl
  < P, R, Lens >
  SelectSum <
    SelectorSucc < Lens >
  >
  for Sum < P, R >
where
  P : Process,
  R : ProcessSum,
  R : SelectSum < Lens >
{
  type SelectedProcess =
    < R as SelectSum < Lens >
    > :: SelectedProcess ;
}