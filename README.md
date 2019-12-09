# Ferrite - Safe session types for Rust

## Table of Contents

  - [Introduction](#introduction)
  - [Hello World](#hello-world)
  - [Greetings](#greetings)
  - [Linearity and Affinity](#linearity-and-affinity)
  - [Ping Pong](#ping-pong)

## Introduction

Ferrite is a library that provides type level embedding of linear session
types in Rust. With Ferrite, users can specify session-types-based
communication protocols as Rust types, and implement processes that conform
to the protocols verified by the Rust type checker.

## Hello World

We will first learn about the basics of SessionRust with a simple hello world
example:

```rust
extern crate ferrite;

use async_std::task;
use ferrite::*;

pub fn main() {
  let p
    : Session < End >
    = terminate_async ( async || {
      printf!("hello world!");
      task::sleep(Duration::from_secs(1)).await;
      printf!("goodbye world!");
    });

  printf!("running session p");
  run_session(p);
  printf!("session p ended");
}
```

Here we construct a session of type `Session < End >` for `p`, which is a session consist of a
root process node that simply terminates. We use the `terminate_async ()` constructor
to construct our only process by passing it a Rust closure. The Rust closure
represents a _suspended computation_ that is later executed when we call
`run_session()` with `p`.

Running the above example would produce output as follow:

```
running session p
hello world!
goodbye world!
session p ended
```

with a one second gap between printing of "hello world!" and "goodbye world!".

`run_session()` accepts a `Session < End >` as an argument. Upon calling, it
spawns processes to be executed in parallel and blocks until all underlying
processes are terminated.

Our hello world example have only a single process. We will next take a look
how to construct multiple processes and link them together.

## Greetings

Before we continue, to simplify the user guide, we will assume that our code
is wrapped around a `main()` function with the import statements like follow:

```rust
extern crate session_rust;

use async_std::task;
use session_rust::*;

pub fn main() {
  ... // code snippets here
}
```

In this example, we will build a session with two processes, with the first one
being a process taking in a String value representing a person's name, and
greets the person on screen.

```rust
type HelloSession =
  ReceiveValue <
    String,
    End
  >
```

We define a type alias `HelloSession` that have the session type
`ReceiveValue < String, End >`. This represents a process that accepts an input
`String` value and then terminates.

The process type `ReceiveValue` is a _session type_ embedded as a Rust type. `ReceiveValue` takes
two type arguments, the first one being the Rust type to be received as input,
and the second being the _continuation session type_ after the process receives
the input value. In our case, the continuation is another session type `End`,
which indicates that the process terminates after receiving the input value.

Our process p1 implements `HelloSession` by printing the greeting on screen.

```rust
let p1
  : Session < HelloSession >
  = receive_value ( | name | {
    printf!("hello, %name!", name);

    terminate ()
  });
```

Unlike our previous example, `p1` here have the process type `ReceiveValue` instead of `End`. This means that at this point although we have
constructed `p1`, we can't just run it by passing it to `run_session()`.
SessionRust enforces that a session can only run after the whole process graph
is constructed. Since `p1` is waiting to receive value, there is no use running
`p1` right now because it will simply block and wait for input forever.

To make `p1` runnable, we have to construct another process `p2` that produce
the string value we need:

```rust
let p2
  : Session <
    ReceiveChannel <
      HelloSession,
      End
    >
  >
  = receive_channel ( | x | {
      send_value_to_async ( x, async || {
        // pretend to read input from some source
        task::sleep(Duration::from_secs(1)).await;

        return (
          "John".to_string(),
          terminate_async ( async || {
            // pretend to cleanup, e.g. close file handler
            task::sleep(Duration::from_secs(1)).await;
          })
        )
      });
```

Our `p2` is a session that _receives_ a channel of type `HelloSession`, consumes it and then terminate.

In practice, `p2` may receive the input of our user's name from some
input source, such as STDIN. For simplicity, we just use `task::sleep()` to
simulate the delay and return the name "john".

`p2` also performs some clean up operation before terminating. In
practice this may be actions such as closing file handler, but it may also be
some other tasks that take longer time. By putting the cleanup actions inside
the closure for `terminate_async ()`, we make sure that `p1` can receive the produced
String value without waiting for `p2` to actually finished terminating.

Now that we have both `p1` and `p2`, we need to find way to connect the two
processes into a single session. For our case we can use `apply_channel` to
forward the value produced from `p2` to `p1`.

```rust
let p3
  : Session < End >
  > = apply_channel (p2, p1);
```

`apply_channel` works if one process is of type `ReceiveChannel` expecting a
channel of the same session type as its second argument. It returns a new
`Session` with the continuation type after the channel is received, which
in our case is simply `End`.

```rust
run_session (p3);
```

## Linearity and Affinity

In the previous example, when we build our sessions, the variables `p1` to `p3`
are actually _affine_ and is enforced by Rust to be usable _at most once_.
When we call functions such as `apply_channel (p2, p1)`, we are moving the
ownership of `p1` and `p2` to `apply_channel ()`. This also means that `p1` and
`p2` can no longer be used anywhere else, or we would get type errors doing so.

At the end of our example program, we only retain ownership to `p3`, which is
then finally be consumed by `run_session` by running the process graph. However
with Rust's affine type system, it is also possible for us to drop `p4` and not
run it at all. This begs a question: does the linearity requirement for session
types still get enforced under an affine type system?

SessionRust solves this problem by enforcing that the entire process graph is
executed at most once, and if it is executed, all processes within it are
guaranteed to be executed exactly once.

In other words, it is ok to drop `p3`, because doing so would mean neither
`p1` nor `p2` are executed. SessionRust guarantees that all processes
are run either together or never. This means we will never have situation such
as `p1` is running but `p2` isn't, which would have otherwise resulted in
deadlock at runtime.

When passing in closures, SessionRust also takes in the closures as the type
`FnOnce`. This means we can move resource ownership into the closures, and
expect them to be called at most once by SessionRust.

## Ping Pong

Now that we learn the basics of running multiple processes and linking them
into one session, we will design a simple ping pong protocol for communicating
between two processes. In the ping pong protocol, a server would receive a
ping message from a client and respond with a pong message. We first define
the protocol session types as a type aliases:

```rust
struct Ping {
  message: String
}

struct Pong {
  message: String
}

type PingPongServer =
  ReceiveValue <
    Ping,
    SendValue <
      Pong,
      End
    >
  >;

type PingPongClient =
  ReceiveChannel <
    PingPongServer,
    End
  >;
```

We first define two dummy data structures `Ping` and `Pong` to represent the
messages being exchanged. In practice these structures may contain distinct
fields such as the timestamp and IP address, but for simplicity we'll leave
it with a simple `String` message field for the tutorial.

Next we define `PingPongServer` as the server-side session type.
The server receives `Ping` value, then sends a `Pong` value, then terminates.

`PingPongClient` is the client-side session type. It simply receives a `PingPongServer` channel to communicate with the server and then terminates.

Now that we have defined our protocol, we can implement our server and client
as follow:

```rust
let pingpong_server
  : Session < PingPongServer >
  = receive_value ( | ping_message | {
      printf!("server received ping message: %s", ping_message.message);

      send_value (
        Pong {
          message = "pong!"
        }
        , terminate()
      )
    });

let pingpong_client
  : Session < PingPongClient >
  = receive_channel ( | x | {
      send_value_to ( x,
        Ping {
          message = "ping!"
        } ,
        receive_value_from ( x,
          | pong_message | {
            printf!("client received pong message: %s", pong_message.message);
            terminate ()
          }) )
    });
```

In our example server and client, they construct the `Ping` and `Pong` structures
directly with a custom message, print them upon receiving, and then terminates
immediately. In practice, the implementation may be more complicated, for example
with the server side of the process acting as an agent to send the message to
an actual server over the network.

With our client and server defined, we can now link them using `apply_channel` and run the linked session:

```rust
let pingpong_session
  : Session < End >
  = apply_channel ( pingpong_client, pingpong_server );

run_session ( pingpong_session );
```

(to be continue..)