use ferrite_session::{
  either::*,
  prelude::*,
};

pub fn external_choice_session() -> Session<End>
{
  let provider: Session<
    ExternalChoice<Either<SendValue<String, End>, ReceiveValue<u64, End>>>,
  > = offer_choice! {
    Left => {
      send_value(
        "provider_left".to_string(),
        terminate() )
    }
    Right => {
      receive_value( |val| {
        println! ( "received value: {}", val );
        terminate()
      })
    }
  };

  let _client_left: Session<
    ReceiveChannel<
      ExternalChoice<Either<SendValue<String, End>, ReceiveValue<u64, End>>>,
      End,
    >,
  > = receive_channel(|chan| {
    choose!(
      chan,
      Left,
      receive_value_from(chan, move |val: String| {
        println!("received string: {}", val);

        wait(chan, terminate())
      })
    )
  });

  let client_right: Session<
    ReceiveChannel<
      ExternalChoice<Either<SendValue<String, End>, ReceiveValue<u64, End>>>,
      End,
    >,
  > = receive_channel(|chan| {
    choose!(
      chan,
      Right,
      send_value_to(chan, 42, wait(chan, terminate()))
    )
  });

  apply_channel(client_right, provider)
}

#[tokio::main]
pub async fn main()
{
  run_session(external_choice_session()).await
}
