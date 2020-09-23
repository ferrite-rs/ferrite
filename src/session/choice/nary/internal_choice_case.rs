use std::pin::Pin;
use std::marker::PhantomData;
use std::future::Future;
use async_std::sync::{ Sender, Receiver };

use crate::base::{
  Empty,
  TyCon,
  TypeApp,
  Applied,
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  NaturalTransformation,
  unsafe_run_session,
  unsafe_create_session,
};

use crate::protocol::choice::nary::*;

pub struct SessionApp < N, C, A, Row >
  ( PhantomData <( N, C, A, Row )> );

pub struct InjectSessionApp < N, C, A, Row, Root >
  ( PhantomData <( N, C, A, Row, Root )> );

impl < N, I, Q, Row > TyCon
  for SessionApp < N, I, Q, Row > {}

impl < N, I, Q, Row, Root > TyCon
  for InjectSessionApp < N, I, Q, Row, Root > {}

impl < N, I, P, Q, Row >
  TypeApp < P > for
  SessionApp < N, I, Q, Row >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  Row : Send + 'static,
  Row :
    SumRow < ReceiverApp >,
  < Row as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >
{
  type Applied =
    PartialSession <
      N :: Target,
      Q
    >;
}

impl < N, I, P, Q, Row, Root >
  TypeApp < P > for
  InjectSessionApp < N, I, Q, Row, Root >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  Row : Send + 'static,
  Row :
    SumRow < ReceiverApp >,
  < Row as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  type Applied =
    InjectSession <
      N, I, P, Q, Row, Root
    >;
}

pub struct ReceiverToSelector {}

impl
  NaturalTransformation
  < ReceiverApp, () >
  for ReceiverToSelector
{
  fn lift < A >
    ( fa: Applied < ReceiverApp, A > )
    -> Applied < (), A >
  { 
    todo!() 
  }
}

pub struct RunCont
  < N, C, A, Row >
where
  A : Protocol,
  C : Context,
  Row : Send + 'static,
  Row :
    SumRow < ReceiverApp >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    >,
  < Row as
    SumRow < ReceiverApp >
  > :: Field :
    Send
{
  ctx :
    < N :: Deleted
      as Context
    > :: Endpoints,
  sender : Sender < A >
}

pub struct InjectSession
  < N, I, P, Q, Row, Root >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  Row : Send + 'static,
  Row :
    SumRow < ReceiverApp >,
  < Row as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  inject_session :
    Box <
      dyn FnOnce (
        PartialSession <
          N :: Target,
          Q
        >
      ) ->
        Root
      + Send
    >
}

pub fn run_internal_cont
  < N, I, P, Q, Row, Root >
(
  inject :
    InjectSession <
      N, I, P, Q, Row, Root
    >,
  session :
    PartialSession <
      N :: Target,
      Q
    >
) ->
  Root
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  Row :
    Send + 'static,
  Row :
    SumRow < ReceiverApp >,
  < Row as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  (inject.inject_session)(session)
}

// impl < B, N, C, Row >
//   ElimField <
//     Merge <
//       ReceiverApp,
//       SessionApp < N, C, B, Row >
//     >,
//     Pin < Box < dyn Future < Output=() > + Send > >
//   > for RunCont < N, C, B, Row >
// where
//   A : Protocol,
//   B : Protocol,
//   C : Context,
//   Row : Send + 'static,
//   Row :
//     SumRow < ReceiverApp >,
//   N :
//     ContextLens <
//       C,
//       InternalChoice < Row >,
//       A,
//       Deleted =
//         < N as
//           ContextLens <
//             C,
//             InternalChoice < Row >,
//             Empty
//           >
//         > :: Deleted
//     >,
//   N :
//     ContextLens <
//       C,
//       InternalChoice < Row >,
//       Empty
//     >,
//   < Row as
//     SumRow < ReceiverApp >
//   > :: Field :
//     Send
// {
//   fn elim_field (
//     self,
//     (receiver, cont) :
//       ( Receiver < A >,
//         PartialSession <
//           < N as
//             ContextLens <
//               C,
//               InternalChoice < Row >,
//               A
//             >
//           > ::Target,
//           B
//         >
//       )
//   ) ->
//     Pin < Box < dyn Future < Output=() > + Send > >
//   {
//     let ctx1 = self.ctx;
//     let sender = self.sender;

//     let ctx2 =
//       < N as
//         ContextLens <
//           C,
//           InternalChoice < Row >,
//           A
//         >
//       > :: insert_target ( receiver, ctx1 );

//     Box::pin(
//       unsafe_run_session ( cont, ctx2, sender ) )
//   }
// }

pub struct LiftUnitToSession < N, C, A, Row >
  ( PhantomData <( N, C, A, Row )> );

impl
  < Root, N, I, P, Row >
  FieldLifter < Root >
  for LiftUnitToSession < N, I, P, Row >
where
  P : Protocol,
  I : Context,
  Row : Send + 'static,
  Row :
    SumRow < ReceiverApp >,
  < Row as
    SumRow < ReceiverApp >
  >  :: Field
    : Send,
  InternalChoice < Row > :
    Protocol,
{
  type SourceF = ();

  type TargetF = SessionApp < N, I, P, Row >;

  type InjectF =
    InjectSessionApp < N, I, P, Row, Root >;

  fn lift_field < A >
    ( self,
      inject:
        impl Fn
          ( Applied < Self::TargetF, A > )
          -> Root
        + Send + 'static,
      row:
        Applied < Self::SourceF, A >
    ) ->
      Applied < Self::InjectF, A >
  {
    todo!()
    // InjectSession {
    //   inject_session : Box::new ( inject )
    // }
  }
}

type RootCont < Row, N, C, A > =
  InjectSessionApp <
    N, C, A, Row,
    AppliedSum <
      Row,
      SessionApp < N, C, A, Row >
    >
  >;

pub fn case
  < Row, N, C, A >
  ( _ : N,
    cont1 : impl FnOnce (
      AppliedSum < 
        Row,
        RootCont < Row, N, C, A >
      >
    ) ->
      AppliedSum <
        Row,
        SessionApp < N, C, A, Row >
      >
      + Send + 'static,
  ) ->
    PartialSession < C, A >
where
  A : Protocol,
  C : Context,
  Row : Send + 'static,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    >,
  Row : RowCon,
  Row : SumFunctor,
  Row : IntersectSum,
  Row : ElimSum,
  // Row : SumFunctorInject,
{
  unsafe_create_session (
    async move | ctx1, sender | {
      // let (sum_chan, ctx2) =
      //   < N as
      //     ContextLens <
      //       C,
      //       InternalChoice < Row >,
      //       Empty
      //     >
      //   > :: extract_source ( ctx1 );

      // let InternalChoice { field : receiver_sum }
      //   = sum_chan.recv().await.unwrap();

      // let selector
      //   : < Row as SumRow < () > > :: Field
      //   = Row::lift_sum_borrow ( &receiver_sum );

      // let cont3 =
      //   Row :: lift_sum3 (
      //     LiftUnitToSession(PhantomData),
      //     selector
      //   );

      // let cont4 :
      //   < Row as
      //     SumRow <
      //       SessionApp < N, C, A, Row >
      //     >
      //   > :: Field =
      //   cont1 ( cont3 );

      // let cont5 :
      //   Option <
      //     < Row as
      //       SumRow <
      //         Merge <
      //           ReceiverApp,
      //           SessionApp < N, C, A, Row >
      //         >
      //       >
      //     > :: Field
      //   > =
      //   Row :: intersect ( receiver_sum, cont4 );

      // match cont5 {
      //   Some ( cont6 ) => {
      //     let runner
      //       : RunCont < N, C, A, Row > =
      //       RunCont {
      //         ctx : ctx2,
      //         sender : sender
      //       };

      //     Row :: elim_sum ( runner, cont6 ).await;
      //   },
      //   None => {
      //     panic!(
      //       "impossible happened: received mismatch choice continuation");
      //   }
      // }
    })
}
