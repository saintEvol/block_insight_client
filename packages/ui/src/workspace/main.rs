mod content;
mod transaction_details_page;
mod signature_component;
mod block_component;

use crate::workspace::main::content::Content;
use dioxus::prelude::*;
use models::WorkspaceState;

const MAIN_STYLE: Asset = asset!("/assets/styling/workspace/main.css");
#[component]
pub fn Main() -> Element {
    let workspace = use_context::<WorkspaceState>();
    // let status = workspace.transaction_service_state.transaction_service_status;
    match &*workspace.transaction_service_state.filtered_handling_data.read_unchecked() {
        Some(_r) => {
            rsx! {
                 document::Stylesheet{href: MAIN_STYLE},
                 div{
                    id: "container",
                    Content{}
                 }
            }
        }
        None => {
            rsx! {
                document::Stylesheet{href: MAIN_STYLE},
                div {
                    id: "empty_container",
                    div {
                        id: "empty_tips",
                        "未查询任何数据"
                    }
                }
            }
        }
    }
}
