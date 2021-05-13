use std::time::Duration;

use ferrite_session::prelude::*;
use tokio::time::sleep;

struct WrapIntStream;

impl Wrapper for WrapIntStream
{
  type Unwrap = IntStream;
}

type IntStream = SendValue<u64, Wrap<WrapIntStream>>;

fn producer(count: u64) -> Session<IntStream>
{
  send_value!(
    {
      sleep(Duration::from_secs(1)).await;

      println!("Producing value: {}", count);

      count
    },
    wrap_session(producer(count + 1))
  )
}

fn consumer() -> Session<ReceiveChannel<IntStream, End>>
{
  receive_channel! ( stream => {
    receive_value_from! ( stream,
      count => {
        println!("Received value: {}", count);
        unwrap_session ( stream,
          include_session! (
            consumer (),
            next => {
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

pub fn stream_session() -> Session<End>
{
  let p1 = producer(0);

  let p2 = consumer();

  apply_channel(p2, p1)
}

#[tokio::main]

pub async fn main()
{
  run_session(stream_session()).await
}
