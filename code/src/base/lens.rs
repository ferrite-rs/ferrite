use async_std::sync::Receiver;

use crate::base::nat::{ S, Z };
use crate::base::protocol::{ Protocol };
use crate::base::context::{ Context };

pub trait Slot : 'static {
  type Endpoint : Send;
}

impl < P > Slot for P
where
  P : Protocol
{
  type Endpoint = Receiver < P >;
}

pub struct Empty { }

impl Slot for Empty {
  type Endpoint = ();
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
    channels : C :: Endpoints
  ) ->
    ( A1 :: Endpoint,
      < Self::Deleted
        as Context
      > :: Endpoints
    );

  fn insert_target (
    receiver : A2 :: Endpoint,
    channels :
      < Self::Deleted
        as Context >
      :: Endpoints
  ) ->
    < Self::Target
      as Context
    > :: Endpoints;
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
    ctx : ( A1::Endpoint, C::Endpoints )
  ) ->
    ( A1::Endpoint, C::Endpoints )
  {
    ctx
  }

  fn insert_target
    ( p : A2 :: Endpoint,
      r : C :: Endpoints
    ) ->
      ( A2::Endpoint, C::Endpoints )
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
    (p, r1) : ( B::Endpoint, C::Endpoints )
  ) ->
    ( A1 :: Endpoint,
      ( B::Endpoint, < N::Deleted as Context >::Endpoints )
    )
  {
    let (q, r2) = N :: extract_source ( r1 );
    ( q, ( p, r2 ) )
  }

  fn insert_target (
    q : A2 :: Endpoint,
    (p, r1) : ( B :: Endpoint, < N::Deleted as Context >::Endpoints )
  ) ->
    ( B::Endpoint, < N::Target as Context >::Endpoints )
  {
    let r2 = N :: insert_target ( q, r1 );
    ( p, r2 )
  }
}
