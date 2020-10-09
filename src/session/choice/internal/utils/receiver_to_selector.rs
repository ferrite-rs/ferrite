use crate::protocol::*;
use crate::functional::*;

pub fn receiver_to_selector < Row >
  ( row1: AppliedSum < Row, ReceiverF > )
  ->
    ( AppliedSum < Row, ReceiverF >,
      AppliedSum < Row, () >
    )
where
  Row : SplitRow,
  Row : SumFunctor,
{
  let row2 = Row::lift_sum::
        < ReceiverToSelector, _, _ >
        ( row1 );

  Row::split_row(row2)
}

struct ReceiverToSelector {}

impl
  NaturalTransformation
  < ReceiverF,
    Merge <
      ReceiverF,
      ()
    >
  >
  for ReceiverToSelector
{
  fn lift < A >
    ( receiver: Applied < ReceiverF, A > )
    ->
      Applied <
        Merge < ReceiverF, () >,
        A
      >
  where
    A: Send + 'static,
  {
    cloak_applied ( (
      receiver,
      cloak_applied( () )
    ) )
  }
}
