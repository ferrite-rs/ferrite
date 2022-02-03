use ferrite_session::prelude::*;
use futures::future::join_all;
use ipc_channel::ipc;

define_choice! { CounterCommand;
  Increment: Release,
  GetCount: SendValue < u64, Release >,
}

type CounterSession = LinearToShared<ExternalChoice<CounterCommand>>;

fn make_counter_session(count: u64) -> SharedSession<CounterSession>
{
  accept_shared_session(async move {
    offer_choice! {
      Increment => {
        println!("provider incrementing count {}", count);

          detach_shared_session (
            make_counter_session ( count + 1 )
          )
      }
      GetCount => {
        println!("provider sending back count {}", count);
        send_value ( count,
            detach_shared_session (
              make_counter_session ( count ) ) )
      }
    }
  })
}

async fn use_counter(
  counter: SharedChannel<CounterSession>,
  count: u64,
) -> u64
{
  let mut futures = vec![];

  for i in 0..count {
    let future = async_acquire_shared_session(counter.clone(), move |chan| {
      choose!(
        chan,
        Increment,
        step(async move {
          println!("client incremented counter");
          release_shared_session(chan, terminate())
        })
      )
    });

    futures.push(future);

    if i % 1000 == 0 {
      join_all(futures.drain(0..)).await;
    }
  }

  join_all(futures).await;

  run_session_with_result(acquire_shared_session(counter, |chan| {
    choose!(
      chan,
      GetCount,
      receive_value_from(chan, move |count| release_shared_session(
        chan,
        send_value(count, terminate())
      ))
    )
  }))
  .await
}

#[tokio::main]

pub async fn main()
{
  env_logger::init();

  let counter = run_shared_session(make_counter_session(0));

  let (sender, receiver) = ipc::channel().unwrap();

  sender.send(counter).unwrap();

  let shared = receiver.recv().unwrap();

  // let shared = counter.clone();

  let count = use_counter(shared, 10000).await;

  println!("count: {}", count);
}
