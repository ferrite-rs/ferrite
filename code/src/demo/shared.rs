extern crate log;

use crate::public::*;

use rand::prelude::*;
use std::time::Duration;
use async_std::task::sleep;

type Counter =
  LinearToShared <
    SendValue < i32, Z >
  >;

async fn random_sleep(start: u64, end: u64) {
  let sleep_time = thread_rng().gen_range(start, end);
  sleep( Duration::from_millis ( sleep_time ) ).await;
}

pub fn make_counter_session
  ( count : i32 ) ->
    SuspendedSharedSession < Counter >
{
  accept_shared_session (
    send_value_async ( async move || {
      info!("[Server] Producing count {}", count);
      random_sleep(100, 1000).await;
      info!("[Server] Produced count {}", count);

      ( count,
        detach_shared_session (
          make_counter_session ( count + 1 ) ) )
    })
  )
}

pub fn read_counter_session
  ( name : String
  , stop_at : i32
  , shared:
      SharedSession <
        LinearToShared <
          SendValue < i32, Z >
        > >
  ) -> Session < End >
{
  let shared2 = shared.clone();

  step ( async move || {
    random_sleep(100, 2000).await;

    acquire_shared_session ( shared, async move | counter | {
      receive_value_from ( counter, async move | count | {
        info!("[{}] Received count: {}", name, count);

        release_shared_session ( counter, {
          if stop_at <= count {
            info!("[{}] terminating", name);
            terminate()
          } else {
            partial_session (
              read_counter_session ( name, stop_at, shared2 ) )
          }
        })
      })
    })
  })
}

pub fn read_counter_session_2
  ( shared_counter:
      SharedSession <
        LinearToShared <
          SendValue < i32, Z >
        > >
  ) -> Session < End >
{
  acquire_shared_session ( shared_counter, async move | linear_counter | {
    random_sleep(100, 2000).await;
    receive_value_from ( linear_counter, async move | count | {
      info!("Received count: {}", count);
      release_shared_session ( linear_counter,
        terminate() )
    })
  })
}

pub fn shared_counter_session ()
  -> Session < End >
{
  let (shared, _) =
    run_shared_session ( make_counter_session ( 0 ));

  let p1 = read_counter_session ( "P1".to_string(), 10, shared.clone() );
  let p2 = read_counter_session ( "P2".to_string(), 8, shared.clone() );
  let p3 = read_counter_session ( "P3".to_string(), 12, shared.clone() );

  wait_sessions (
    vec! [ p1, p2, p3 ],
    terminate () )
}