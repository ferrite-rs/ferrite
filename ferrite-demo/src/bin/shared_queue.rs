use ferrite_session::prelude::*;

type Queue = LinearToShared<ExternalChoice<QueueOps>>;

define_choice! { QueueOps;
  Enqueue: ReceiveValue<String, Release>,
  Dequeue: InternalChoice<DequeueOps>
}

define_choice! { DequeueOps;
  Some: SendValue<String, Release>,
  None: Release,
}

fn empty_queue() -> SharedSession<Queue>
{
  accept_shared_session(async move {
    offer_choice! {
      Enqueue =>
        receive_value(move |val| {
          detach_shared_session(
            head_queue(val, run_shared_session(empty_queue())))
        }),
      Dequeue =>
        offer_case!(None,
          detach_shared_session(empty_queue()))
    }
  })
}

fn head_queue(
  val1: String,
  tail: SharedChannel<Queue>,
) -> SharedSession<Queue>
{
  accept_shared_session(async move {
    offer_choice! {
      Enqueue =>
        receive_value(move |val2| {
          acquire_shared_session(tail.clone(), move |c| {
            choose!(c, Enqueue,
              send_value_to(c, val2,
                release_shared_session(c,
                  detach_shared_session(
                    head_queue(val1, tail)))))
          })
        }),
      Dequeue =>
        offer_case!(Some,
          send_value(val1,
            shared_forward(tail)))
    }
  })
}

async fn enqueue(
  queue: SharedChannel<Queue>,
  val: String,
)
{
  run_session(acquire_shared_session(queue, move |c| {
    choose!(
      c,
      Enqueue,
      send_value_to(c, val, release_shared_session(c, terminate()))
    )
  }))
  .await;
}

async fn dequeue_and_print(queue: SharedChannel<Queue>)
{
  run_session(acquire_shared_session(queue, move |c| {
    choose!(
      c,
      Dequeue,
      case! { c;
        Some =>
          receive_value_from(c, move |val| {
            println!("Gotten dequeue value: {}", val);

            release_shared_session(c,
              terminate())
          }),
        None => step(async move {
          println!("Dequeue returns None");

          release_shared_session(c,
            terminate())
        })
      }
    )
  }))
  .await;
}

#[tokio::main]
pub async fn main()
{
  env_logger::init();

  let queue = run_shared_session(empty_queue());

  enqueue(queue.clone(), "Hello".to_string()).await;
  enqueue(queue.clone(), "World".to_string()).await;
  enqueue(queue.clone(), "Foo".to_string()).await;
  enqueue(queue.clone(), "Bar".to_string()).await;
  enqueue(queue.clone(), "Baz".to_string()).await;

  dequeue_and_print(queue.clone()).await;
  dequeue_and_print(queue.clone()).await;
  dequeue_and_print(queue.clone()).await;
  dequeue_and_print(queue.clone()).await;
  dequeue_and_print(queue.clone()).await;
  dequeue_and_print(queue.clone()).await;
  dequeue_and_print(queue.clone()).await;
}
