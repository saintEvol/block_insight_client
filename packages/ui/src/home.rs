use dioxus::prelude::*;
use models::{auth::auth_service::AuthService, service::service_provider::ServiceProvider};

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    let auth = AuthService::use_service();
    let authenticated = auth.is_authenticated(&None);
    let login_btn_label = if authenticated { "登出" } else { "登录" };
    let on_click_logout = |e| {};
    let on_click_login = |e| {};
    let on_click_login_or_logout = if authenticated {
        on_click_logout
    } else {
        on_click_login
    };
    rsx! {
        document::Stylesheet {href: HOME_CSS}
        div {
            class: "home_container",
            div {
                class: "home_content_container",
                div {
                    "欢迎您！"
                }
                div {
                    class: "home_content_button_container",
                    button{
                        class: "home_menu_item",
                        "工作区"
                    }

                    button {
                        class: "home_menu_item",
                        onclick: on_click_login_or_logout,
                        {login_btn_label}
                    }
                }

            }
        }
        // AuthGuard {
        //     required_role: None,
        //     target: None,
        //     Workspace {}
        // }
    }
}

#[component]
fn Welcome() -> Element {
    let auth = AuthService::use_service();
    let authenticated = auth.is_authenticated(&None);
    rsx! {
        div {
            if authenticated {
                div {

                }
            } else {
                div {  }
            }
        }
    }
}
