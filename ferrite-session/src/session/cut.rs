use std::future::Future;
use async_macros::join;
use async_std::task;

use crate::base::*;
use crate::functional::nat::*;

pub enum L {}
pub enum R {}

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

impl < X, C, C1, C2 >
  SplitContext
  < ( Empty, C ) >
  for ( Empty, X )
where
  C : Context,
  C1 : Context,
  C2 : Context,
  X : SplitContext < C, Left = C1, Right = C2 >
{
  type Left = ( Empty, C1 );
  type Right = ( Empty, C2 );

  fn split_endpoints
    ( ( a, ctx ): ( (), C::Endpoints ) )
    ->  ( ( (), C1::Endpoints ),
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
  for ( L, X )
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
  for ( R, X )
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
    < A, B, Fut >
    ( cont1 : PartialSession < Self::Left, A >,
      cont2 : impl FnOnce
        ( < Self::Right as Context > ::Length )
        -> Fut
    ) ->
      PartialSession < C, B >
  where
    A : Protocol,
    B : Protocol,
    Self::Right : AppendContext < ( A, () ) >,
    Fut :
      Future < Output =
        PartialSession <
          < Self::Right
            as AppendContext < ( A, () ) >
          > ::Appended, B
        >
      >
      + Send + 'static
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
    < A, B, Fut >
    ( cont1 : PartialSession < Self::Left, A >
    , cont2 : impl FnOnce
        ( < Self::Right as Context > ::Length )
        -> Fut
    ) ->
      PartialSession < C, B >
  where
    A : Protocol,
    B : Protocol,
    Self::Right : AppendContext < ( A, () ) >,
    Fut :
      Future < Output =
        PartialSession <
          < Self::Right
            as AppendContext < ( A, () ) >
          > ::Appended, B
        >
      >
      + Send + 'static
  {
    cut :: < X, _, _, _, _, _, _, _> ( cont1, cont2 )
  }
}

pub fn cut
  < X, C, C1, C2, A, B, Func, Fut >
  ( cont1 : PartialSession < C1, A >,
    cont2 : Func
  ) ->
    PartialSession < C, B >
where
  A : Protocol,
  B : Protocol,
  C : Context,
  C1 : Context,
  C2 : Context,
  X : SplitContext < C, Left = C1, Right = C2 >,
  C2 : AppendContext < ( A, () ) >,
  Func : FnOnce ( C2::Length ) -> Fut,
  Fut :
    Future < Output =
      PartialSession < C2::Appended, B >
    >
    + Send + 'static
{
  let cont3 = cont2 ( C2::Length::nat () );

  unsafe_create_session (
    move | ctx, sender1 | async move {
      let ( ctx1, ctx2 ) = X :: split_endpoints ( ctx );
      let ( sender2, receiver2 ) = once_channel();
      let ctx3 = C2::append_context ( ctx2, ( receiver2, () ) );

      let child1 = task::spawn ( async move {
        unsafe_run_session (
          cont3.await,
          ctx3, sender1
        ).await;
      });

      let child2 = task::spawn ( async {
        unsafe_run_session( cont1, ctx1, sender2 ).await;
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
    move | ctx1, b_sender | async move {
      let (ctx2, ctx3) =
        < C1 as
          AppendContext < C2 >
        > :: split_context (ctx1);

      let (a_sender, a_receiver) = once_channel();

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
