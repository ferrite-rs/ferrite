
use std::pin::Pin;
use std::marker::PhantomData;
use std::future::Future;
use async_std::sync::{ Sender, Receiver };

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
  unsafe_create_session,
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
  Row : Iso,
  Row : Send + 'static,
  Row::Canon :
    SumRow < ReceiverCon >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    >,
  < Row::Canon as
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

pub struct InjectSession
  < N, I, P, Q, Row, Root >
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

pub fn run_cont
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
  InternalChoice < Row > :
    Protocol,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  (inject.inject_session)(session)
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
    InjectSession <
      N, I, P, Q, Row, Root
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
  Row : Iso,
  Row : Send + 'static,
  Row::Canon :
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
  < Row::Canon as
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
    ContextCon < N, I, P, Row >,
    Root,
    A,
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
  type RootType = InternalCont < N, I, P, Row, Root >;

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
    _ : ()
  ) ->
    InjectSession < N, I, A, P, Row, Root >
  {
    InjectSession {
      inject_session : Box::new ( inject )
    }
  }
}

fn id < A > (a : A) -> A {
  a
}

fn make_cont_sum
  < N, I, P, T, Row, T3 >
  ( selector :
      < Row::Canon as
        SumRow < T >
      > :: Field
  ) ->
    < Row::Canon as
      LiftSum2 <
        MakeCont,
        T,
        ContextCon < N, I, P, Row >,
        < Row::Canon as
          SumRow <
            ContextCon < N, I, P, Row >
          >
        > :: Field
      >
    > ::RootSum
where
  P : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  Row : Iso,
  Row::Canon : SumRow < T >,
  Row::Canon :
    SumRow <
      ContextCon < N, I, P, Row >
    >,
  Row::Canon :
    LiftSum3 <
      MakeCont,
      T,
      ContextCon < N, I, P, Row >,
      < Row::Canon as
        SumRow <
          ContextCon < N, I, P, Row >
        >
      > :: Field,
      T3
    >,
  < Row::Canon as
    SumRow <
      ContextCon < N, I, P, Row >
    >
  > :: Field :
    'static,
{
  Row::Canon :: lift_sum (
    id,
    selector
  )
}

pub fn case
  < Row, N, C, A, Canon, T3 >
  ( _ : N,
    cont1 : impl FnOnce (
      < Row as
        SumRow <
          T3
        >
      > :: Field
    ) ->
      < Canon as
        SumRow <
          ContextCon < N, C, A, Row >
        >
      > :: Field
      + Send + 'static,
  ) ->
    PartialSession < C, A >
where
  A : Protocol,
  C : Context,
  Row : Send + 'static,
  Row : Iso < Canon = Canon >,
  Row : IsoRow < T3 >,
  Canon : 'static,
  Canon : SumRow < () >,
  N :
    ContextLens <
      C,
      InternalChoice < Row >,
      Empty
    > + 'static,
  Canon : SumRow < ReceiverCon >,
  Canon :
    SumRow <
      ContextCon < N, C, A, Row >
    >,
  Canon :
    LiftSumBorrow <
      ReceiverCon,
      (),
      ReceiverToSelector
    >,
  Canon :
    IntersectSum <
      ReceiverCon,
      ContextCon < N, C, A, Row >
    >,
  Canon :
    ElimSum <
      Merge <
        ReceiverCon,
        ContextCon < N, C, A, Row >
      >,
      RunCont < N, C, A, Row >,
      Pin < Box < dyn Future < Output=() > + Send > >
    >,
  Canon :
    LiftSum3 <
      MakeCont,
      (),
      ContextCon < N, C, A, Row >,
      < Canon as
        SumRow <
          ContextCon < N, C, A, Row >
        >
      > :: Field,
      T3,
    >
{
  unsafe_create_session (
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
        : < Canon as
            SumRow < ReceiverCon >
          >  :: Field
        =
        sum_chan.recv().await.unwrap();

      let selector
        : < Canon as SumRow < () > > :: Field
        = Canon::lift_sum_borrow ( &receiver_sum );

      let cont2 = make_cont_sum ::
        < N, C, A, (), Row, T3 >
        ( selector );

      let cont3 =
        < Row as
          IsoRow <
            T3
          >
        > :: from_canon ( cont2 );

      let cont4 :
        < Canon as
          SumRow <
            ContextCon < N, C, A, Row >
          >
        > :: Field =
        cont1 ( cont3 );

      let cont4 :
        Option <
          < Canon as
            SumRow <
              Merge <
                ReceiverCon,
                ContextCon < N, C, A, Row >
              >
            >
          > :: Field
        > =
        Canon :: intersect ( receiver_sum, cont4 );

      match cont4 {
        Some ( cont5 ) => {
          let runner
            : RunCont < N, C, A, Row > =
            RunCont {
              ins : ins2,
              sender : sender
            };

          Canon :: elim_sum ( runner, cont5 ).await;
        },
        None => {
          panic!(
            "impossible happened: received mismatch choice continuation");
        }
      }
    })
}
