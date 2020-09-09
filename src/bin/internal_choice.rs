#![feature(async_closure)]

use ferrite::*;
use ferrite::choice as choice;

pub fn internal_choice_session ()
  -> Session < End >
{
  let client :
    Session <
      ReceiveChannel <
        choice::InternalChoice <
          choice::Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          > >,
        End
      > > =
  receive_channel ( | chan | {
    choice::case ( chan, move | choice1 | {
      match choice1 {
        choice::Either::Left ( ret ) => {
          choice::run_internal_cont ( ret,
            receive_value_from ( chan,
              async move | val: String | {
                println! ("receied string: {}", val);
                wait ( chan,
                  terminate () )
              }) )
        },
        choice::Either::Right ( ret ) => {
          choice::run_internal_cont ( ret,
            send_value_to ( chan, 42,
              wait ( chan,
                terminate () ) ) )
        },
      }
    })
  });

  let provider_left :
    Session <
      choice::InternalChoice <
        choice::Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        >
      >
    > =
      choice::offer_case ( Z(),
        send_value ( "provider_left".to_string(),
          terminate() ) );

  let _provider_right :
    Session <
      choice::InternalChoice <
        choice::Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        >
      >
    > =
      choice::offer_case ( succ(Z()),
        receive_value ( async move | val: u64 | {
          println! ( "received int: {}", val );
          terminate()
        })
      );


  apply_channel ( client, provider_left )
}

#[async_std::main]
pub async fn main() {
  run_session( internal_choice_session() ).await
}
