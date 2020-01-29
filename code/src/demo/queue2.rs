extern crate log;
use std::time::Duration;
use std::future::{ Future };
use async_std::task::sleep;
use std::pin::Pin;

use crate::base::*;
use crate::process::fix2::*;
use crate::session::fix2::*;

use crate::process::{
  InternalChoice,
  SendValue,
  ReceiveChannel,
  Either,
  End
};

use crate::session::{
  wait,
  forward,
  offer_left,
  offer_right,
  case,
  apply_channel,
  terminate,
  include_session,
  receive_channel,
  send_value_async,
  receive_value_from,
  send_channel_to,
};

type Queue < A > =
  FixProcess <
    InternalChoice <
      End,
      SendValue < A, Zero >,
    >
  >;

type StringQueue = Queue < String >;

fn nil_queue < A > ()
  -> Session < Queue < A > >
where
  A : Send + 'static
{
  fix_session (
    offer_left (
      terminate ()
    ) )
}

fn append_queue_2
  < A, Func >
  ( builder : Func,
    rest : Session < Queue < A > >
  ) ->
    Session < Queue < A > >
where
  A : Send + 'static,
  Func :
    FnOnce() ->
      Pin < Box <
        dyn Future <
          Output = A
        > + Send + 'static
      > >
    + Send + 'static,
{
  fix_session (
    offer_right (
      send_value_async ( async move || {
        ( builder ().await
        , rest
        )
      }) ) )
}

fn append_queue
  < A, Func, Fut >
  ( builder : Func,
    rest : Session < Queue < A > >
  ) ->
    Session < Queue < A > >
where
  A : Send + 'static,
  Func :
    FnOnce () -> Fut
    + Send + 'static,
  Fut :
    Future < Output = A > + Send
{
  let builder2
    : Box <
        dyn FnOnce () ->
          Pin < Box <
            dyn Future < Output = A >
                + Send
          > >
        + Send
      >
  = Box::new ( move || {
      Box::pin ( async move {
        builder().await
      })
    });

  append_queue_2 (
    builder2,
    rest
  )
}

fn read_queue () ->
  Session <
    ReceiveChannel <
      StringQueue,
      End
    > >
{
  receive_channel ( | queue | {
    unfix_session ( queue,
      case ( queue, move | option | {
        match option {
          Either::Left( ret ) => {
            ret ( wait (
              queue, terminate () )
            ) },
          Either::Right( ret ) => {
            ret (
              receive_value_from ( queue,
                async move | val | {
                  info!("Receive value: {}", val);

                  include_session (
                    read_queue (),
                    | next | {
                      send_channel_to (
                        next,
                        queue,
                        forward ( next )
                      ) })
                } ) ) } } }) ) }) }

pub fn queue_session () ->
  Session < End >
{
  let p11
    : Session < StringQueue >
  = nil_queue ();

  let p12
    : Session < StringQueue >
  = append_queue ( async || {
      info!("producing world..");
      sleep(Duration::from_secs(3)).await;
      "World".to_string()
    },
    p11
  );

  let p13
    : Session < StringQueue >
  = append_queue ( async || {
      info!("producing hello..");
      sleep(Duration::from_secs(2)).await;
      "Hello".to_string()
    },
    p12
  );

  apply_channel ( read_queue (), p13 )
}
