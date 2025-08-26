use dioxus::prelude::*;
const MODAL_STYLE: Asset = asset!("/assets/styling/modal.css");
#[component]
pub fn Modal(content: String) -> Element {
    rsx!{
        document::Stylesheet{href: MODAL_STYLE},
        div {
            id: "root_modal_container",
            div {
                {content}
            }

        }
    }
}