use reqwest::RequestBuilder;

pub struct ClientConfig {

}

impl ClientConfig {
    pub fn new() -> Self {
        ClientConfig {}
    }
}

impl block_insight_cross::api::client::ClientConfig for ClientConfig {
    fn get_base_url(&self) -> String {
        "http://127.0.0.1:8090/api".to_string()
    }

    fn before_request(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder
    }
}