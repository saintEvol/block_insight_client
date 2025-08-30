use crate::{
    network::network_service::NetworkService,
    service::service_provider::{Service, ServiceProvider},
};
use block_insight_cross::protos::messages::{
    client::client_message::Payload,
    user::{FetchUserReq, FetchUserResp, UserInfo},
};
use dioxus::{hooks::use_signal, signals::Signal};
use utils::{storage::local_storage::LocalStorage, ws_cross::WebSocketError};

#[derive(serde::Serialize, serde::Deserialize)]
struct UserStorage(UserInfo);

impl LocalStorage for UserStorage {
    fn key() -> &'static str {
        "user_info"
    }
}

impl UserService {
    pub fn fetch_user_info(&self) -> Result<(), WebSocketError> {
        let network = NetworkService::use_service();
        network.send(Payload::FetchUserReq(FetchUserReq {}))
    }
}

impl UserService {
    pub fn on_fetch_user_resp(&self, _resp: FetchUserResp) {
        todo!("on fetch user resp")
    }
}

#[derive(Default, Clone)]
pub struct UserService {
    pub user_info: Signal<Option<UserInfo>>,
}

impl Service for UserService {
    fn instance() -> Self {
        let user_info = use_signal(|| {
            let user_info = UserStorage::load().map(|info| info.0);
            user_info
        });
        UserService { user_info }
    }
}
