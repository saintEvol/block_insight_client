use crate::auth::auth_service_command::AuthServiceCommand;
use crate::auth::user_role::UserRole;
use block_insight_cross::api::api_error::ApiError;
use block_insight_cross::api::auth;
use block_insight_cross::api::auth::RegisterParams;
use dioxus::logger::tracing::{error, info};
use dioxus::prelude::*;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use utils::storage::{local_storage::LocalStorageProvider, kv_storage::KvStorage};
use utils::storage::local_storage::LocalStorage;
use utils::time::now_timestamp_ms;
use crate::app::app_state::AppState;
use crate::state_provider::StateProvider;

#[derive(Clone, Copy)]
pub struct AuthState {
    inner: Signal<Option<AuthInfo>>,
    service: Coroutine<AuthServiceCommand>,
}

const AUTH_INFO_KEY: &str = "auth_info";

impl AuthState {
    pub fn start() -> AuthState {
        let mut inner = use_signal(|| {
            // 加载本地存储的auth info
            let auth_info = AuthInfo::load();
            info!("加载旧的auth info: {:?}", auth_info);
            auth_info

        });
        let mut app_state = AppState::use_context();
        let auth_service = use_coroutine(
            move |mut receiver: UnboundedReceiver<AuthServiceCommand>| async move {
                while let Some(cmd) = receiver.next().await {
                    match cmd {
                        AuthServiceCommand::Register { email, password } => {
                            app_state.engaged("正在注册".to_string());
                            match execute_register(email, password).await {
                                Ok(auth) => {
                                    auth.as_ref().map(|auth|auth.save());
                                    inner.set(auth);
                                }
                                Err(e) => {
                                    error!("注册出错: {e:?}");
                                }
                            }
                            app_state.unengaged();
                        }
                    }
                }
                info!("auth服务退出");
            },
        );
        use_context_provider(|| AuthState::new(inner, auth_service))
    }

    pub fn register(&self, email: String, password: String) {
        self.service
            .send(AuthServiceCommand::Register { email, password });
    }

    pub fn is_authenticated(&self, role: &Option<UserRole>) -> bool {
        if let Some(auth_info) = &*self.inner.read() {
            auth_info.is_authenticated(role)
        } else {
            false
        }
    }

    pub fn inner(&self) -> Signal<Option<AuthInfo>> {
        self.inner
    }

    pub(super) fn new(
        inner: Signal<Option<AuthInfo>>,
        service: Coroutine<AuthServiceCommand>,
    ) -> Self {
        AuthState { inner, service }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub token: String,
    pub until_ms: i64,
}

impl LocalStorage for AuthInfo {
    fn key() -> &'static str {
        "auth_info"
    }
}

impl AuthInfo {
    pub(super) fn is_authenticated(&self, _role: &Option<UserRole>) -> bool {
        info!("now get time by rust utils");
        let now_ms = now_timestamp_ms();
        info!("success get time by rust utils");
        self.until_ms > now_ms as i64
    }

}

pub(super) async fn execute_register(
    email: String,
    password: String,
) -> Result<Option<AuthInfo>, ApiError> {
    let params = RegisterParams { email, password };
    let ret = auth::client::register(&params).await?;
    Ok(ret.map(
        |auth::AuthInfo {
             token,
             expires_at_ms,
         }| AuthInfo {
            token,
            until_ms: expires_at_ms,
        },
    ))
}
