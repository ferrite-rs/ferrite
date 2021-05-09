use ferrite_session::{
  either::*,
  prelude::*,
};

pub type Stream = Rec1<
  InternalChoice<
    Either<
      Rec1<InternalChoice<Either<SendValue<String, Z>, S<Z>>>>,
      ReceiveValue<String, Z>,
    >,
  >,
>;

pub fn stream_producer() -> Session<Stream>
{
  fix_session(offer_case!(
    Left,
    fix_session(offer_case!(
      Left,
      send_value(
        "foo".to_string(),
        fix_session(offer_case!(
          Left,
          send_value(
            "bar".to_string(),
            fix_session(offer_case!(Right, stream_producer()))
          )
        ))
      )
    ))
  ))
}

#[tokio::main]
pub async fn main() {}
