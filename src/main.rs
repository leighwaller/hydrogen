#[macro_use]
extern crate log;

mod config;

fn main() {
    config::logging::configure()
        .expect("unable to configure logging");

    // todo implement a simple server to execute the load
    info!("starting server");
}
