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
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub url: String,
    pub method: HttpMethod,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
}

//pub struct Message<T> {
//    pub id: Uuid,
//    pub payload: T,
//}

//pub enum Command {
//    Run,
//    Check,
//}

#[cfg(test)]
mod tests {

    use super::*;
    use toml;

    #[test]
    fn should_deserialize_simulation() {
        let simulation: Simulation = toml::from_str(r#"
            user_count = 1
            fail_on_error = false
            [endpoint]
            method = 'GET'
            url = 'http://localhost:8080/test'
        "#).unwrap();

        assert_ne!(simulation.id, Uuid::nil());
        assert_eq!(simulation.user_count, 1);
        assert_eq!(simulation.endpoint.url, "http://localhost:8080/test");
    }

    #[test]
    fn should_deserialize_endpoint() {
        let endpoint: HttpEndpoint = toml::from_str(r#"
            url = 'http://test.com.au'
            method = 'POST'
        "#).unwrap();

        assert_ne!(endpoint.id, Uuid::nil());
        assert_eq!(endpoint.url, "http://test.com.au");
        assert_eq!(endpoint.method, HttpMethod::POST);
    }

}
