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
