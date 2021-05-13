use crate::internal::{
  base::ReceiverF,
  functional::{
    lift_sum,
    wrap_type_app,
    App,
    AppSum,
    Merge,
    SplitRow,
    SumFunctor,
  },
};

pub fn receiver_to_selector<Row>(
  row1: AppSum<Row, ReceiverF>
) -> (AppSum<Row, ReceiverF>, AppSum<Row, ()>)
where
  Row: SplitRow,
  Row: SumFunctor,
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
        wrap_type_app ( (
          receiver,
          wrap_type_app( () )
        ) )
      }
    },
    row1,
  );

  Row::split_row(row2)
}
