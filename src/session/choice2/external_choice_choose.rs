use async_std::sync::{ Receiver, channel };

pub use crate::base::{
  Nat,
  Z,
  Empty,
  TypeApp,
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

pub use crate::context::*;
pub use crate::protocol::choice2::*;

pub fn choose
  < N, M, C, A, B, Row, Canon >
  ( _ : N,
    _ : M,
    cont : PartialSession < N::Target, A >
  ) ->
    PartialSession < C, A >
where
  C : Context,
  A : Protocol,
  B : Protocol,
  Row : Send + 'static,
  Row : Iso < Canon = Canon >,
  Canon : SumRow < () >,
  Canon : SumRow < ReceiverApp >,
  N :
    ContextLens <
      C,
      ExternalChoice < Row >,
      B
    >,
  M :
    Prism <
      Canon,
      ReceiverApp,
      Elem = Receiver < B >
    >,
  M :
    Prism <
      Canon,
      (),
      Elem = ()
    >,
{
  unsafe_create_session (
    async move | ctx1, sender1 | {
      let (receiver1, ctx2) = N::extract_source(ctx1);

      let choice =
        < M as
          Prism < Canon, () >
        > :: inject_elem ( () );

      let ExternalChoice { sender: sender2 } =
        receiver1.recv().await.unwrap();

      let (sender3, receiver3) = channel(1);

      sender2.send((choice, sender3)).await;

      let receiver_sum = receiver3.recv().await.unwrap();

      let m_receiver =
        < M as
          Prism < Canon, ReceiverApp >
        > :: extract_elem(receiver_sum);

      match m_receiver {
        Some(receiver4) => {
          let ctx3 = N::insert_target(receiver4, ctx2);
          unsafe_run_session ( cont, ctx3, sender1 ).await;
        },
        None => {
          panic!(
            "impossible happened: received mismatch choice from provider");
        }
      }
    })
}
