use ferrite_session::*;

use std::time::Duration;
use async_std::task::sleep;

pub fn pair_session()
  -> Session < End >
{

  /*
          cont_builder() :: Prim Int ; End
    =====================================================
      P1 = send_value_async(cont_builder) :: · ⊢ Int ∧ End
   */
  let p1 :
    Session <
      SendValue < u64, End >
    >
  = send_value! (
      {
        println!("[P1] Spending 7 seconds to produce first output");
        sleep(Duration::from_secs(7)).await;
        println!("[P1] Done producing first output - 42");
        42
      },
      terminate! ( {
        println!("[P1] Spending 3 seconds to cleanup");
        sleep(Duration::from_secs(3)).await;
        println!("[P1] Terminating");
      })
    );

  /*
                        cont_builder() :: Prim Str ; End
    ==========================================================================
                cont1 = send_value_async(cont_builder) :: · ⊢ Str ∧ End
    ==========================================================================
      P2 = send_channel_from (cont1) :: (Int ∧ End) ⊢ (Int ∧ End) ⊗ (Str ∧ End)
   */
  let p2 :
     Session <
      ReceiveChannel <
        SendValue< u64, End >,
        SendChannel <
          SendValue< u64, End >,
          SendValue< String, End >
        >
      >
    >
  = receive_channel! ( val_chan => {
      send_channel_from ( val_chan,
        send_value! (
          {
            println!("[P2] Spending 2 seconds to produce second output");
            sleep(Duration::from_secs(2)).await;
            println!("[P2] Done producing second output - Hello World");

            "Hello World".to_string()
          },
          terminate! ({
            println!("[P2] Spending 10 seconds to cleanup");
            sleep(Duration::from_secs(10)).await;
            println!("[P2] Terminating");
          })
      ))
    });

  /*
                        cont_builder4 = terminate_async () :: · ⊢ End
    ===========================================================================
                cont_builder3(Str) = wait_async (cont_builder4) :: End ⊢ End
    ===========================================================================
        cont_builder2() = receive_value_from(cont_builder3) :: Str ∧ End ⊢ End
    ===========================================================================
        cont_builder1(Int) = wait_async (cont_builder2) :: End, Str ∧ End ⊢ End
    ===========================================================================
       cont2 = receive_value_from(cont_builder1) :: Int ∧ End, Str ∧ End ⊢ End
    ===========================================================================
        cont1 = receive_channel_from(cont2) :: (Int ∧ End) ⊗ (Str ∧ End) ⊢ End
    ===========================================================================
            P3 = wait_async (cont1) :: End, (Int ∧ End) ⊗ (Str ∧ End) ⊢ End
   */
  let p3 :
    Session <
      ReceiveChannel <
        SendChannel <
          SendValue<u64, End>,
          SendValue<String, End>
        >,
        ReceiveChannel <
          End,
          End
        >
      >
    >
  = receive_channel! ( str_chan => {
      receive_channel! ( timer_chan => {
        wait! ( timer_chan, {
          println!("[P3] P4 has terminated. Receiving channel from P1");

          receive_channel_from! ( str_chan, int_chan => {
            receive_value_from! ( int_chan, input1 => {
              println!("[P3] Received input from P1: {}", input1);

              wait! ( int_chan, {
                println!("[P3] P1 has terminated");

                receive_value_from! ( str_chan, input2 => {
                  println!("[P3] Received input from P2: {}", input2);

                  wait! ( str_chan, {
                    println!("[P3] P2 has terminated");

                    terminate! ({
                      println!("[P3] Spending 2 seconds to clean up");
                      sleep(Duration::from_secs(2)).await;
                      println!("[P3] Terminating");
                    })
                  })
                })
              })
            })
          })
        })
      })
    });

  /*
    ===============
      P4 :: · ⊢ 1
   */
  let p4 : Session <
    End
  > = terminate! ( {
    println!("[P4] Sleeping for 3 seconds before terminating");
    sleep(Duration::from_secs(2)).await;
    println!("[P4] Terminating");
  });

  let p5 :
    Session <
      SendChannel <
        SendValue< u64, End >,
        SendValue< String, End >
      >
    >
  = apply_channel(p2, p1);

  let p6 : Session < End >
  = apply_channel( apply_channel ( p3, p5), p4 );

  p6
}

#[async_std::main]
pub async fn main() {
  run_session( pair_session () ).await
}
