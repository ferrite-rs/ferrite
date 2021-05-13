use ferrite_session::{
  either::*,
  prelude::*,
};

type Stream = Rec<
  ExternalChoice<
    Either<
      Rec<InternalChoice<Either<SendValue<String, Z>, S<Z>>>>,
      ReceiveValue<String, End>,
    >,
  >,
>;

fn producer() -> Session<Stream>
{
  fix_session(offer_choice! {
    Left =>
      fix_session(offer_case!(
        Left,
        send_value(
          "foo".to_string(),
          fix_session(offer_case!(
            Left,
            send_value(
              "bar".to_string(),
              fix_session(offer_case!(Right, producer()))
            )
          ))
        )))
    Right =>
      receive_value(| str | {
        println!("[producer] received string: {}", str);
        terminate()
      })
  })
}

fn consume_input() -> Session<
  ReceiveChannel<
    RecX<HList![Stream], InternalChoice<Either<SendValue<String, Z>, S<Z>>>>,
    End,
  >,
>
{
  receive_channel(|chan| {
    unfix_session(
      chan,
      case! { chan;
        Left =>
          receive_value_from(chan, move | val | {
            println!("[consumer] received value: {}", val);
            include_session(consume_input(), | consume |
              send_channel_to(consume, chan,
                wait(consume, terminate())))
          }),
        Right =>
          unfix_session(chan,
            choose!(chan, Right,
              send_value_to(chan,
                "hello".to_string(),
                wait(chan,
                  terminate()))))
      },
    )
  })
}

fn consumer() -> Session<ReceiveChannel<Stream, End>>
{
  include_session(consume_input(), |consume| {
    receive_channel(|chan| {
      unfix_session(
        chan,
        choose!(
          chan,
          Left,
          send_channel_to(consume, chan, wait(consume, terminate()))
        ),
      )
    })
  })
}

#[tokio::main]
pub async fn main()
{
  run_session(apply_channel(consumer(), producer())).await
}
