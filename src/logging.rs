extern crate env_logger;

pub fn configure() -> Result<(), ()> {
    env_logger::init();

    info!("configured logging to std out");
    Ok(())
}