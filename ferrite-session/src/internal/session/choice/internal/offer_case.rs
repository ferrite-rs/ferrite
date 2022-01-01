use crate::internal::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    ConsumerEndpointF,
    Context,
    PartialSession,
    Protocol,
    ReceiverF,
  },
  functional::{
    wrap_type_app,
    Prism,
    SumApp,
    ToRow,
  },
  protocol::InternalChoice,
};

pub fn offer_case<N, C, A, Row1, Row2>(
  _: N,
  cont: PartialSession<C, A>,
) -> PartialSession<C, InternalChoice<Row1>>
where
  C: Context,
  A: Protocol,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  Row2: SumApp<ReceiverF>,
  N: Prism<Row2, Elem = A>,
{
  unsafe_create_session::<C, InternalChoice<Row1>, _, _>(
    move |ctx, consumer_end_sum_sender| async move {
      let (provider_end, consumer_end) = A::create_endpoints();

      let consumer_end_sum =
        N::inject_elem(wrap_type_app::<ConsumerEndpointF, A>(consumer_end));

      consumer_end_sum_sender.send(consumer_end_sum).unwrap();

      unsafe_run_session(cont, ctx, provider_end).await;
    },
  )
}
