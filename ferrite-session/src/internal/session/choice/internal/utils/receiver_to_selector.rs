use crate::internal::{
  base::ConsumerEndpointF,
  functional::{
    lift_sum,
    wrap_type_app,
    App,
    AppSum,
    Merge,
    NaturalTransformation,
    SplitRow,
    SumFunctor,
  },
};

pub fn receiver_to_selector<Row: 'static>(
  row1: AppSum<'static, Row, ConsumerEndpointF>
) -> (AppSum<'static, Row, ConsumerEndpointF>, AppSum<Row, ()>)
where
  Row: SplitRow,
  Row: SumFunctor,
{
  struct ReceiverOnceToSelector;

  impl
    NaturalTransformation<
      'static,
      ConsumerEndpointF,
      Merge<ConsumerEndpointF, ()>,
    > for ReceiverOnceToSelector
  {
    fn lift<A: 'static>(
      self,
      receiver: App<ConsumerEndpointF, A>,
    ) -> App<Merge<ConsumerEndpointF, ()>, A>
    {
      wrap_type_app((receiver, wrap_type_app(())))
    }
  }

  let row2 = lift_sum(ReceiverOnceToSelector, row1);

  Row::split_row(row2)
}
