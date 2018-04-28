use model::Simulation;
use std;
use std::fs::File;
use std::io::Read;
use toml;

pub type Result<T> = std::result::Result<T, ConfigurationError>;

pub trait ConfigLoader {
    fn new() -> Self;
    fn load_simulation(&self, filename: &str) -> Result<Simulation>;
}

pub struct DefaultConfigLoader;

impl ConfigLoader for DefaultConfigLoader {

    fn new() -> DefaultConfigLoader {
        DefaultConfigLoader {}
    }

    fn load_simulation(&self, filename: &str) -> Result<Simulation> {
        info!("loading simulation config: {}", filename);
        let mut file = File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        info!("simulation read in {}", buffer);
        return Ok(toml::from_str(&buffer)?);
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum ConfigurationError {
        InvalidFile(err: std::io::Error) {
            display("Invalid file path provided for configuration: {}", err)
            from()
        }
        InvalidFormat(err: toml::de::Error) {
            display("Invalid content in configuration file: {}", err)
            from()
        }
        Other(descr: &'static str) {
            description(descr)
            display("Error {}", descr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn should_deserialize_simulation() {
        let loader = DefaultConfigLoader::new();
        let path = "config/sample_simulation.toml";

        let result = loader.load_simulation(path);

        assert!(result.is_ok());

        let simulation = result.unwrap();

        assert_ne!(simulation.id, Uuid::nil());
        assert_eq!(simulation.user_count, 1);
        assert_eq!(simulation.endpoint.url, "http://localhost:8080/test");
    }

    #[test]
    fn should_handle_invalid_config_path() {
        let loader = DefaultConfigLoader::new();
        let result = loader.load_simulation("fake.tml");

        assert!(result.is_err());
    }

    #[test]
    fn should_handle_invalid_content() {
        let loader = DefaultConfigLoader::new();
        let result = loader.load_simulation(".gitignore");

        assert!(result.is_err());
        let err = result.err().unwrap();
    }

}
