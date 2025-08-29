use block_insight_cross::protos::messages::client::client_message::Payload;
use utils::ws_cross::WsEvent;

use crate::{constants::WS_BASE_URL, service::service_provider::Service};

#[derive(Clone)]
pub struct NetworkService;

impl Service for NetworkService {
    fn instance() -> Self {
        NetworkService
    }
}

impl NetworkService {
    pub fn connect(&self) {
        let cb = |event: WsEvent| todo!("ws event");
        let cb = Box::new(cb);
        utils::ws_cross::WebSocket::connect(WS_BASE_URL.to_string(), cb);
    }

    pub fn send(&self, payload: Payload) {
        utils::ws_cross::WebSocket::send(payload);
    }
}
