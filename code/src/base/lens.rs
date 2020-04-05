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
    < P as Protocol > :: Payload
  >;
}

pub struct Empty { }

impl Slot for Empty {
  type Value = ();
}

pub trait ContextLens < C, A1, A2 >
where
  C : Context,
  A1 : Slot,
  A2 : Slot,
{
  type Deleted : Context;
  type Target : Context;

  fn extract_source (
    channels : C :: Values
  ) ->
    ( A1 :: Value,
      < Self::Deleted
        as Context
      > :: Values
    );

  fn insert_target (
    receiver : A2 :: Value,
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
  < C, A1, A2 >
  ContextLens <
    ( A1, C ),
    A1,
    A2
  > for
  Z
where
  A1 : Slot,
  A2 : Slot,
  C : Context
{
  type Deleted = C;
  type Target = (A2, C);

  fn extract_source (
    ctx : ( A1::Value, C::Values )
  ) ->
    ( A1::Value, C::Values )
  {
    ctx
  }

  fn insert_target
    ( p : A2 :: Value,
      r : C :: Values
    ) ->
      ( A2::Value, C::Values )
  {
    (p, r)
  }
}

impl
  < B, A1, A2, C, N >
  ContextLens <
    ( B, C ),
    A1,
    A2
  > for
  S < N >
where
  B : Slot,
  A1 : Slot,
  A2 : Slot,
  C : Context,
  N : ContextLens < C, A1, A2 >,
{
  type Deleted =
    ( B,
      N :: Deleted
    );

  type Target =
    ( B,
      N :: Target
    );

  fn extract_source (
    (p, r1) : ( B::Value, C::Values )
  ) ->
    ( A1 :: Value,
      ( B::Value, < N::Deleted as Context >::Values )
    )
  {
    let (q, r2) = N :: extract_source ( r1 );
    ( q, ( p, r2 ) )
  }

  fn insert_target (
    q : A2 :: Value,
    (p, r1) : ( B::Value, < N::Deleted as Context >::Values )
  ) ->
    ( B::Value, < N::Target as Context >::Values )
  {
    let r2 = N :: insert_target ( q, r1 );
    ( p, r2 )
  }
}
