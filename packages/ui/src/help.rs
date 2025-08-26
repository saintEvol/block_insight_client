use dioxus::prelude::*;

#[component]
pub fn Help(routes: Vec<String>) -> Element {
    rsx!{
        h1 {
            "帮助"
        }
    }
}