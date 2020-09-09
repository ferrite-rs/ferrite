#![feature(async_closure)]

use ferrite::*;
use ferrite::choice::nary::*;
use ferrite::choice::nary::either as either;

pub fn external_choice_session ()
  -> Session < End >
{
  let provider :
    Session <
      ExternalChoice <
        either::Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        >
      >
    > =
      offer_choice ( move | choice | {
        match either::extract(choice) {
          either::Left ( cont ) => {
            run_external_cont ( cont,
              send_value ( "provider_left".to_string(),
                terminate() ) )
          }
          either::Right ( cont ) => {
            run_external_cont ( cont,
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
        ExternalChoice <
          either::Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          >
        >,
        End
      >
    > =
      receive_channel (| chan | {
        choose ( chan, either::LeftChoice,
          receive_value_from ( chan,
            async move | val: String | {
              println! ( "received string: {}", val );

              wait ( chan, terminate() )
            }) )
      });

  let client_right :
    Session <
      ReceiveChannel <
        ExternalChoice <
          either::Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          >
        >,
        End
      >
    > =
      receive_channel (| chan | {
        choose ( chan, either::RightChoice,
          send_value_to (chan, 42,
            wait ( chan, terminate () ) ) )
      });

  apply_channel ( client_right, provider )
}

#[async_std::main]
pub async fn main() {
  run_session( external_choice_session() ).await
}
