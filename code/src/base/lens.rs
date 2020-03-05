use async_std::sync::Receiver;

use crate::base::nat::{ S, Z };
use crate::base::protocol::{ Protocol };
use crate::base::context::{ Context };

pub trait Slot : 'static {
  type Value : Send;
}

impl < P > Slot for P
where
  P : Protocol
{
  type Value = Receiver <
    < P as Protocol > :: Value
  >;
}

pub struct Empty { }

impl Slot for Empty {
  type Value = ();
}

pub trait ContextLens < I, P1, P2 >
where
  I : Context,
  P1 : Slot,
  P2 : Slot,
{
  type Deleted : Context;
  type Target : Context;

  fn split_channels (
    channels :
      < I as Context > :: Values
  ) ->
    ( < P1 as Slot > :: Value,
      < Self::Deleted
        as Context
      > :: Values
    );

  fn merge_channels (
    receiver : < P2 as Slot > :: Value,
    channels :
      < Self::Deleted
        as Context >
      :: Values
  ) ->
    < Self::Target
      as Context
    > :: Values;
}


impl
  < P1, P2, R >
  ContextLens <
    ( P1, R ),
    P1,
    P2
  > for
  Z
where
  P1 : Slot,
  P2 : Slot,
  R : Context
{
  type Deleted = R;
  type Target = (P2, R);

  fn split_channels (
    (p, r) :
      < ( P1, R )
        as Context
      > :: Values
  ) ->
    ( < P1 as Slot > :: Value,
      < R as Context
      > :: Values
    )
  {
    return (p, r);
  }

  fn merge_channels
    ( p : < P2 as Slot > :: Value,
      r :
        < R as Context
        > :: Values
    ) ->
      < ( P2, R )
        as Context
      > :: Values
  {
    return (p, r);
  }
}

impl
  < P, Q1, Q2, R, N >
  ContextLens <
    ( P, R ),
    Q1,
    Q2
  > for
  S < N >
where
  P : Slot,
  Q1 : Slot,
  Q2 : Slot,
  R : Context,
  N : ContextLens < R, Q1, Q2 >,
{
  type Deleted =
    ( P,
      N :: Deleted
    );

  type Target =
    ( P,
      N :: Target
    );

  fn split_channels (
    (p, r1) :
      < ( P, R ) as Context >
      :: Values
  ) ->
    ( < Q1 as Slot > :: Value,
      < ( P,
          N :: Deleted
        ) as Context
      > :: Values
    )
  {
    let (q, r2) =
      < N as ContextLens < R, Q1, Q2 >
      > :: split_channels ( r1 );

    return ( q, ( p, r2 ) );
  }

  fn merge_channels (
    q : < Q2 as Slot > :: Value,
    (p, r1) :
      < ( P,
          N ::Deleted
        ) as Context
      > :: Values
  ) ->
    < ( P,
        N :: Target
      ) as Context
    > :: Values
  {
    let r2 =
      < N as ContextLens < R, Q1, Q2 >
      > :: merge_channels ( q, r1 );

    return ( p, r2 );
  }
}
