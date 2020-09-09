#![feature(async_closure)]

use ferrite::*;
use ferrite::choice::nary::*;
use ferrite::choice::nary::either::*;
use ferrite::choice::nary::either as either;

pub fn internal_choice_session ()
  -> Session < End >
{
  let client :
    Session <
      ReceiveChannel <
        InternalChoice <
          either::Either <
            SendValue < String, End >,
            ReceiveValue < u64, End >
          > >,
        End
      > > =
  receive_channel ( | chan | {
    case ( chan, move | choice1 | {
      match either::extract(choice1) {
        either::Left ( cont ) => {
          run_internal_cont ( cont,
            receive_value_from ( chan,
              async move | val: String | {
                println! ("receied string: {}", val);
                wait ( chan,
                  terminate () )
              }) )
        }
        either::Right ( cont ) => {
          run_internal_cont ( cont,
            send_value_to ( chan, 42,
              wait ( chan,
                terminate () ) ) )
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
        >
      >
    > =
      offer_case ( LeftChoice,
        send_value ( "provider_left".to_string(),
          terminate() ) );

  let _provider_right :
    Session <
      InternalChoice <
        Either <
          SendValue < String, End >,
          ReceiveValue < u64, End >
        >
      >
    > =
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
