use std::{
  time::Duration,
  vec::*,
};

use ferrite_session::*;
use tokio::time::sleep;

pub fn concat_session() -> Session<End>
{
  let p1 : Session<
    ReceiveValue<
      Vec<String>,
      ReceiveValue<Vec<String>, SendValue<String, End>>,
    >,
  > = receive_value! ( (l1: Vec < String >) => {
    println!("[P1] Received first list: [{}]", l1.join(", "));

    receive_value! ( (l2: Vec < String >) => {
      println!("[P1] Received second list: [{}]", l2.join(", "));

      send_value! (
        {
          println!("[P1] Spending 3 seconds to concat lists");
          sleep(Duration::from_secs(3)).await;

          let l5 = {
            let mut l3 = l1;
            let mut l4 = l2;
            l3.append(&mut l4);
            l3
          };

          l5.join(", ")
        },
        terminate! ({
          println!("[P1] Spending 2 seconds to cleanup");
          sleep(Duration::from_secs(2)).await;
          println!("[P1] Terminating");
        })
      )
    })
  });

  let p2 : Session<
    ReceiveChannel<
      ReceiveValue<
        Vec<String>,
        ReceiveValue<Vec<String>, SendValue<String, End>>,
      >,
      End,
    >,
  > = receive_channel! ( val_chan => {
      send_value_to! ( val_chan,
        {
          println!("[P2] spending 2 seconds to produce ['hello', 'world']");
          sleep(Duration::from_secs(2)).await;

          vec!
            [ "hello".to_string()
            , "world".to_string()
            ]
        },
        send_value_to! ( val_chan,
          {
            println!("[P2] spending 1 second to produce ['foo', 'bar', 'baz']");
            sleep(Duration::from_secs(1)).await;

            vec!
              [ "foo".to_string()
              , "bar".to_string()
              , "baz".to_string()
              ]
          },
          receive_value_from! ( val_chan, res => {
            println!("[P2] received result from P1: [{}]", res);

            wait! ( val_chan, {
              println!("[P2] P1 has terminated");

              terminate! ({
                println!("[P2] Spending 1 second to cleanup");
                sleep(Duration::from_secs(1)).await;
                println!("[P2] Terminating");
              })
            })
          })
        )
      )
  });

  let p3 : Session<End> = apply_channel(p2, p1);

  return p3;
}

#[tokio::main]

pub async fn main()
{
  run_session(concat_session()).await;
}
