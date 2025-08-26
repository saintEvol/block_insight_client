use dioxus::prelude::*;
use crate::auth::login_register::{LoginPanelType, LoginRegister};

#[component]
pub fn Login() -> Element {
    rsx!{
        LoginRegister {
            panel_type: LoginPanelType::Login,
        }
    }
}