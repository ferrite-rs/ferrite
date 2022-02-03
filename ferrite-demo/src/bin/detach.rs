use ferrite_session::{
  internal::protocol::Lock,
  prelude::*,
};

type Counter = LinearToShared<SendValue<u64, Release>>;
type Detached = ReceiveValue<u64, SharedToLinear<Counter>>;
type CounterLock = Lock<SendValue<u64, Release>>;

fn shared_provider_1(count: u64) -> SharedSession<Counter>
{
  accept_shared_session(async move {
    send_value(count, detach_shared_session(shared_provider_1(count + 1)))
  })
}

fn detached_provider_1(counter: SharedChannel<Counter>) -> Session<Detached>
{
  receive_value(move |count| {
    println!("[detached_provider_1] received count 1: {}", count);

    acquire_shared_session(counter, move |c| {
      receive_value_from(c, move |count| {
        println!("[detached_provider_1] received count 2: {}", count);
        forward(c)
      })
    })
  })
}

fn detached_provider_2() -> Session<ReceiveChannel<CounterLock, Detached>>
{
  receive_channel(move |_lock| {
    receive_value(move |count| {
      println!("[detached_provider_2] received count: {}", count);

      detach_shared_session(shared_provider_1(count + 1))
    })
  })
}

fn shared_provider_2(count: u64) -> SharedSession<Counter>
{
  accept_shared_session(async move {
    send_value(
      count,
      include_session(detached_provider_2(), move |c| {
        send_channel_to(c, Z, send_value_to(c, count + 1, forward(c)))
      }),
    )
  })
}

fn detached_client() -> Session<ReceiveChannel<Detached, End>>
{
  receive_channel(move |c| {
    send_value_to(c, 42, release_shared_session(c, terminate()))
  })
}

fn shared_client() -> Session<ReceiveValue<SharedChannel<Counter>, End>>
{
  receive_value(move |counter| {
    include_session(detached_provider_1(counter), move |c1| {
      include_session(detached_client(), move |c2| {
        send_channel_to(c2, c1, wait(c2, terminate()))
      })
    })
  })
}

async fn run_shared_client(counter: SharedChannel<Counter>)
{
  run_session(include_session(shared_client(), move |c| {
    send_value_to(c, counter, wait(c, terminate()))
  }))
  .await;
}

#[tokio::main]
pub async fn main()
{
  let counter1 = run_shared_session(shared_provider_1(100));
  let counter2 = run_shared_session(shared_provider_2(200));

  run_shared_client(counter1).await;
  run_shared_client(counter2).await;
}
