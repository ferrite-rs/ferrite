
use std::pin::Pin;
use std::marker::PhantomData;
use async_std::task;
use async_macros::join;
use std::future::Future;
use async_std::sync::{ Sender, channel };

pub use crate::base::*;
pub use crate::processes::*;
pub use crate::process::choice2::*;

struct SessionCon < I >
  ( PhantomData < I > );

struct ContextCon < N, I, P, Row >
  ( PhantomData <( N, I, P, Row )> );

// struct CaseCont < >

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
  N : Nat,
  P : Process,
  Q : Process,
  I : Processes,
  Row : Send + 'static,
  Row : SumRow < ReceiverCon >,
  < Row as
    SumRow < ReceiverCon >
  > :: Field : Send,
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