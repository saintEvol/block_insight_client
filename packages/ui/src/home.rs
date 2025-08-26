use crate::Workspace;
use crate::auth::auth_guard::AuthGuard;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Stylesheet {href: HOME_CSS}
        AuthGuard {
            required_role: None,
            target: None,
            Workspace {}
        }
    }
}
