use crate::internal::{
  base::{
    unsafe_create_session,
    unsafe_run_session,
    Context,
    PartialSession,
    Protocol,
  },
  functional::{
    App,
    Prism,
    RowCon,
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
  Row2: Send + 'static,
  Row2: RowCon,
  Row1: ToRow<Row = Row2>,
  N: Prism<Row2, Elem = A>,
{
  unsafe_create_session::<C, InternalChoice<Row1>, _, _>(
    move |ctx, client_end_sum_sender| async move {
      let (provider_end, client_end) = A::create_endpoints();

      let client_end_sum = N::inject_elem(App::new(client_end));

      client_end_sum_sender.send(client_end_sum).unwrap();

      unsafe_run_session(cont, ctx, provider_end).await;
    },
  )
}
