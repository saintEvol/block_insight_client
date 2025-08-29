use crate::auth::login_register::{LoginPanelType, LoginRegister};
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        LoginRegister {
            panel_type: LoginPanelType::Login,
        }
    }
}
