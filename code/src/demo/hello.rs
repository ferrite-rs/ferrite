extern crate log;

use crate::public::*;

use std::format;

type HelloSession =
  ReceiveValue <
    String,
    SendValue < String, End >
  >;

#[allow(dead_code)]
pub fn hello_session()
  -> Session < End >
{
  let server :
    Session < HelloSession >
  = receive_value ( async move | name | {
      send_value (
        format!("Hello, {}!", name),
        terminate()
      )
    });

  let client :
    Session <
      ReceiveChannel <
        HelloSession,
        End
      > >
  = receive_channel ( | x | {
      send_value_to ( x,
        "John".to_string(),
        receive_value_from ( x,
          async move | result | {
              println! ("{}", result);
              wait ( x,
                terminate()
              ) }) ) });

  let main : Session < End >
    = apply_channel (client, server);

  return main;
}

/*
  Example Log

  12:12:43 INFO  [process_builder_dynamics] [Main] Running main program
  12:12:43 INFO  [process_builder_dynamics::demo::hello] [P1] spending 2 seconds to produce output
  12:12:45 INFO  [process_builder_dynamics::demo::hello] [P1] Spending 3 seconds to cleanup
  12:12:45 INFO  [process_builder_dynamics::demo::hello] [P2] received input from P1: Hello World
  12:12:45 INFO  [process_builder_dynamics::demo::hello] [P2] Spending 5 seconds to cleanup
  12:12:48 INFO  [process_builder_dynamics::demo::hello] [P1] Terminating
  12:12:48 INFO  [process_builder_dynamics::demo::hello] [P3] P1 is terminated
  12:12:50 INFO  [process_builder_dynamics::demo::hello] [P2] Terminating
  12:12:50 INFO  [process_builder_dynamics::demo::hello] [P3] P2 is terminated
  12:12:50 INFO  [process_builder_dynamics::demo::hello] [P3] P1 and P2 are both terminated
  12:12:50 INFO  [process_builder_dynamics::demo::hello] [P3] Spending 1 second to cleanup
  12:12:52 INFO  [process_builder_dynamics::demo::hello] [P3] Terminating
  12:12:52 INFO  [process_builder_dynamics] [Main] Main program terminating

 */
