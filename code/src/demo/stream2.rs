extern crate log;
use std::time::Duration;
use async_std::task::sleep;

use crate::public::*;

type IntStream = FixProtocol <
  SendValue < i32, Z > >;

fn producer (count: i32) ->
  Session < IntStream >
{
  fix_session (
    send_value_async ( async move || {
      sleep(Duration::from_secs(1)).await;
      info!("Producing value: {}", count);

      ( count,
        producer ( count + 1 )
      )
    }))
}

fn consumer < A: Protocol > () ->
  Session <
    ReceiveChannel <
      IntStream,
      A
    >
  >
{
  receive_channel ( | stream | {
    unfix_session_for ( stream,
      receive_value_from ( stream,
        async move | count | {
          info!("Received value: {}", count);
          include_session (
            consumer (),
            | next | {
              send_channel_to (
                next,
                stream,
                forward ( next )
              )
            })
      }) )
  })
}

pub fn stream_session () ->
  Session < End >
{
  let p1 = producer ( 0 );
  let p2 = consumer ();

  apply_channel ( p2, p1 )
}
