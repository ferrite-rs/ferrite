extern crate log;
use std::pin::Pin;
use std::time::Duration;
use std::future::{ Future };
use async_std::task::sleep;

use crate::public::*;

type QueueF < A > =
  InternalChoice <
    End,
    SendChannel <
      A,
      Recurse
    >
  >;

// Now we can define Queue v using FixProcess
type Queue < A > =
  FixProcess <
    QueueF < A >
  >;

type OutputString = SendValue < String, End >;

// We will use a string queue, Queue ((String ∧ End)), where the
// element process output a string value and then terminates.
type StringQueue = Queue < OutputString >;

// Create a partial session that read the strings from the string queue.
// The function is recursive when there are more elements remaining.
fn read_queue_session ()
  ->
    PartialSession <
      ( FixProcess < QueueF < OutputString > >,
        ( Inactive, () )
      ),
      End
    >
{
  partial_session_2 ( move | slot1, slot2 | {
    unfix_session ( slot1,
      case ( slot1, move | option | {
        match option {
          Either::Left(return_left) => {
            return_left(
              wait_async ( slot1, async move || {
                info!("Queue process terminated");
                terminate ()
              }))
          },
          Either::Right(return_right) => {
            return_right(
              receive_channel_from_slot ( slot1, slot2,
                receive_value_from ( slot2, async move | x : String | {
                  info!("Receive value: {}", x);

                  wait_async ( slot2, async move || {
                    read_hole ( slot1,
                      read_queue_session()
                    )
                  })
                })
              )
            )
          }
        }
      })
    )
  })
}

// Create an empty queue session.
fn nil_queue ()
  -> Session < StringQueue >
{
  fix_session(
    offer_left(
      terminate ()
    )
  )
}

// Takes an existing queue session and extend it
// with a new element process that output string.
fn append_queue_2
  < Func >
  ( build_string : Func,
    rest : Session < StringQueue >
  ) ->
    Session < StringQueue >
where
  Func :
    FnOnce() ->
      Pin < Box <
        dyn Future <
          Output = String
        > + Send + 'static
      > >
    + Send + 'static,
{
  fix_session (
    offer_right(
      // TODO: fork() is currently not working with both
      // processes start running at the same time.
      fork(
        send_value_async ( async move || {
          ( build_string().await
          , terminate_nil ()
          )
        }),
        fill_hole ( rest )
      )))
}

fn append_queue
  < Func, Fut >
  ( build_string : Func,
    rest : Session < StringQueue >
  ) ->
    Session < StringQueue >
where
  Func :
    FnOnce() -> Fut
    + Send + 'static,
  Fut :
    Future <
      Output = String
    > + Send + 'static
{
  let build_string2
    : Box <
        dyn FnOnce () ->
          Pin < Box <
            dyn Future < Output = String >
                + Send
          > >
        + Send
      >
  = Box::new ( move || {
      Box::pin ( async move {
        build_string().await
      })
    });

  append_queue_2 (
    build_string2,
    rest
  )
}

pub fn queue_session()
  -> Session < End >
{
  // Create a queue with two elements: "Hello", "World"
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

  let p2
    : Session <
        ReceiveChannel <
          StringQueue,
          End
        >
      >
    = session_2 ( | queue_chan, _ | {
        receive_channel_slot ( queue_chan,
          read_queue_session ()
        )
      });

  let p3 :
    Session < End >
  = apply_channel( p2, p13 );

  p3
}

/*
  Example Output:
    2019-08-27 18:44:25,035 INFO  [session_types_demo] [Main] Running main program
    2019-08-27 18:44:25,035 INFO  [session_types_demo::demo::queue] producing hello..
    2019-08-27 18:44:25,035 INFO  [session_types_demo::demo::queue] producing world..
    2019-08-27 18:44:27,036 INFO  [session_types_demo::demo::queue] Receive value: Hello
    2019-08-27 18:44:28,036 INFO  [session_types_demo::demo::queue] Receive value: World
    2019-08-27 18:44:28,036 INFO  [session_types_demo::demo::queue] Queue process terminated
    2019-08-27 18:44:28,036 INFO  [session_types_demo] [Main] Main program terminating
 */
