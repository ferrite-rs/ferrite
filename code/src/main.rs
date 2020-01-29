#![feature(async_closure)]
#[macro_use]

extern crate log;
// extern crate simple_logger;
extern crate env_logger;

pub mod demo;
pub mod macros;

pub mod base;
pub mod shared;
pub mod session;
pub mod process;
pub mod processes;

mod public;

pub use crate::demo::*;
pub use crate::public::*;

pub fn main() {
  // simple_logger::init().unwrap();
  env_logger::init();

  info!("[Main] Running main program");

  // run_session(hello_session());
  // run_session(pair_session());
  // run_session(restaurant_session());
  // run_session(concat_session());
  // run_session(queue_session());
  // run_session(counter_session());
  // run_session ( shared_counter_session() );

  // run_session (
  //   demo::channel::channel_session()
  // );

  // run_session (
  //   demo::nary_choice::nary_choice_demo()
  // );

  run_session (
    demo::queue2::queue_session()
  );

  info!("[Main] Main program terminating");
}
