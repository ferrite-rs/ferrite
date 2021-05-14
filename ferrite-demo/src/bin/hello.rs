use std::format;

use ferrite_session::prelude::*;

type HelloSession = ReceiveValue<String, SendValue<String, End>>;

pub fn hello_session() -> Session<End>
{
  let server: Session<HelloSession> =
    receive_value(|name| send_value(format!("Hello, {}!", name), terminate()));

  let client: Session<ReceiveChannel<HelloSession, End>> =
    receive_channel(|x| {
      send_value_to(
        x,
        "John".to_string(),
        receive_value_from(x, move |result| {
          println!("{}", result);
          wait(x, terminate())
        }),
      )
    });

  let main: Session<End> = apply_channel(client, server);

  return main;
}

#[tokio::main]
pub async fn main()
{
  run_session(hello_session()).await
}
