use std::time::Duration;

use ferrite_session::prelude::*;
use tokio::time::sleep;

pub fn pair_session() -> Session<End>
{
  let p1: Session<SendValue<u64, End>> = step(async {
    println!("[P1] Spending 7 seconds to produce first output");
    sleep(Duration::from_secs(7)).await;
    println!("[P1] Done producing first output - 42");

    send_value(
      42,
      terminate_async(|| async {
        println!("[P1] Spending 3 seconds to cleanup");
        sleep(Duration::from_secs(3)).await;
        println!("[P1] Terminating");
      }),
    )
  });

  let p2: Session<
    ReceiveChannel<
      SendValue<u64, End>,
      SendChannel<SendValue<u64, End>, SendValue<String, End>>,
    >,
  > = receive_channel(|val_chan| {
    send_channel_from(
      val_chan,
      step(async move {
        println!("[P2] Spending 2 seconds to produce second output");
        sleep(Duration::from_secs(2)).await;
        println!("[P2] Done producing second output - Hello World");

        send_value(
          "Hello World".to_string(),
          terminate_async(|| async {
            println!("[P2] Spending 10 seconds to cleanup");
            sleep(Duration::from_secs(10)).await;
            println!("[P2] Terminating");
          }),
        )
      }),
    )
  });

  let p3: Session<
    ReceiveChannel<
      SendChannel<SendValue<u64, End>, SendValue<String, End>>,
      ReceiveChannel<End, End>,
    >,
  > = receive_channel(|str_chan| {
    receive_channel(|timer_chan| {
      wait(
        timer_chan,
        step(async move {
          println!("[P3] P4 has terminated. Receiving channel from P1");

          receive_channel_from(str_chan, |int_chan| {
            receive_value_from(int_chan, move |input1| {
              println!("[P3] Received input from P1: {}", input1);

              wait(
                int_chan,
                step(async move {
                  println!("[P3] P1 has terminated");

                  receive_value_from(str_chan, move |input2| {
                    println!("[P3] Received input from P2: {}", input2);

                    wait(
                      str_chan,
                      step(async move {
                        println!("[P3] P2 has terminated");

                        terminate_async(|| async {
                          println!("[P3] Spending 2 seconds to clean up");
                          sleep(Duration::from_secs(2)).await;
                          println!("[P3] Terminating");
                        })
                      }),
                    )
                  })
                }),
              )
            })
          })
        }),
      )
    })
  });

  let p4: Session<End> = terminate_async(|| async {
    println!("[P4] Sleeping for 3 seconds before terminating");

    sleep(Duration::from_secs(2)).await;

    println!("[P4] Terminating");
  });

  let p5: Session<SendChannel<SendValue<u64, End>, SendValue<String, End>>> =
    apply_channel(p2, p1);

  let p6: Session<End> = apply_channel(apply_channel(p3, p5), p4);

  p6
}

#[tokio::main]

pub async fn main()
{
  run_session(pair_session()).await
}
