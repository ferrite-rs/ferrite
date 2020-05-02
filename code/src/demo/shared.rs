extern crate log;

use crate::public::*;

use std::time::Duration;
use async_std::task::sleep;

type Counter =
  LinearToShared <
    SendValue < i32, Z >
  >;

pub fn make_counter_session
  ( count : i32 ) ->
    SuspendedSharedSession < Counter >
{
  accept_shared_session (
    send_value_async ( async move || {
      info!("[Server] Producing count {}", count);
      sleep(Duration::from_secs(1)).await;
      info!("[Server] Produced count {}", count);

      ( count,
        detach_shared_session (
          make_counter_session ( count + 1 ) ) )
    }) ) }

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

  acquire_shared_session ( shared, move | counter | {
    receive_value_from ( counter, async move | count | {
      info!("[{}] Received count: {}", name, count);
      sleep(Duration::from_secs(1)).await;

      release_shared_session ( counter, {
        if stop_at <= count {
          info!("[{}] terminating", name);
          terminate()
        } else {
          partial_session (
            read_counter_session ( name, stop_at, shared2 ) )
        }
      }) }) }) }

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