use std::marker::PhantomData;

use crate::base::*;

pub enum Sum < P, Q > {
  Inl ( P ),
  Inr ( Q )
}

pub trait ProcessField < P >
where
  P : Process
{
  type Value : Send;
}

pub trait ConvergeSum < T, P, R >
where
  P : Process,
  T : ProcessField < P >
{
  fn match_proc
    ( self,
      val : T :: Value ) ->
      R
  ;
}

pub trait ProcessSum < T >
{
  type ValueSum : Send;
}

pub trait ProcessPrism
  < S, P, T >
where
  P : Process,
  S : ProcessSum < T >,
  T : ProcessField < P >
{
  fn intro_sum
    ( val : T :: Value )
    -> S :: ValueSum;

  fn elim_sum
    ( val_sum : S :: ValueSum )
    ->
      Option <
        T :: Value
      >;
}

pub trait MergeProcessSum < T1, T2 >
  : ProcessSum < T1 >
  + ProcessSum < T2 >
  + ProcessSum <
      ToMerge < T1, T2 >
    >
{
  fn merge_sum (
    sum1 :
      < Self as
        ProcessSum < T1 >
      > :: ValueSum,
    sum2 :
      < Self as
        ProcessSum < T2 >
      > :: ValueSum
  ) ->
    Option <
      < Self as
        ProcessSum <
          ToMerge < T1, T2 >
        >
      > :: ValueSum
    >
  ;
}

pub struct ToValue { }

pub struct ToUnit { }

pub struct ToSession < I >
{
  i : PhantomData < I >
}

pub struct ToMerge < T1, T2 >
{
  t1 : PhantomData < T1 >,
  t2 : PhantomData < T2 >
}

impl
  < T, P >
  ProcessSum < T >
  for P
where
  P : Process,
  T : ProcessField < P >,
{
  type ValueSum = T :: Value;
}

impl
  < P, T >
  ProcessPrism
  < P, P, T >
  for Zero
where
  P : Process,
  T : ProcessField < P >
{
  fn intro_sum
    ( val : T :: Value )
    -> T :: Value
  {
    val
  }

  fn elim_sum
    ( val : T :: Value )
    ->
      Option < T :: Value >
  {
    Some ( val )
  }
}

impl
  < P, R, T >
  ProcessPrism <
    (P, R),
    P,
    T
  >
  for Zero
where
  P : Process,
  R : ProcessSum < T >,
  T : ProcessField < P >
{
  fn intro_sum
    ( val : T :: Value )
    ->
      Sum <
        T :: Value,
        R :: ValueSum
      >
  {
    Sum::Inl ( val )
  }

  fn elim_sum
    ( val_sum :
      Sum <
        T :: Value,
        R :: ValueSum
      >
    ) ->
      Option <
        T :: Value
      >
  {
    match val_sum {
      Sum::Inl ( val ) => {
        Option::Some ( val )
      },
      Sum::Inr ( _ ) => {
        Option::None
      }
    }
  }
}

impl
  < N, Q, P, R, T >
  ProcessPrism <
    (Q, R),
    P,
    T
  >
  for Succ < N >
where
  N : Nat,
  P : Process,
  Q : Process,
  R : ProcessSum < T >,
  T : ProcessField < P >,
  T : ProcessField < Q >,
  N : ProcessPrism < R, P, T >
{

  fn intro_sum
    ( val :
        < T as
          ProcessField < P >
        > :: Value )
    ->
      Sum <
        < T as
          ProcessField < Q >
        > :: Value,
        R :: ValueSum
      >
  {
    Sum::Inr (
      N :: intro_sum ( val )
    )
  }

  fn elim_sum
    ( val_sum :
      Sum <
        < T as
          ProcessField < Q >
        > :: Value,
        R :: ValueSum
      >
    ) ->
      Option <
        < T as
          ProcessField < P >
        > :: Value
      >
  {
    match val_sum {
      Sum::Inl ( _ ) => {
        Option::None
      },
      Sum::Inr ( val_sum2 ) => {
        N :: elim_sum ( val_sum2 )
      }
    }
  }
}


impl
  < T, P, R >
  ProcessSum < T >
  for (P, R)
where
  P : Process,
  T : ProcessField < P >,
  R : ProcessSum < T >
{
  type ValueSum =
    Sum <
      T :: Value,
      R :: ValueSum
    >;
}

impl < P >
  ProcessField < P >
  for ToValue
where
  P : Process
{
  type Value = P :: Value;
}

impl < P >
  ProcessField < P >
  for ToUnit
where
  P : Process
{
  type Value = ();
}

impl < I, P >
  ProcessField < P >
  for ToSession < I >
where
  P : Process,
  I : Processes
{
  type Value =
    PartialSession < I, P >;
}

impl
  < P, T1, T2 >
  ProcessField < P >
  for ToMerge < T1, T2 >
where
  P : Process,
  T1 : ProcessField < P >,
  T2 : ProcessField < P >,
{
  type Value =
    ( T1 :: Value,
      T2 :: Value
    );
}