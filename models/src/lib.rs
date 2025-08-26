mod client_config;
pub mod workspace;
pub mod auth;
pub mod state_provider;
pub mod user;
pub mod app;

use dioxus::logger::tracing::info;
use utils::storage::local_storage::LocalStorageProvider;
use crate::client_config::ClientConfig;
pub use workspace::workspace_state::WorkspaceState;
use crate::app::app_state::AppState;
use crate::auth::auth_state::AuthState;
use crate::state_provider::StateProvider;

pub fn init_network() {
    block_insight_cross::api::client::init(ClientConfig::new());
}

pub fn init() {
    info!("现在初始化数据");
    // init_network();
    LocalStorageProvider::use_context_provider();
    AppState::use_context_provider();
    AuthState::start();
    // UserState::use_context_provider();
    WorkspaceState::start();
    // info!("base: {}", dioxus::config::get);

}
