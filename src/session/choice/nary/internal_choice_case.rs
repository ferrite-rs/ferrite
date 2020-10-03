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
  wrap_applied,
  ContextLens,
  PartialSession,
  NaturalTransformation,
  unsafe_run_session,
  unsafe_create_session,
};

use crate::protocol::choice::nary::*;

use super::internal_session::*;

pub struct InjectSessionApp < N, C, B, Row, Root >
  ( PhantomData <( N, C, B, Row, Root )> );

impl < N, C, B, Row, Root > TyCon
  for InjectSessionApp < N, C, B, Row, Root >
where
  N: 'static,
  C: 'static,
  B: 'static,
  Row: 'static,
  Root: 'static,
{}

pub struct InjectSession
  < N, C, A, B, Row, Root >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A
    >,
{
  inject_session :
    Box <
      dyn FnOnce (
        PartialSession <
          N :: Target,
          B
        >
      ) ->
        Root
      + Send
    >
}

impl < N, C, A, B, Row, Root >
  TypeApp < A > for
  InjectSessionApp < N, C, B, Row, Root >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  N : 'static,
  Root : 'static,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A
    >,
{
  type Applied =
    InjectSession <
      N, C, A, B, Row, Root
    >;
}

pub struct ReceiverToSelector {}

impl
  NaturalTransformation
  < ReceiverApp,
    Merge <
      ReceiverApp,
      ()
    >
  >
  for ReceiverToSelector
{
  fn lift < A >
    ( receiver: Applied < ReceiverApp, A > )
    ->
      Applied <
        Merge <
          ReceiverApp,
          ()
        >,
        A
      >
  where
    A: Send + 'static,
  {
    wrap_applied ( (
      receiver,
      wrap_applied( () )
    ) )
  }
}

pub fn run_internal_cont
  < N, C, A, B, Row, Root >
(
  inject :
    InjectSession <
      N, C, A, B, Row, Root
    >,
  session :
    PartialSession <
      N :: Target,
      B
    >
) ->
  Root
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A
    >,
{
  (inject.inject_session)(session)
}

pub struct RunCont
  < N, C, B, Row >
where
  B : Protocol,
  C : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    >,
{
  ctx :
    < N :: Target
      as Context
    > :: Endpoints,
  sender : Sender < B >
}

pub struct ContRunner
  < N, C, A, B, Row, Del >
where
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty,
      Deleted = Del,
    >,
{
  ctx : Del::Endpoints,

  sender : Sender < B >,

  receiver : Receiver < A >,

  phantom: PhantomData <( N, C, Row )>,
}

impl < N, C, A, B, Row, Del >
  NeedInternalSession <
    N, C, A, B, Row, Del,
    Pin < Box < dyn Future < Output=() > + Send > >
  >
  for ContRunner < N, C, A, B, Row, Del >
where
  B : Protocol,
  C : Context,
  Del : Context,
  Row : RowCon,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty,
      Deleted = Del,
    >,
{
  fn on_internal_session
    ( self: Box < Self >,
      cont: InternalSession < N, C, A, B, Row, Del >
    ) ->
      Pin < Box < dyn Future < Output=() > + Send > >
  where
    A : Protocol,
    B : Protocol,
    C : Context,
    Row : RowCon,
    N :
      ContextLens <
        C,
        InternalChoice < Row >,
        A,
        Deleted = Del,
      >
  {
    let ctx1 = self.ctx;
    let sender = self.sender;
    let receiver = self.receiver;

    let ctx2 =
      < N as
        ContextLens <
          C,
          InternalChoice < Row >,
          A
        >
      > :: insert_target ( receiver, ctx1 );

    Box::pin(
      unsafe_run_session (
        cont.session, ctx2, sender
      ) )
  }
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
//   // A : Protocol,
//   B : Protocol,
//   C : Context,
//   N : 'static,
//   Row : Send + 'static,
//   Row :
//     SumRow < ReceiverApp >,
  // N :
  //   ContextLens <
  //     C,
  //     InternalChoice < Row >,
  //     A,
  //     Deleted = Del,
  //   >,
  // N :
  //   ContextLens <
  //     C,
  //     InternalChoice < Row >,
  //     Empty,
  //     Deleted = Del,
  //   >,
  // < Row as
  //   SumRow < ReceiverApp >
  // > :: Field :
  //   Send
// {
//   fn elim_field < A >
//     ( self,
//       a : Applied < F, A >
//     ) ->
//       R
//   where
//     A: 'static,
//   {
//     todo!()
//   }
  // fn elim_field (
  //   self,
  //   (receiver, cont) :
  //     ( Receiver < A >,
  //       PartialSession <
  //         < N as
  //           ContextLens <
  //             C,
  //             InternalChoice < Row >,
  //             A
  //           >
  //         > ::Target,
  //         B
  //       >
  //     )
  // ) ->
  //   Pin < Box < dyn Future < Output=() > + Send > >
  // {
  //   let ctx1 = self.ctx;
  //   let sender = self.sender;

  //   let ctx2 =
  //     < N as
  //       ContextLens <
  //         C,
  //         InternalChoice < Row >,
  //         A
  //       >
  //     > :: insert_target ( receiver, ctx1 );

  //   Box::pin(
  //     unsafe_run_session ( cont, ctx2, sender ) )
  // }
// }

pub struct LiftUnitToSession < N, C, A, Row, Del >
  ( PhantomData <( N, C, A, Row, Del )> );

impl
  < Root, N, C, B, Row, Del >
  FieldLifter < Root >
  for LiftUnitToSession < N, C, B, Row, Del >
where
  B : Protocol,
  C : Context,
  Del : Context,
  N : 'static,
  Root : 'static,
  Row : RowCon,
  InternalChoice < Row > :
    Protocol,
{
  type SourceF = ();

  type TargetF = InternalSessionF < N, C, B, Row, Del >;

  type InjectF =
    InjectSessionApp < N, C, B, Row, Root >;

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

type RootCont < Row, N, C, B, Del > =
  InjectSessionApp <
    N, C, B, Row,
    AppliedSum <
      Row,
      InternalSessionF < N, C, B, Row, Del >
    >
  >;

pub fn case
  < N, C, B, Row, Del >
  ( _ : N,
    cont1 : impl FnOnce (
      AppliedSum <
        Row,
        RootCont < Row, N, C, B, Del >
      >
    ) ->
      AppliedSum <
        Row,
        InternalSessionF < N, C, B, Row, Del >
      >
      + Send + 'static,
  ) ->
    PartialSession < C, B >
where
  B : Protocol,
  C : Context,
  Del : Context,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty,
      Deleted = Del,
    >,
  Row : RowCon,
  Row : SumFunctor,
  Row : IntersectSum,
  Row : ElimSum,
  Row : SumFunctorInject,
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
