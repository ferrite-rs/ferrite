
// use std::marker::PhantomData;
// use std::sync::mpsc::{ Receiver };

// use crate::base::*;
// use crate::fix::fix::*;
// use crate::process::{ End };

// pub trait ProcessF < R > {
//   type ValueF : Sized + Send;

//   type SenderF : Send;

//   fn create_channel_f()
//     -> ( Self::SenderF, Receiver < Self::ValueF >);

//   fn send_f(sender: Self::SenderF, value: Self::ValueF);
// }

// impl
//   < P, R >
//   ProcessF < R >
//   for P
// where
//   P : Process
// {
//   type ValueF = P::Value;
//   type SenderF = P::Sender;

//   fn create_channel_f()
//     -> ( Self::SenderF, Receiver < Self::ValueF >)
//   {
//     return P::create_channel()
//   }

//   fn send_f(sender: Self::SenderF, value: Self::ValueF)
//   {
//     P::send(sender, value)
//   }
// }

// pub struct Hole {}

// impl
//   < R >
//   ProcessF < R >
//   for Hole
// where
//   R : Process
// {
//   type ValueF = R::Value;
//   type SenderF = R::Sender;

//   fn create_channel_f()
//     -> ( Self::SenderF, Receiver < Self::ValueF >)
//   {
//     return R::create_channel()
//   }

//   fn send_f(sender: Self::SenderF, value: Self::ValueF)
//   {
//     R::send(sender, value)
//   }
// }

// impl
//   < F, R >
//   ProcessF < R >
//   for Fix < F >
// where
//   F : AlgebraT < Hole >,
//   F : AlgebraT < Fix < F > >,
//   <
//     F as AlgebraT < Hole >
//   > :: Algebra
//     : ProcessF < Fix < F > >
// {
//   type ValueF = <
//     <
//       F as AlgebraT < Hole >
//     > :: Algebra
//     as ProcessF < Fix < F > >
//   > :: ValueF;

//   type SenderF = <
//     <
//       F as AlgebraT < Hole >
//     > :: Algebra
//     as ProcessF < Fix < F > >
//   > :: SenderF;

//   fn create_channel_f()
//     -> ( Self::SenderF, Receiver < Self::ValueF > )
//   {
//     return <
//       <
//         F as AlgebraT < Hole >
//       > :: Algebra
//       as ProcessF < Fix < F > >
//     > :: create_channel_f()
//   }

//   fn send_f(sender: Self::SenderF, value: Self::ValueF)
//   {
//     <
//       <
//         F as AlgebraT < Hole >
//       > :: Algebra
//       as ProcessF < Fix < F > >
//     > :: send_f(sender, value)
//   }
// }

// pub struct ProcessT < F >
// where
//   F : ProcessF < End >
// {
//   f : PhantomData < F >
// }

// impl
//   < F >
//   Process
//   for ProcessT < F >
// where
//   F : ProcessF < End >
// {
//   type Value = <
//     F as ProcessF < End >
//   > :: ValueF;

//   type Sender = <
//     F as ProcessF < End >
//   > :: SenderF;

//   fn create_channel()
//     -> ( Self::Sender, Receiver < Self::Value >)
//   {
//     return <
//       F as ProcessF < End >
//     > :: create_channel_f()
//   }

//   fn send(sender: Self::Sender, value: Self::Value)
//   {
//     <
//       F as ProcessF < End >
//     > :: send_f(sender, value)
//   }
// }
