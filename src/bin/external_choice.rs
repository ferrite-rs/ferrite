#![feature(async_closure)]

use ferrite::*;
use ferrite::choice as choice;

pub fn external_choice_session ()
  -> Session < End >
{
  let provider :
    Session <
      choice::ExternalChoice <
        choice::Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        >
      >
    > =
      choice::offer_choice ( move | choice | {
        match choice {
          choice::Either::Left ( ret ) => {
            choice::run_external_cont ( ret,
              send_value ( "provider_left".to_string(),
                terminate() ) )
          },
          choice::Either::Right ( ret ) => {
            choice::run_external_cont ( ret,
              receive_value ( async move | val | {
                println! ( "received value: {}", val );
                terminate()
              }))
          }
        }
      });

  let _client_left :
    Session <
      ReceiveChannel <
        choice::ExternalChoice <
          choice::Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          >
        >,
        End
      >
    > =
      receive_channel (| chan | {
        choice::choose ( chan, Z(),
          receive_value_from ( chan,
            async move | val: String | {
              println! ( "received string: {}", val );

              wait ( chan, terminate() )
            }) )
      });

  let client_right :
    Session <
      ReceiveChannel <
        choice::ExternalChoice <
          choice::Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          >
        >,
        End
      >
    > =
      receive_channel (| chan | {
        choice::choose ( chan, succ(Z()),
          send_value_to (chan, 42,
            wait ( chan, terminate () ) ) )
      });

  apply_channel ( client_right, provider )
}

#[async_std::main]
pub async fn main() {
  run_session( external_choice_session() ).await
}
