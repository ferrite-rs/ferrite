#![feature(async_closure)]

use ferrite::*;
use ferrite::choice::nary::*;

mod foobar {
  use ferrite::*;

  define_choice! {
    Foo: SendValue < String, End >,
    Bar: ReceiveValue < u64, End >,
    Baz: End,
  }
}

pub fn external_choice_session ()
  -> Session < End >
{
  let provider :
    Session <
      ExternalChoice <
        foobar::Protocol
      >
    > =
      offer_choice ( move | choice | {
        match foobar::extract(choice) {
          foobar::Foo ( cont ) => {
            run_external_cont ( cont,
              send_value ( "provider_foo".to_string(),
                terminate() ) )
          }
          foobar::Bar ( cont ) => {
            run_external_cont ( cont,
              receive_value ( async move | val | {
                println! ( "received bar value: {}", val );
                terminate()
              }))
          }
          foobar::Baz ( cont ) => {
            run_external_cont ( cont,
              terminate() )
          }
        }
      });

  let client_bar :
    Session <
      ReceiveChannel <
        ExternalChoice <
          foobar::Protocol
        >,
        End
      >
    > =
      receive_channel (| chan | {
        choose ( chan, foobar::BarLabel,
          send_value_to (chan, 42,
            wait ( chan, terminate () ) ) )
      });

  apply_channel ( client_bar, provider )
}

#[async_std::main]
pub async fn main() {
  run_session( external_choice_session() ).await
}