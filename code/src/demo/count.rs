extern crate log;

use crate::base::*;
use crate::session::*;
use crate::process::*;

use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use async_std::task::sleep;

type CounterSession = SendValue < i32, End >;

pub fn make_counter_server
  ( initial_count : i32 )
  ->
    PersistentSession <
      CounterSession
    >
{
  let counter1 :
    Arc < Mutex < i32 > >
    = Arc :: new (
        Mutex :: new (initial_count)
      );

  create_persistent_session ( move || {
    info!("[CounterServer] starting new session");
    let counter2 = counter1.clone();

    send_value_async ( async move || {
      info!("[CounterServer] Getting count");
      let mut count1 = counter2.lock().unwrap();
      let count2 : i32 = *count1;
      *count1 += 1;

      ( count2,
        terminate()
      )
    })
  })
}

pub fn make_counter_client
  ( name : String,
    timeout : u64,
    counter_server : PersistentSession < CounterSession >
  ) -> Session < End >
{
  let timer : Session < End > =
    terminate_async ( async move || {
      sleep ( Duration::from_secs(timeout) ).await;
    });

  include_session ( timer, move | timer_chan | {
    wait_async ( timer_chan, async move || {
      info!("[{}] Timer reached", name);
      clone_session(&counter_server, move | counter_chan | {
        receive_value_from ( counter_chan, async move | count | {
          info!("[{}] Received count: {}", name, count);

          wait ( counter_chan, terminate () )
        })
      })
    })
  })
}

pub fn counter_session()
  -> RunnableSession
{
  let counter = make_counter_server( 8 );

  let p1 = make_counter_client ( "P1".to_string(), 8, counter.clone() );
  let p2 = make_counter_client ( "P2".to_string(), 5, counter.clone() );
  let p3 = make_counter_client ( "P3".to_string(), 3, counter.clone() );

  wait_sessions (
    vec! [ p1, p2, p3 ],
    terminate () )
}

/*
  Example Log

  20:04:10,677 INFO  [session_rust] [Main] Running main program
  20:04:13,679 INFO  [session_rust::demo::count] [P3] Received count: 8
  20:04:15,678 INFO  [session_rust::demo::count] [P2] Received count: 9
  20:04:18,678 INFO  [session_rust::demo::count] [P1] Received count: 10
  20:04:18,679 INFO  [session_rust] [Main] Main program terminating
 */