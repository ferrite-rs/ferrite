extern crate log;

use ferrite::*;
use ferrite::choice as choice;

pub fn choice2_demo ()
  -> Session < End >
{
  let _client :
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
              async move | val | {
                info! ("receied value: {}", val);
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

  let _provider_left :
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
        receive_value ( async move | val | {
          info! ( "received value: {}", val );
          terminate()
        })
      );

  let _provider :
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
                info! ( "received value: {}", val );
                terminate()
              }))
          }
        }
      });

  unimplemented!()
}
