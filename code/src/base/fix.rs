use crate::base::nat::*;
use async_std::task;
use async_std::sync::{ Sender, Receiver, channel };

pub trait TyCon < A > {
  type Type;
}

pub trait Functor < A, B >
  : TyCon < A > + TyCon < B >
{
  fn fmap (
    mapper: impl FnOnce (A) -> B,
    x : < Self as TyCon < A > > :: Type
  ) ->
    < Self as TyCon < B > > :: Type
  ;
}

pub struct Fix < F >
where
  F : TyCon < Fix < F > >
{
  unfix : Box < F :: Type >
}

pub fn fix < F >
  (x : F :: Type)
  -> Fix < F >
where
  F : TyCon < Fix < F > >
{
  Fix {
    unfix : Box::new ( x )
  }
}

pub fn unfix < F >
  (x : Fix < F >)
  -> F :: Type
where
  F : TyCon < Fix < F > >
{
  *x.unfix
}

// impl < A, F >
//   TyCon < A > for
//   Fix < F >
// where
//   F : TyCon < Fix < F > >,
//   < F as
//     TyCon < Fix < F > >
//   > :: Type :
//     TyCon < A >
// {
//   type Type =
//     < < F as
//         TyCon < Fix < F > >
//       > :: Type
//       as TyCon < A >
//     > :: Type;
// }

impl < A >
  TyCon < A > for
  Zero
{
  type Type = A;
}

impl < A >
  TyCon < A > for
  String
{
  type Type = String;
}

impl < A >
  TyCon < A > for
  ()
{
  type Type = ();
}

impl < A, N >
  TyCon < A > for
  Succ < N >
where
  N : Nat
{
  type Type = N;
}

impl < A, X >
  TyCon < A > for
  Box < X >
where
  X : TyCon < A >
{
  type Type = Box < X :: Type >;
}

impl < A, X >
  TyCon < A > for
  Receiver < X >
where
  X : TyCon < A >
{
  type Type = Receiver < X :: Type >;
}

impl < A, X >
  TyCon < A > for
  Sender < X >
where
  X : TyCon < A >
{
  type Type = Sender < X :: Type >;
}

impl < A, X, Y >
  TyCon < A > for
  ( X, Y )
where
  X : TyCon < A >,
  Y : TyCon < A >,
{
  type Type =
    ( X :: Type,
      Y :: Type
    );
}
impl < A, B >
  Functor < A, B > for
  Zero
{
  fn fmap (
    mapper : impl FnOnce (A) -> B,
    x : A
  ) ->
    B
  {
    mapper ( x )
  }
}

impl < A, B, N >
  Functor < A, B > for
  Succ < N >
where
  N : Nat
{
  fn fmap (
    _ : impl FnOnce (A) -> B,
    x : N
  ) ->
    N
  {
    x
  }
}

impl < A, B, X >
  Functor < A, B > for
  Box < X >
where
  X : Functor < A, B >
{
  fn fmap (
    mapper : impl FnOnce (A) -> B,
    x : Box <
      < X as TyCon < A > > :: Type
    >
  ) ->
    Box <
      < X as TyCon < B > > :: Type
    >
  {
    Box::new (
      X :: fmap (
        mapper,
        *x
      ) )
  }
}

// impl < A, B, X >
//   Functor < A, B > for
//   Receiver < X >
// where
//   X : Functor < A, B >
// {
//   fn fmap (
//     mapper : impl FnOnce (A) -> B,
//     receiver1 : Receiver <
//       < X as TyCon < A > > :: Type
//     >
//   ) ->
//     Receiver <
//       < X as TyCon < B > > :: Type
//     >
//   {
//     let (sender, receiver2) = channel(1);

//     task::spawn ( async move {
//       let val1 = receiver1.recv().await.unwrap();
//       let val2 = mapper (val1);
//       sender.send (val2).await;
//     });

//     receiver2
//   }
// }