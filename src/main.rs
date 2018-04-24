#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate uuid;
extern crate toml;

mod logging;
mod model;

// todo implement a simple client to execute the load
// todo it should be message based and highly concurrent
// todo set up some commands and a cli for the execution of the tests
// todo future could include the option for a distributed mode as well as the default standalone

fn main() {
    logging::configure().expect("unable to configure logging");
    info!("initializing system");

    info!("shutting down system resources");
}
