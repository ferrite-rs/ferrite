use ferrite_session::*;

define_choice!{ CounterCommand;
  Increment: Z,
  GetCount: SendValue < u64, Z >,
}

type CounterSession =
  LinearToShared <
    ExternalChoice <
      CounterCommand
    >
  >;

fn make_counter_session
  ( count: u64 )
  -> SharedSession < CounterSession >
{
  accept_shared_session (
    offer_choice! {
      Increment =>
        detach_shared_session (
          make_counter_session ( count + 1 )
        )
      GetCount =>
        send_value ( count,
          detach_shared_session (
            make_counter_session ( count ) ) )
    }
  )
}

async fn use_counter
  ( counter: SharedChannel < CounterSession >,
    count: u64,
  ) ->
    u64
{
  for _ in 0..count {
    run_session (
      acquire_shared_session! ( counter, chan =>
        choose! ( chan, Increment,
          release_shared_session ( chan,
            terminate() ) ) )
    ).await;
  }

  run_session_with_result (
    acquire_shared_session! ( counter, chan =>
      choose! ( chan, GetCount,
        receive_value_from! ( chan, count =>
          release_shared_session ( chan,
            send_value ( count ,
              terminate() ) ) ) ) )
  ).await
}

#[tokio::main]
pub async fn main() {
  env_logger::init();

  let (counter, _) =
    run_shared_session ( make_counter_session ( 0 ) );

  use_counter ( counter, 100000 ).await;
}
