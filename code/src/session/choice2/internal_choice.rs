
use std::pin::Pin;
use std::marker::PhantomData;
use async_std::task;
use async_macros::join;
use std::future::Future;
use async_std::sync::{ Sender, channel };

pub use crate::base::{
  Nat,
  TyCon,
  Process,
  Processes,
  PartialSession,
  ProcessLens,
};

pub use crate::processes::*;
pub use crate::process::choice2::*;

struct SessionCon < I >
  ( PhantomData < I > );

struct ContextCon < N, I, P, Row >
  ( PhantomData <( N, I, P, Row )> );

struct InternalCont < N, I, P, Row, Root >
  ( PhantomData <( N, I, P, Row, Root )> );

struct MakeCont {}

impl < I, P >
  TyCon < P > for
  SessionCon < I >
where
  P : Process,
  I : Processes,
{
  type Type =
    PartialSession < I, P >;
}

impl < N, I, P, Q, Row >
  TyCon < P > for
  ContextCon < N, I, Q, Row >
where
  P : Process,
  Q : Process,
  I : Processes,
  InternalChoice < Row > :
    Process,
  N :
    ProcessLens <
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
  TyCon < P > for
  InternalCont < N, I, Q, Row, Root >
where
  P : Process,
  Q : Process,
  I : Processes,
  InternalChoice < Row > :
    Process,
  N :
    ProcessLens <
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
  A : Process,
  P : Process,
  I : Processes,
  InternalChoice < Row > :
    Process,
  N :
    ProcessLens <
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
  < N, I, P, Row >
  ( selector :
      < Row as
        SumRow < () >
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
  P : Process,
  I : Processes,
  InternalChoice < Row > :
    Process,
  Row : SumRow < () >,
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
      (),
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

type TestSum =
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
          TestSum
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
            TestSum
          + Send
        >,
        Bottom
      >
    >
where
  A : Process,
  B : Process,
  P : Process,
{
  make_cont_sum ::
    < SelectorZ,
      ( InternalChoice <
        ( A, ( B, () ))
      >,
      () ),
      P,
      ( A, ( B, () ))
    >
    (Sum::Inl(()))
}