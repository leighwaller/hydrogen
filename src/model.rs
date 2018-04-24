use uuid::Uuid;
use toml;
use toml::de::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Simulation {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub user_count: u16,
    pub pause_millis: Option<u16>,
    pub fail_on_error: bool,
    pub endpoint: HttpEndpoint,
}

#[derive(Debug, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
}

#[derive(Debug, Deserialize)]
pub struct HttpEndpoint {
    #[serde(skip)]
    pub id: Uuid,
    pub url: String,
    pub method: HttpMethod,
}

impl HttpEndpoint {
    fn new() -> HttpEndpoint {
        HttpEndpoint {
            id: Uuid::new_v4(),
            url: String::from(""), // todo update this
            method: HttpMethod::GET,
        }
    }
}

pub struct Message<T> {
    pub id: Uuid,
    pub payload: T,
}

pub enum Command {
    Run,
    Check,
}

pub trait ConfigLoader {
    fn new() -> Self;
    // todo check if the common practice is to return custom wrapped errors
    fn load_simulation(&self, filename: &str) -> Result<Simulation, Error>;
}

pub struct DefaultConfigLoader;

impl ConfigLoader for DefaultConfigLoader {
    fn new() -> DefaultConfigLoader {
        DefaultConfigLoader {}
    }

    fn load_simulation(&self, filename: &str) -> Result<Simulation, Error> {
        info!("loading simulation config: {}", filename);
        let mut file = File::open(filename).unwrap();
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();
        info!("simulation read in {}", buffer);
        //todo fix this to actually handle errors properly not unwrap
        return toml::from_str(&buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deserialize_simulation() {
        let loader = DefaultConfigLoader::new();

        let result = loader.load_simulation("config/simulation.toml");

        assert!(result.is_ok());

        let simulation = result.unwrap();

        assert_ne!(simulation.id, Uuid::nil());
        assert_eq!(simulation.endpoint.url, "http://localhost:8080/test");
    }
}