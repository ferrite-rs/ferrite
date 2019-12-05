extern crate log;
use std::time::Duration;
use std::marker::PhantomData;
use std::pin::Pin;
use std::future::{ Future };
use async_std::task::sleep;

use crate::base::*;
use crate::session::*;
use crate::process::*;
use crate::processes::*;
use crate::fix::*;


/*
  We want to define the recursive session type:

    Queue e = End ⊕ (e ⊗ Queue v)

  Where e is the element process.
  But it not possible as Rust does not support recursive types.

  So instead we use recursion scheme and define an algebra
  for queue:

    QueueF e r = End ⊕ (e ⊗ r)

  Using the fixed point process type we can fix the recursive
  part of QueueF:

    Fix (QueueF e) ~ Queue v
*/

type QueueF < E, R > =
  InternalChoice <
    End,
    SendChannel <
      E,
      R
    >
  >;

/*
  QueueT is a dummy data type used with the AlgebraT trait
  to simulate recursion schemes in Rust. We can't do the
  direct style like in Haskell, because Rust does not support
  higher ranked polymorphism.
 */

struct QueueT < A > {
  a : PhantomData < A >
}

// The algebra for QueueF exists for all r, including itself.
impl
  < A, R >
  AlgebraT < R >
  for QueueT < A >
where
  A : Process,
  R : Process
{
  type Algebra = QueueF < A, R >;
}

// Now we can define Queue v using FixProcess
type Queue < A > = FixProcess < QueueT < A > >;

type OutputString = SendValue < String, End >;

// We will use a string queue, Queue ((String ∧ End)), where the
// element process output a string value and then terminates.
type StringQueue = Queue < OutputString >;

// Create a partial session that read the strings from the string queue.
// The function is recursive when there are more elements remaining.
fn read_queue_session ()
  ->
    PartialSession <
      ( FixProcess < QueueT < OutputString > >,
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
              wait_async ( slot1, move || {
                Box::pin ( async move {
                  info!("Queue process terminated");
                  terminate ()
                })
              }))
          },
          Either::Right(return_right) => {
            return_right(
              receive_channel_from_slot ( slot1, slot2,
                receive_value_from ( slot2, move | x : String | {
                  Box::pin ( async move {
                    info!("Receive value: {}", x);

                    wait_async ( slot2, move || {
                      Box::pin ( async move {
                        read_hole ( slot1,
                          read_queue_session()
                        )
                      })
                    })
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
fn append_queue
  < F >
  ( build_string : F,
    rest : Session < StringQueue >
  ) ->
    Session < StringQueue >
where
  F: FnOnce() ->
        Pin < Box < dyn Future <
          Output = String
        > + Send > >
      + Send + 'static
{
  fix_session (
    offer_right(
      // TODO: fork() is currently not working with both
      // processes start running at the same time.
      fork(
        send_value_async ( || {
          Box::pin ( async move {
            ( build_string().await
            , terminate_nil ()
            )
          })
        }),
        fill_hole ( rest )
      )))
}

#[allow(dead_code)]
pub fn queue_session()
  -> RunnableSession
{
  // Create a queue with two elements: "Hello", "World"
  let p11
    : Session < StringQueue >
  = nil_queue ();

  let p12
    : Session < StringQueue >
  = append_queue( || {
      Box::pin ( async move {
        info!("producing world..");
        sleep(Duration::from_secs(3)).await;
        "World".to_string()
      })
    },
    p11
  );

  let p13
    : Session < StringQueue >
  = append_queue(
    || {
      Box::pin ( async move {
        info!("producing hello..");
        sleep(Duration::from_secs(2)).await;
        "Hello".to_string()
      })
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
    RunnableSession
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
