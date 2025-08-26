use dioxus::prelude::*;
use models::auth::auth_state::AuthState;
use models::auth::user_role::UserRole;
use models::state_provider::StateProvider;

#[component]
pub fn AuthGuard(required_role: Option<UserRole>, target: Option<String>, children: Element) -> Element {
    let navigator = use_navigator();
    let auth = AuthState::use_context();
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
        rsx!{}
    }
}