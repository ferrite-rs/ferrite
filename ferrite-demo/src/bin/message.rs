use ferrite_session::*;
use ipc_channel::ipc;
use futures::future::join_all;

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
  let mut futures = vec![];

  for i in 0..count {
    let future = async_acquire_shared_session ( counter.clone(), move | chan | {
      choose! ( chan, Increment,
        release_shared_session ( chan,
          terminate() ) )
    });

    futures.push(future);

    if i % 1000 == 0 {
      join_all(futures.drain(0..)).await;
    }
  }

  join_all(futures).await;

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

  let counter = run_shared_session ( make_counter_session ( 0 ) );

  let (sender, receiver) = ipc::channel().unwrap();
  sender.send(counter).unwrap();
  let shared = receiver.recv().unwrap();
  // let shared = counter.clone();

  let count = use_counter ( shared, 10000 ).await;
  println!("count: {}", count);
}
