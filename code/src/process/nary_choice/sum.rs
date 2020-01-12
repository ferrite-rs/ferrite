use std::marker::PhantomData;

use crate::base::*;
use crate::processes::lens::*;
use async_std::sync::Receiver;

pub enum Sum < P, Q > {
  Inl ( P ),
  Inr ( Q )
}

pub trait ProcessSum {
  type ValueSum : Send;

  type SelectCurrent : Send + 'static;
  type SelectorSum : Send + 'static;

  fn select_current () ->
    Self :: SelectCurrent;

  fn value_sum_to_selector_sum
    ( value_sum: &Self::ValueSum
    ) ->
      Self::SelectorSum;
}

pub trait SelectSum < Lens > : ProcessSum {
  type SelectedProcess : Process + 'static;

  fn inject_selected
    ( receiver :
        Receiver <
          < Self :: SelectedProcess
            as Process
          > :: Value
        >
    ) ->
      Self :: ValueSum;
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
  < Sum >
  Process for
  InternalChoice < Sum >
where
  Sum : ProcessSum
{
  type Value = Sum :: ValueSum;
}

impl
  < Sum >
  Process for
  ExternalChoice < Sum >
where
  Sum : ProcessSum
{
  type Value =
    Box <
      dyn FnOnce
        ( Sum :: SelectorSum
        ) ->
          Sum :: ValueSum
      + Send
    >;
}

impl < P > ProcessSum for P
where
  P : Process
{
  type ValueSum =
    Receiver < P :: Value >;

  type SelectCurrent = SelectorZ;
  type SelectorSum = SelectorZ;

  fn select_current () ->
    Self :: SelectCurrent
  {
    SelectorZ {}
  }

  fn value_sum_to_selector_sum
    ( _: &Self::ValueSum
    ) ->
      Self::SelectorSum
  {
    Self :: select_current()
  }
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
      Receiver <
        P :: Value
      >,
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

  fn select_current () ->
    Self :: SelectCurrent
  {
    mk_selector_succ ()
  }

  fn value_sum_to_selector_sum
    ( val_sum: &Self::ValueSum
    ) ->
      Self::SelectorSum
    {
      match val_sum {
        Sum::Inl (_) => {
          Sum::Inl (
            Self :: select_current()
          )
        },
        Sum::Inr (val_sum2) => {
          Sum::Inr (
            R :: value_sum_to_selector_sum
              ( val_sum2) )
        }
      }
    }
}

impl
  < P >
  SelectSum < SelectorZ >
  for P
where
  P : Process + 'static
{
  type SelectedProcess = P;

  fn inject_selected
    ( receiver :
        Receiver <
          P :: Value
        >
    ) ->
      Self :: ValueSum
  {
    receiver
  }
}

impl
  < P, R >
  SelectSum <
    SelectorZ
  >
  for Sum < P, R >
where
  P : Process + 'static,
  R : ProcessSum
{
  type SelectedProcess = P;

  fn inject_selected
    ( receiver :
        Receiver <
          P :: Value
        >
    ) ->
      Self :: ValueSum
  {
    Sum::Inl ( receiver )
  }
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

  fn inject_selected
    ( receiver :
        Receiver <
          < Self :: SelectedProcess
            as Process
          > :: Value
        >
    ) ->
      Self :: ValueSum
  {
    Sum :: Inr (
      < R as SelectSum < Lens >
      > :: inject_selected ( receiver )
    )
  }
}