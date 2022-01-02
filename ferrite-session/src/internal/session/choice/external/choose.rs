use crate::internal::{
  base::{
    once_channel,
    unsafe_create_session,
    unsafe_run_session,
    Context,
    ContextLens,
    PartialSession,
    Protocol,
    Value,
  },
  functional::{
    wrap_type_app,
    AppSum,
    Prism,
    RowCon,
    ToRow,
  },
  protocol::ExternalChoice,
};

pub fn choose<N, M, C1, C2, A, B, Row1, Row2>(
  _: N,
  _: M,
  cont: PartialSession<C2, A>,
) -> PartialSession<C1, A>
where
  C1: Context,
  C2: Context,
  A: Protocol,
  B: Protocol,
  Row2: RowCon,
  Row1: Send + 'static,
  Row1: ToRow<Row = Row2>,
  N: ContextLens<C1, ExternalChoice<Row1>, B, Target = C2>,
  M: Prism<Row2, Elem = B>,
{
  unsafe_create_session(move |ctx1, provider_end| async move {
    let (endpoint, ctx2) = N::extract_source(ctx1);

    let choice_sender = endpoint.get_applied();

    let (sum_sender, sum_receiver) = once_channel();

    let choice: AppSum<Row2, ()> = M::inject_elem(wrap_type_app(()));

    choice_sender.send((Value(choice), sum_sender)).unwrap();

    let consumer_end_sum = sum_receiver.recv().await.unwrap();

    let m_consumer_end = M::extract_elem(consumer_end_sum);

    match m_consumer_end {
      Some(consumer_end) => {
        let ctx3 = N::insert_target(consumer_end, ctx2);

        unsafe_run_session(cont, ctx3, provider_end).await;
      }
      None => {
        panic!("impossible happened: received mismatch choice from provider");
      }
    }
  })
}
