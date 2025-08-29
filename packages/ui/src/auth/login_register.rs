use dioxus::core::Element;
use dioxus::document::Stylesheet;
use dioxus::prelude::*;
use models::app::app_state::GlobalService;
use models::auth::auth_service::AuthService;
use models::service::service_provider::ServiceProvider;

const LOGIN_STYLE: Asset = asset!("/assets/styling/auth/login_register.css");

#[derive(Clone, PartialEq)]
pub enum LoginPanelType {
    Login,
    Register,
}

#[component]
pub fn LoginRegister(panel_type: LoginPanelType) -> Element {
    let nav = use_navigator();
    let user_name_input = use_signal(|| "".to_string());
    let password_input = use_signal(|| "".to_string());
    let confirming_password_input = use_signal(|| "".to_string());
    let app = GlobalService::use_service();
    let update_input = |mut c: Signal<String>| move |e: Event<FormData>| c.set(e.value());

    let is_login = match panel_type {
        LoginPanelType::Login => true,
        LoginPanelType::Register => false,
    };
    let content = if is_login { "登录" } else { "注册" };
    let other_content = if is_login { "注册" } else { "登录" };
    let auth = AuthService::use_service();
    let on_click = move |e| {
        if !is_login {
            auth.register(
                user_name_input.peek_unchecked().clone(),
                password_input.peek_unchecked().clone(),
            );
        }
    };
    let on_click_other = move |e| {
        if is_login {
            nav.replace("/register");
        } else {
            nav.replace("/login");
        }
    };
    rsx! {
        Stylesheet{href: LOGIN_STYLE},
        div {
            id: "login_page_container",
            div {
                {content}
            }

            div {
                id: "login_form_container",
                div{
                    class: "login_item_container",
                    label {
                        r#for: "user_name",
                        "用户名"
                    }
                    input {
                        id: "user_name",
                        onchange: update_input(user_name_input),
                    }
                }
                div{
                    class: "login_item_container",
                    label {
                        r#for: "password",
                        "密码"
                    }
                    input {
                        r#type: "password",
                        id: "password",
                        onchange: update_input(password_input),
                    }

                }
                if !is_login {
                    div{
                        class: "login_item_container",
                        label {
                            r#for: "password",
                            "确认密码"
                        }
                        input {
                            r#type: "password",
                            id: "password",
                            onchange: update_input(confirming_password_input),
                        }

                    }
                }
            }

            div {
                id: "login_panel_button_container",
                button {
                    onclick: on_click,
                    {content}
                }
                span{
                    onclick: on_click_other,
                    {other_content}
                }
            }

        }
    }
}
