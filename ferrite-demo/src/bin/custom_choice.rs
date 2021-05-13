use ferrite_session::prelude::*;

define_choice! { FooBarBaz;
  Foo : SendValue < String, End >,
  Bar : ReceiveValue < u64, End >,
  Baz : End,
  Quux : SendValue < i32, End >,
}

pub fn external_choice_session() -> Session<End>
{
  let provider: Session<ExternalChoice<FooBarBaz>> = offer_choice! {
    Foo => {
      send_value! (
        "provider_foo".to_string(),
        terminate() )
    }
    Bar => {
      receive_value! ( val => {
        println! ( "received bar value: {}", val );
        terminate()
      })
    }
    Baz => {
      terminate()
    }
    Quux => {
      send_value! (
        8,
        terminate() )
    }
  };

  let client_bar: Session<ReceiveChannel<ExternalChoice<FooBarBaz>, End>> = receive_channel! ( chan => {
    choose! ( chan, Bar,
      send_value_to (chan, 42,
        wait ( chan, terminate () ) )
    )
  });

  apply_channel(client_bar, provider)
}

#[tokio::main]

pub async fn main()
{
  run_session(external_choice_session()).await
}
