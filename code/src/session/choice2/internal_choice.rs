
use std::pin::Pin;
use std::marker::PhantomData;
use async_std::task;
use async_macros::join;
use std::future::Future;
use async_std::sync::{ Sender, Receiver, channel };

pub use crate::base::{
  Nat,
  Z,
  Empty,
  TyApp,
  Protocol,
  Context,
  ContextLens,
  PartialSession,
  run_partial_session,
  create_partial_session,
};

pub use crate::processes::*;
pub use crate::process::choice2::*;

pub struct SessionCon < I >
  ( PhantomData < I > );

pub struct ContextCon < N, I, P, Row >
  ( PhantomData <( N, I, P, Row )> );

pub struct InternalCont < N, I, P, Row, Root >
  ( PhantomData <( N, I, P, Row, Root )> );

pub struct MakeCont {}

pub struct ReceiverToSelector {}

pub struct RunCont
  < N, C, A, Row >
where
  A : Protocol,
  C : Context,
  Row : Send + 'static,
  Row :
    SumRow < ReceiverCon >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    >,
  < Row as
    SumRow < ReceiverCon >
  > :: Field :
    Send
{
  ins :
    < N :: Deleted
      as Context
    > :: Values,
  sender : Sender < A :: Value >
}

impl < I, P >
  TyApp < P > for
  SessionCon < I >
where
  P : Protocol,
  I : Context,
{
  type Type =
    PartialSession < I, P >;
}

impl < N, I, P, Q, Row >
  TyApp < P > for
  ContextCon < N, I, Q, Row >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >
{
  type Type =
    PartialSession <
      N :: Target,
      Q
    >;
}

impl < N, I, P, Q, Row, Root >
  TyApp < P > for
  InternalCont < N, I, Q, Row, Root >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  type Type =
    Box <
      dyn FnOnce (
        PartialSession <
          N :: Target,
          Q
        >
      ) ->
        Root
      + Send
    >;
}

impl < A >
  LiftFieldBorrow
  < ReceiverCon, (), A >
  for ReceiverToSelector
where
  A : Protocol
{
  fn lift_field_borrow (
    _ : &Receiver < A :: Value >
  ) ->
    ()
  { () }
}

impl < A, B, N, C, Row >
  ElimField <
    Merge <
      ReceiverCon,
      ContextCon < N, C, B, Row >
    >,
    A,
    Pin < Box < dyn Future < Output=() > + Send > >
  > for RunCont < N, C, B, Row >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  Row : Send + 'static,
  Row :
    SumRow < ReceiverCon >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      A,
      Deleted =
        < N as
          ContextLens <
            C,
            InternalChoice < Row >,
            Empty
          >
        > :: Deleted
    >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    >,
  < Row as
    SumRow < ReceiverCon >
  > :: Field :
    Send
{
  fn elim_field (
    self,
    merged :
      MergeField <
        ReceiverCon,
        ContextCon < N, C, B, Row >,
        A
      >
  ) ->
    Pin < Box < dyn Future < Output=() > + Send > >
  {
    let ins1 = self.ins;
    let sender = self.sender;

    let receiver = merged.field1;
    let cont = merged.field2;


    let ins2 =
      < N as
        ContextLens <
          C,
          InternalChoice < Row >,
          A
        >
      > :: merge_channels ( receiver, ins1 );

    Box::pin(
      run_partial_session ( cont, ins2, sender ) )
  }
}


impl
  < A, Root, N, I, P, Row >
  LiftField2
  < (),
    InternalCont < N, I, P, Row, Root >,
    A,
    ContextCon < N, I, P, Row >,
    Root
  > for
  MakeCont
where
  A : Protocol,
  P : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      A
    >
{

  fn lift_field (
    inject :
      impl Fn (
        PartialSession <
          N :: Target,
          P
        >
      ) ->
        Root
      + Send + 'static,
    field : ()
  ) ->
    Box <
      dyn FnOnce (
        PartialSession <
          N :: Target,
          P
        >
      ) ->
        Root
      + Send
    >
  {
    Box::new ( inject )
  }
}

fn id < A > (a : A) -> A {
  a
}

fn make_cont_sum
  < N, I, P, T, Row >
  ( selector :
      < Row as
        SumRow < T >
      > :: Field
  ) ->
    < Row as
      SumRow <
        InternalCont <
          N, I, P, Row,
          < Row as
            SumRow <
              ContextCon < N, I, P, Row >
            >
          > :: Field
        >
      >
    > :: Field
where
  P : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  Row : SumRow < T >,
  Row :
    SumRow <
      ContextCon < N, I, P, Row >
    >,
  Row :
    SumRow <
      InternalCont <
        N, I, P, Row,
        < Row as
          SumRow <
            ContextCon < N, I, P, Row >
          >
        > :: Field
      >
    >,
  Row :
    LiftSum2 <
      T,
      InternalCont <
        N, I, P, Row,
        < Row as
          SumRow <
            ContextCon < N, I, P, Row >
          >
        > :: Field
      >,
      MakeCont,
      ContextCon < N, I, P, Row >,
      < Row as
        SumRow <
          ContextCon < N, I, P, Row >
        >
      > :: Field
    >,
  < Row as
    SumRow <
      ContextCon < N, I, P, Row >
    >
  > :: Field :
    'static,
{
  Row :: lift_sum (
    id,
    selector
  )
}

pub fn case
  < N, C, A, Row, F >
  ( _ : N,
    cont1 : F
  ) ->
    PartialSession < C, A >
where
  A : Protocol,
  C : Context + 'static,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    > + 'static,
  F :
    FnOnce (
      < Row as
        SumRow <
          InternalCont <
            N, C, A, Row,
            < Row as
              SumRow <
                ContextCon < N, C, A, Row >
              >
            > :: Field
          >
        >
      > :: Field
    ) ->
      < Row as
        SumRow <
          ContextCon < N, C, A, Row >
        >
      > :: Field
    + Send + 'static,
  Row : SumRow < () >,
  Row : Send + 'static,
  Row : SumRow < ReceiverCon >,
  < Row as
    SumRow < ReceiverCon >
  >  :: Field
    : Send,
  Row :
    SumRow <
      ContextCon < N, C, A, Row >
    >,
  Row :
    LiftSumBorrow <
      ReceiverCon,
      (),
      ReceiverToSelector
    >,
  Row :
    IntersectSum <
      ReceiverCon,
      ContextCon < N, C, A, Row >
    >,
  Row :
    ElimSum <
      Merge <
        ReceiverCon,
        ContextCon < N, C, A, Row >
      >,
      RunCont < N, C, A, Row >,
      Pin < Box < dyn Future < Output=() > + Send > >
    >,
  Row :
    SumRow <
      InternalCont <
        N, C, A, Row,
        < Row as
          SumRow <
            ContextCon < N, C, A, Row >
          >
        > :: Field
      >
    >,
  Row :
    LiftSum2 <
      (),
      InternalCont <
        N, C, A, Row,
        < Row as
          SumRow <
            ContextCon < N, C, A, Row >
          >
        > :: Field
      >,
      MakeCont,
      ContextCon < N, C, A, Row >,
      < Row as
        SumRow <
          ContextCon < N, C, A, Row >
        >
      > :: Field
    >,
  < Row as
    SumRow < () >
  > :: Field :
    Send,
  < Row as
    SumRow < ReceiverCon >
  > :: Field :
    Send,
  < Row as
    SumRow <
      ContextCon < N, C, A, Row >
    >
  > :: Field : Send,
  < Row as
    SumRow <
      Merge <
        ReceiverCon,
        ContextCon < N, C, A, Row >
      >
    >
  > :: Field :
    Send,
  < Row as
    SumRow <
      InternalCont <
        N, C, A, Row,
        < Row as
          SumRow <
            ContextCon < N, C, A, Row >
          >
        > :: Field
      >
    >
  > :: Field : Send
{
  create_partial_session (
    async move | ins1, sender | {
      let (sum_chan, ins2) =
        < N as
          ContextLens <
            C,
            InternalChoice < Row >,
            Empty
          >
        > :: split_channels ( ins1 );

      let receiver_sum
        : < Row as
            SumRow < ReceiverCon >
          >  :: Field
        =
        sum_chan.recv().await.unwrap();

      let selector
        : < Row as SumRow < () > > :: Field
        = Row::lift_sum_borrow ( &receiver_sum );

      let cont2 = make_cont_sum ::
        < N, C, A, (), Row >
        ( selector );

      let cont3 :
        < Row as
          SumRow <
            ContextCon < N, C, A, Row >
          >
        > :: Field =
        cont1 ( cont2 );

      let cont4 :
        Option <
          < Row as
            SumRow <
              Merge <
                ReceiverCon,
                ContextCon < N, C, A, Row >
              >
            >
          > :: Field
        > =
        Row :: intersect ( receiver_sum, cont3 );

      match cont4 {
        Some ( cont5 ) => {
          let runner
            : RunCont < N, C, A, Row > =
            RunCont {
              ins : ins2,
              sender : sender
            };

          Row :: elim_sum ( runner, cont5 ).await;
        },
        None => {
          panic!(
            "impossible happened: received mismatch choice continuation");
        }
      }
    })
}

type TestSum < A, B, P > =
  Sum <
    PartialSession <
      (A, ()),
      P
    >,
    Sum <
      PartialSession <
        (B, ()),
        P
      >,
      Bottom
    >
  >;

fn make_test_sum
  < A, B, P >
  () ->
    Sum <
      Box <
        dyn FnOnce (
          PartialSession <
            (A, ()),
            P
          >
        ) ->
          TestSum < A, B, P >
        + Send
      >,
      Sum <
        Box <
          dyn FnOnce (
            PartialSession <
              (B, ()),
              P
            >
          ) ->
            TestSum < A, B, P >
          + Send
        >,
        Bottom
      >
    >
where
  A : Protocol,
  B : Protocol,
  P : Protocol,
{
  make_cont_sum ::
    < Z,
      ( InternalChoice <
        ( A, ( B, () ))
      >,
      () ),
      P,
      (),
      ( A, ( B, () ))
    >
    (Sum::Inl(()))
}
