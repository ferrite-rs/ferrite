#![feature(async_closure)]

use ferrite::*;

pub fn external_choice_session ()
  -> Session < End >
{
  let provider :
    Session <
      ExternalChoice <
        Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        > > > =
    offer_choice! {
      Left => {
        send_value ( "provider_left".to_string(),
          terminate() )
      }
      Right => {
        receive_value ( async move | val | {
          println! ( "received value: {}", val );
          terminate()
        })
      }
    };

  let _client_left :
    Session <
      ReceiveChannel <
        ExternalChoice <
          Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          >
        >,
        End
      > > =
    receive_channel (| chan | {
      choose ( chan, LeftLabel,
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
          Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          >
        >,
        End
      > > =
    receive_channel (| chan | {
      choose ( chan, RightLabel,
        send_value_to (chan, 42,
          wait ( chan, terminate () ) ) )
    });

  apply_channel ( client_right, provider )
}

#[async_std::main]
pub async fn main() {
  run_session( external_choice_session() ).await
}
