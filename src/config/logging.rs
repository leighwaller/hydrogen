extern crate env_logger;

use self::env_logger::init;

pub fn configure() -> Result<(), ()> {
    env_logger::init();

    info!("configured logging to std out");
    Ok(())
}