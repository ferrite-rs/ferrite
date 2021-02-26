use crate::internal::{
  base::ReceiverF,
  functional::{
    cloak_applied,
    lift_sum,
    Applied,
    AppliedSum,
    Merge,
    SplitRow,
    SumFunctor,
  },
};

pub fn receiver_to_selector<Row>(
  row1 : AppliedSum<Row, ReceiverF>
) -> (AppliedSum<Row, ReceiverF>, AppliedSum<Row, ()>)
where
  Row : SplitRow,
  Row : SumFunctor,
{
  let row2 = lift_sum(
    crate::natural_transformation! {
      { } ;
      ReceiverOnceToSelector :
        forall x .
          ReceiverF [@x] ->
          Merge < ReceiverF, () > [@x]
        ;
      (receiver) => {
        cloak_applied ( (
          receiver,
          cloak_applied( () )
        ) )
      }
    },
    row1,
  );

  Row::split_row(row2)
}
