use crate::internal::{
  base::{
    once_channel,
    unsafe_create_session,
    unsafe_run_session,
    Context,
    ContextLens,
    Fixed,
    PartialSession,
    Protocol,
    Value,
  },
  functional::{
    wrap_type_app,
    AppSum,
    Prism,
    RowCon,
  },
  protocol::ExternalChoice,
};

pub fn choose<N, M, C1, C2, A, B, Row, Choice>(
  _: N,
  _: M,
  cont: PartialSession<C2, A>,
) -> PartialSession<C1, A>
where
  Choice: Protocol,
  Choice: Fixed<Unfixed = ExternalChoice<Row>>,
  C1: Context,
  C2: Context,
  A: Protocol,
  B: Protocol,
  Row: RowCon,
  N: ContextLens<C1, Choice, B, Target = C2>,
  M: Prism<Row, Elem = B>,
{
  unsafe_create_session(move |ctx1, sender1| async move {
    let (receiver1, ctx2) = N::extract_source(ctx1);

    let choice: AppSum<Row, ()> = M::inject_elem(wrap_type_app(()));

    let payload = receiver1.recv().await.unwrap();

    let ExternalChoice { sender: sender2 } = Choice::unfix(payload);

    let (sender3, receiver3) = once_channel();

    sender2.send((Value(choice), sender3)).unwrap();

    let receiver_sum = receiver3.recv().await.unwrap();

    let m_receiver = M::extract_elem(receiver_sum);

    match m_receiver {
      Some(receiver4) => {
        let ctx3 = N::insert_target(receiver4.get_applied(), ctx2);

        unsafe_run_session(cont, ctx3, sender1).await;
      }
      None => {
        panic!("impossible happened: received mismatch choice from provider");
      }
    }
  })
}
