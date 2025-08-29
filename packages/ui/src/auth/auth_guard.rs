use dioxus::prelude::*;
use models::{
    auth::{auth_service::AuthService, user_role::UserRole},
    service::service_provider::ServiceProvider,
};

#[component]
pub fn AuthGuard(
    required_role: Option<UserRole>,
    target: Option<String>,
    children: Element,
) -> Element {
    let navigator = use_navigator();
    let auth = AuthService::use_service();
    let auth_clone = auth.clone();
    let role = required_role.clone();
    use_effect(move || {
        if !auth_clone.is_authenticated(&role) {
            if let Some(target) = &target {
                navigator.replace(target.as_ref());
            } else {
                navigator.replace("/login");
            }
        }
    });
    if auth.is_authenticated(&required_role) {
        children
    } else {
        rsx! {}
    }
}
