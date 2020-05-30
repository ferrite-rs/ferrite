use async_macros::join;
use async_std::task;
use async_std::sync::{ channel };

use crate::base::{
  Nat,
  Slot,
  Empty,
  Context,
  Protocol,
  AppendContext,
  PartialSession,
  unsafe_run_session,
  unsafe_create_session,
};

pub enum First {}
pub enum Second {}

pub trait SplitContext < C, C1 >
where
  C : Context,
  C1 : Context
{
  type Splitted : Context;

  fn split_endpoints ( ctx : C::Endpoints )
    -> ( C1::Endpoints, < Self::Splitted as Context >::Endpoints )
  ;

  fn merge_endpoints
    ( ctx1 : C1::Endpoints,
      ctx2 : < Self::Splitted as Context >::Endpoints )
    -> C::Endpoints
  ;
}

impl SplitContext < (), () > for ()
{
  type Splitted = ();

  fn split_endpoints ( _: () )
    -> ( (), () )
  { ( (), () ) }

  fn merge_endpoints ( _: (), _: () )
  { }
}

impl < C > SplitContext < C, C >
  for First
where
  C : Context
{
  type Splitted = ();

  fn split_endpoints ( ctx : C::Endpoints )
    -> ( C::Endpoints, () )
  {
    ( ctx, () )
  }


  fn merge_endpoints
    ( ctx : C::Endpoints, _ : () )
    -> C :: Endpoints
  {
    ctx
  }
}

impl < C > SplitContext < C, () >
  for Second
where
  C : Context
{
  type Splitted = C;

  fn split_endpoints ( ctx : C::Endpoints )
    -> ( (), C::Endpoints )
  {
    ( (), ctx )
  }


  fn merge_endpoints
    ( _ : (), ctx : C::Endpoints )
    -> C :: Endpoints
  {
    ctx
  }
}

impl < X, A, C, C1, C2 >
  SplitContext
  < ( A, C ), ( A, C1 )>
  for ( First, X )
where
  A : Slot,
  C : Context,
  C1 : Context,
  C2 : Context,
  X : SplitContext < C, C1, Splitted=C2 >
{
  type Splitted = ( Empty, C2 );

  fn split_endpoints
    ( ( a, ctx ): ( A::Endpoint, C::Endpoints ) )
    ->  ( ( A::Endpoint, C1::Endpoints ),
          ( (), C2::Endpoints )
        )
  {
    let ( ctx1, ctx2 ) = X :: split_endpoints ( ctx );
    ( ( a, ctx1 ), ( (), ctx2 ) )
  }

  fn merge_endpoints
    ( ( a, ctx1 ): ( A::Endpoint, C1::Endpoints ),
      ( (), ctx2 ): ( (), C2::Endpoints )
    ) ->
      ( A::Endpoint, C::Endpoints )
  {
    let ctx = X :: merge_endpoints ( ctx1, ctx2 );
    ( a, ctx )
  }
}

impl < X, A, C, C1, C2 >
  SplitContext
  < ( A, C ), ( Empty, C1 ) >
  for ( Second, X )
where
  A : Slot,
  C : Context,
  C1 : Context,
  C2 : Context,
  X : SplitContext < C, C1, Splitted=C2 >
{
  type Splitted = ( A, C2 );

  fn split_endpoints
    ( ( a, ctx ): ( A::Endpoint, C::Endpoints ) )
    ->  ( ( (), C1::Endpoints ),
          ( A::Endpoint, C2::Endpoints )
        )
  {
    let ( ctx1, ctx2 ) = X :: split_endpoints ( ctx );
    ( ( (), ctx1 ), ( a, ctx2 ) )
  }

  fn merge_endpoints
    ( ( (), ctx1 ): ( (), C1::Endpoints ),
      ( a, ctx2 ): ( A::Endpoint, C2::Endpoints )
    ) ->
      ( A::Endpoint, C::Endpoints )
  {
    let ctx = X :: merge_endpoints ( ctx1, ctx2 );
    ( a, ctx )
  }
}

pub fn cut
  < X, C, C1, C2, A, B, F >
  ( cont1 : F,
    cont2 : PartialSession < C2, A >
  ) ->
    PartialSession < C, B >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  C1 : Context,
  C2 : Context,
  X : SplitContext < C, C1, Splitted=C2 >,
  C1 : AppendContext < ( A, () ) >,
  F : FnOnce ( C1::Length )
      -> PartialSession < C1::Appended, B >
{
  let cont3 = cont1 ( C1::Length::nat () );

  unsafe_create_session (
    async move | ctx, sender1 | {
      let ( ctx1, ctx2 ) = X :: split_endpoints ( ctx );
      let ( sender2, receiver2 ) = channel(1);
      let ctx3 = C1::append_context ( ctx1, ( receiver2, () ) );

      let child1 = task::spawn ( async move {
        unsafe_run_session (
          cont3,
          ctx3, sender1
        ).await;
      });

      let child2 = task::spawn ( async {
        unsafe_run_session( cont2, ctx2, sender2 ).await;
      });

      join!(child1, child2).await;
    })
}

/*
  Cut (Communication)

    cont1 :: Δ1, Q, Δ2 ⊢ P    cont2 :: Δ3 ⊢ Q
  ==============================================
       link(cont1, cont2) :: Δ1, Δ2, Δ3 ⊢ P
 */

pub fn cut_append
  < C1, C2, C3, C4, A, B >
  ( cont1 : PartialSession < C3, B >,
    cont2 : PartialSession < C2, A >
  ) ->
    PartialSession < C4, B >
where
  A : Protocol,
  B : Protocol,
  C1 : Context,
  C2 : Context,
  C3 : Context,
  C4 : Context,
  C1 :
    AppendContext <
      (A, ()),
      Appended = C3
    >,
  C1 :
    AppendContext <
      C2,
      Appended = C4
    >,
{
  unsafe_create_session (
    async move | ctx1, b_sender | {
      let (ctx2, ctx3) =
        < C1 as
          AppendContext < C2 >
        > :: split_context (ctx1);

      let (a_sender, a_receiver) = channel(1);

      let ctx4 =
        < C1 as
          AppendContext < (A, ()) >
        > :: append_context ( ctx2, (a_receiver, ()) );

      let child1 = task::spawn(async {
        unsafe_run_session
          ( cont1, ctx4, b_sender
          ).await;
      });

      let child2 = task::spawn(async {
        unsafe_run_session
          ( cont2, ctx3, a_sender
          ).await;
      });

      join!(child1, child2).await;
    })
}
