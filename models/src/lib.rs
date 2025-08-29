pub mod app;
pub mod auth;
mod client_config;
pub mod constants;
pub mod network;
pub mod service;
pub mod transaction;
pub mod user;
pub mod workspace;

use crate::app::app_state::GlobalService;
use crate::auth::auth_service::AuthService;
use crate::client_config::ClientConfig;
use crate::network::network_service::NetworkService;
use crate::service::service_provider::ServiceProvider;
use dioxus::logger::tracing::info;
use utils::storage::local_storage::LocalStorageProvider;
pub use workspace::workspace_state::WorkspaceState;

pub fn init_network() {
    block_insight_cross::api::client::init(ClientConfig::new());
}

pub fn init_services() {
    info!("现在初始化数据");
    // init_network();
    LocalStorageProvider::init();
    GlobalService::init();
    AuthService::init();
    // UserState::use_context_provider();
    WorkspaceState::start();
    NetworkService::init();
    // info!("base: {}", dioxus::config::get);
}
