#![feature(async_closure)]

use ferrite::*;
use ferrite::choice::nary::*;
use ferrite::choice::nary::either::{
  Either,
  LeftChoice,
  RightChoice,
};

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
  receive_channel ( | chan | {
    case ( chan, move | choice | {
      match_choice! { choice;
        either::Left => {
          receive_value_from ( chan,
            async move | val: String | {
              println! ("receied string: {}", val);
              wait ( chan,
                terminate () )
            })
        }
        either::Right => {
          send_value_to ( chan, 42,
            wait ( chan,
              terminate () ) )
        }
      }
    })
  });

  let provider_left :
    Session <
      InternalChoice <
        Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        > > > =
    offer_case ( LeftChoice,
      send_value ( "provider_left".to_string(),
        terminate() ) );

  let _provider_right :
    Session <
      InternalChoice <
        Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        > > > =
    offer_case ( RightChoice,
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
