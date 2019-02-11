extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate uuid;

use std::env;

use loader::ConfigLoader;
use loader::DefaultConfigLoader;
use model::Simulation;

mod logging;
mod model;
mod loader;

fn main() {
    logging::configure().expect("Unable to configure logging");
    let args: Vec<String> = env::args().collect();
    info!("initializing system from args: {:?}", args);
    // todo get this from args
    let path = &args[1];
    let config = load_config(path);
    info!("starting simulation");
    // todo iterate the config and perform operations

    info!("shutting down system resources");
}

fn load_config(path: &str) -> Simulation {
    let loader = DefaultConfigLoader::new();
    match loader.load_simulation(path) {
        Ok(config) => return config,
        Err(error) => panic!("There was a problem initializing the simulation: {:?}", error)
    };
}

#[cfg(test)]
mod tests {

    use std::process::Command;

    #[test]
    fn should_run() {
        Command::new("hydrogen").arg("config/sample_simulation.toml").output();
    }

    #[test]
    #[should_panic]
    fn should_fail_invalid_config() {
        Command::new("hydrogen").arg("fake.toml")
            .output()
            .expect_err("There was a problem initializing the simulation");
    }
}