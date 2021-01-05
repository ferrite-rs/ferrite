use std::marker::PhantomData;

use crate::functional::nat::{ S, Z };
use crate::base::protocol::{ Protocol };
use crate::base::context::{ Context };
use crate::base::channel::ReceiverOnce;

pub trait Slot : Send + 'static {
  type Endpoint : Send;
}

impl < P > Slot for P
where
  P : Protocol
{
  type Endpoint = ReceiverOnce < P >;
}

pub struct Empty;

impl Slot for Empty {
  type Endpoint = ();
}

pub trait ContextLens < C, A1, A2 >
  : Send + 'static
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

pub trait GenericLensCont < N, C, D, A, B, K >
{
  fn on_lens( self )
    -> K
  where
    C: Context,
    D: Context,
    A: Slot,
    B: Slot,
    N:
      ContextLens <
        C, A, B,
        Deleted = D
      >
  ;
}

pub trait GenericLens < C, D, A >
  : Sized + Send + 'static
{
  fn with_lens < B, K >
    ( cont: impl
        GenericLensCont < Self, C, D, A, B, K > )
    -> K
  where
    B: Slot,
  ;
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

impl < C, A >
  GenericLens <
    ( A, C ),
    C,
    A
  >
  for Z
where
  A: Slot,
  C: Context,
{
  fn with_lens < B, K >
    ( cont: impl
        GenericLensCont <
          Z,
          (A, C),
          C, A, B, K >
    ) -> K
  where
    B: Slot
  {
    cont.on_lens()
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

struct SuccLensCont
  < Cont, N, C, D, A, B1, B2, K >
where
  A: Slot,
  Cont:
    GenericLensCont <
      S < N >,
      (A, C),
      (A, D),
      B1, B2, K
    >
{
  cont: Cont,
  phantom: PhantomData <( N, C, D, A, B1, B2, K )>
}

impl
  < Cont, N, C, D, A, B1, B2, K >
  GenericLensCont <
    N, C, D, B1, B2, K
  >
  for SuccLensCont <
    Cont, N, C, D, A, B1, B2, K
  >
where
  A: Slot,
  Cont:
    GenericLensCont <
      S < N >,
      (A, C),
      (A, D),
      B1, B2, K
    >
{
  fn on_lens( self )
    -> K
  where
    C: Context,
    D: Context,
    B1: Slot,
    B2: Slot,
    N:
      ContextLens <
        C, B1, B2,
        Deleted = D
      >
  {
    self.cont.on_lens()
  }
}

impl < N, C, D, A, B1 >
  GenericLens <
    ( A, C ),
    ( A, D ),
    B1
  >
  for S < N >
where
  C: Context,
  D: Context,
  A: Slot,
  B1: Slot,
  N: GenericLens < C, D, B1 >
{
  fn with_lens < B2, K >
    ( cont1: impl
        GenericLensCont <
          S < N >,
          (A, C),
          (A, D),
          B1, B2, K
        >
    ) -> K
  where
    B2: Slot,
  {
    let cont2 = SuccLensCont ::
      < _, N, C, D, A, B1, B2, K >
      {
        cont: cont1,
        phantom: PhantomData,
      };

    N::with_lens( cont2 )
  }
}
