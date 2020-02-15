extern crate log;

use crate::public::*;
use crate::public::choice as choice;

type Choice =
  choice::Either <
    SendValue < String, End >,
    ReceiveValue < i32, End >
  >;

pub fn choice2_demo ()
  -> Session < End >
{
  let client :
    Session <
      ReceiveChannel <
        choice::InternalChoice < Choice >,
        End
      >
    > =
  receive_channel ( | chan | {
    choice::case ( chan, move | choice1 | {
      match choice1 {
        choice::Either::Left ( ret ) => {
          choice::run_cont ( ret,
            receive_value_from ( chan,
              async move | val | {
                info! ("receied value: {}", val);
                wait ( chan,
                  terminate () )
              })
          )
        },
        choice::Either::Right ( ret ) => {
          choice::run_cont ( ret,
            send_value_to ( chan, 42,
              wait ( chan,
                terminate () ) ) )
        },
      }
    })
  });

  unimplemented!()
}
