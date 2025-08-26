use dioxus::prelude::*;
use crate::auth::login_register::{LoginPanelType, LoginRegister};

#[component]
pub fn Register() -> Element {
    rsx!{
        LoginRegister {
            panel_type: LoginPanelType::Register,
        }
    }
}