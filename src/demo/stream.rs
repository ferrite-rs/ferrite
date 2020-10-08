extern crate log;
use std::time::Duration;
use async_std::task::sleep;

use ferrite::*;

struct WrapIntStream;

impl Wrapper for WrapIntStream
{ type Unwrap = IntStream; }

type IntStream = SendValue < u64, Wrap < WrapIntStream > >;

fn producer (count: u64) ->
  Session < IntStream >
{
  send_value_async ( async move || {
    sleep(Duration::from_secs(1)).await;
    info!("Producing value: {}", count);

    ( count,
      rec_wrap_session ( producer ( count + 1 ) )
    )
  })
}

fn consumer () ->
  Session <
    ReceiveChannel <
      IntStream,
      End
    >
  >
{
  receive_channel ( | stream | {
    receive_value_from ( stream,
      async move | count | {
        info!("Received value: {}", count);
        rec_unwrap_session ( stream,
          include_session (
            consumer (),
            | next | {
              send_channel_to (
                next,
                stream,
                forward ( next )
              )
            })
        )
    })
  })
}

pub fn stream_session () ->
  Session < End >
{
  let p1 = producer ( 0 );
  let p2 = consumer ();

  apply_channel ( p2, p1 )
}
