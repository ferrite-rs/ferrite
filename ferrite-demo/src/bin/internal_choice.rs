use ferrite_session::*;

pub fn internal_choice_session ()
  -> Session < End >
{
  let client :
    Session <
      ReceiveChannel <
        InternalChoice <
          Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          > >,
        End
      > > =
  receive_channel! ( chan => {
    case! { chan ;
      Left => {
        receive_value_from! ( chan,
          (val: String) => {
            println! ("receied string: {}", val);
            wait! ( chan,
              terminate! () )
          })
      }
      Right => {
        send_value_to! ( chan,
          42,
          wait! ( chan,
            terminate! () ) )
      }
    }
  });

  let provider_left :
    Session <
      InternalChoice <
        Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        > > > =
    offer_case! ( Left,
      send_value! (
        "provider_left".to_string(),
        terminate! ()
      ) );

  let _provider_right :
    Session <
      InternalChoice <
        Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        > > > =
    offer_case! ( Right,
      receive_value! ( (val: u64) => {
        println! ( "received int: {}", val );
        terminate! ()
      })
    );


  apply_channel ( client, provider_left )
}

#[tokio::main]
pub async fn main() {
  run_session( internal_choice_session() ).await
}
