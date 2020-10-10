#![feature(async_closure)]

pub use ferrite::*;

use std::time::Duration;
use async_std::task::sleep;

type Producer = SendValue < String, End >;

fn cut_session ()
  -> Session < End >
{
  let client :
    Session <
      ReceiveChannel <
        Producer,
        ReceiveChannel <
          Producer,
          ReceiveChannel <
            Producer,
            End > > > >
  = receive_channel ( | c1 | {
      receive_channel ( | c2 | {
        receive_channel ( | c3 | {
          < ( R, ( L, ( R, () ) ) ) > :: cut (
            receive_value_from! ( c2, x2 => {
              println! ("[right] got x2: {}", x2);
              sleep(Duration::from_secs(1)).await;

              wait! ( c2,
                terminate! ({
                  println! ("[right] terminating");
                }) )
            }),
            | c4 | {
              receive_value_from! ( c1, x1 => {
                println! ("[left] got x1: {}", x1);

                receive_value_from! ( c3, x3 => {
                  println! ("[left] got x3: {}", x3);

                  wait! ( c1,
                    wait! ( c3,
                      wait! ( c4,
                        terminate! ({
                          println! ("[left] terminating");
                        }) ) ) )
                })
              })
            }
          )
        })
      })
    });

  let p1 : Session < Producer >
    = send_value ( "foo".to_string(), terminate () );

  let p2 : Session < Producer >
    = send_value ( "bar".to_string(), terminate () );

  let p3 : Session < Producer >
    = send_value ( "baz".to_string(), terminate () );

  apply_channel (
    apply_channel (
      apply_channel (
        client,
        p1
      ),
      p2
    ),
    p3
  )
}

#[async_std::main]
pub async fn main() {
  run_session( cut_session() ) .await;
}
