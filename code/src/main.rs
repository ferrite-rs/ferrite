#![feature(async_closure)]

#[macro_use]

extern crate log;
// extern crate simple_logger;
extern crate env_logger;

mod demo;
mod base;
mod session;
mod process;
mod processes;
mod fix;
mod shared;

pub mod macros;

pub use crate::demo::*;
pub use crate::base::*;
pub use crate::session::*;
pub use crate::process::*;
pub use crate::processes::*;
pub use crate::fix::*;
pub use crate::shared::*;

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
  run_session ( shared_counter_session() );

  info!("[Main] Main program terminating");
}
