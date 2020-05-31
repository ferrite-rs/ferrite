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

pub enum AllLeft {}
pub enum AllRight {}


pub trait SplitContext < C >
where
  C : Context
{
  type Left : Context;
  type Right : Context;

  fn split_endpoints ( ctx : C::Endpoints )
    ->  ( < Self::Left as Context >::Endpoints,
          < Self::Right as Context >::Endpoints )
  ;
}

impl SplitContext < () > for ()
{
  type Left = ();
  type Right = ();

  fn split_endpoints ( _: () )
    -> ( (), () )
  { ( (), () ) }
}

impl < C > SplitContext < C >
  for AllLeft
where
  C : Context
{
  type Left = C;
  type Right = ();

  fn split_endpoints ( ctx : C::Endpoints )
    -> ( C::Endpoints, () )
  {
    ( ctx, () )
  }
}

impl < C > SplitContext < C >
  for AllRight
where
  C : Context
{
  type Left = ();
  type Right = C;

  fn split_endpoints ( ctx : C::Endpoints )
    -> ( (), C::Endpoints )
  {
    ( (), ctx )
  }
}

impl < X, A, C, C1, C2 >
  SplitContext
  < ( A, C ) >
  for ( First, X )
where
  A : Slot,
  C : Context,
  C1 : Context,
  C2 : Context,
  X : SplitContext < C, Left = C1, Right = C2 >
{
  type Left = ( A, C1 );
  type Right = ( Empty, C2 );

  fn split_endpoints
    ( ( a, ctx ): ( A::Endpoint, C::Endpoints ) )
    ->  ( ( A::Endpoint, C1::Endpoints ),
          ( (), C2::Endpoints )
        )
  {
    let ( ctx1, ctx2 ) = X :: split_endpoints ( ctx );
    ( ( a, ctx1 ), ( (), ctx2 ) )
  }
}

impl < X, A, C, C1, C2 >
  SplitContext
  < ( A, C ) >
  for ( Second, X )
where
  A : Slot,
  C : Context,
  C1 : Context,
  C2 : Context,
  X : SplitContext < C, Left = C1, Right = C2 >
{
  type Left = ( Empty, C1 );
  type Right = ( A, C2 );

  fn split_endpoints
    ( ( a, ctx ): ( A::Endpoint, C::Endpoints ) )
    ->  ( ( (), C1::Endpoints ),
          ( A::Endpoint, C2::Endpoints )
        )
  {
    let ( ctx1, ctx2 ) = X :: split_endpoints ( ctx );
    ( ( (), ctx1 ), ( a, ctx2 ) )
  }
}

pub trait Cut < C >
  : SplitContext < C >
where
  C : Context,
{
  fn cut
    < A, B >
    ( cont1 : impl FnOnce
        ( < Self::Left as Context > ::Length )
        ->
          PartialSession <
            < Self::Left
              as AppendContext < ( A, () ) >
            > ::Appended, B
          >,
      cont2 : PartialSession < Self::Right, A >
    ) ->
      PartialSession < C, B >
  where
    A : Protocol,
    B : Protocol,
    Self::Left : AppendContext < ( A, () ) >,
  ;
}

impl < X, C >
  Cut < C >
  for X
where
  C : Context,
  X : SplitContext < C >
{
  fn cut
    < A, B >
    ( cont1 : impl FnOnce
        ( < Self::Left as Context > ::Length )
        ->
          PartialSession <
            < Self::Left
              as AppendContext < ( A, () ) >
            > ::Appended, B
          >,
      cont2 : PartialSession < Self::Right, A >
    ) ->
      PartialSession < C, B >
  where
    A : Protocol,
    B : Protocol,
    Self::Left : AppendContext < ( A, () ) >,
  {
    cut :: < X, _, _, _, _, _, _> ( cont1, cont2 )
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
  X : SplitContext < C, Left = C1, Right = C2 >,
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
