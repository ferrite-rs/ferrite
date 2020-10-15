use std::time::Duration;
use async_std::task::sleep;

use ferrite::*;

type IntStream = Fix <
  SendValue < u64, Z > >;

fn producer (count: u64) ->
  Session < IntStream >
{
  fix_session (
    send_value! (
      {
        sleep(Duration::from_secs(1)).await;
        println!("[producer] Producing value: {}", count);

        count
      },
      producer ( count + 1 )
    ) )
}

fn consumer < A: Protocol > () ->
  Session <
    ReceiveChannel <
      IntStream,
      A
    >
  >
{
  receive_channel! ( stream => {
    unfix_session_for ( stream,
      receive_value_from! ( stream,
        count => {
          println!("[consumer] Received value: {}", count);
          include_session! (
            consumer (),
            next => {
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

#[async_std::main]
pub async fn main() {
  run_session ( stream_session() ).await;
}
