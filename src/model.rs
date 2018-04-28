use std;
use std::error;
use std::fmt;
use toml;
use uuid::Uuid;

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
pub struct HttpEndpoint {
    #[serde(skip)]
    pub id: Uuid,
    pub url: String,
    pub method: HttpMethod,
}

#[derive(Debug, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
}

pub struct Message<T> {
    pub id: Uuid,
    pub payload: T,
}

pub enum Command {
    Run,
    Check,
}

// todo have a look at quick_error or error_chain
#[derive(Debug)]
pub enum Error {
    IllegalConfiguration,
    ParseToml(toml::de::Error),
    ParseIO(std::io::Error)
}

pub type Result<T> = std::result::Result<T, Error>;

// todo it would be good not to couple this module to the toml::de error types
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::ParseToml(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::ParseIO(err)
    }
}

const ILLEGAL_CONFIG_ERROR_MSG: &str = "There is an error in the provided simulation config";

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IllegalConfiguration => ILLEGAL_CONFIG_ERROR_MSG.fmt(f),
            Error::ParseToml(ref e) => e.fmt(f),
            Error::ParseIO(ref e) => e.fmt(f)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IllegalConfiguration => ILLEGAL_CONFIG_ERROR_MSG,
            Error::ParseToml(ref e) => e.description(),
            Error::ParseIO(ref e) => e.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IllegalConfiguration => None,
            Error::ParseToml(ref e) => Some(e),
            Error::ParseIO(ref e) => Some(e)
        }
    }
}