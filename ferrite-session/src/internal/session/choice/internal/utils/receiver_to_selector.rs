use crate::internal::{
  base::ReceiverF,
  functional::{
    cloak_applied,
    lift_sum,
    App,
    AppSum,
    Merge,
    SplitRow,
    SumFunctor,
  },
};

pub fn receiver_to_selector<Row>(
  row1 : AppSum<Row, ReceiverF>
) -> (AppSum<Row, ReceiverF>, AppSum<Row, ()>)
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
