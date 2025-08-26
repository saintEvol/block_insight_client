use crate::workspace::block_transaction_list::BlockTransactionList;
use dioxus::prelude::*;
use models::WorkspaceState;
use models::workspace::transaction_service::HandlingData;

#[component]
pub fn LeftPanel() -> Element {
    let content = route_content();
    rsx! {
        {content}
    }
}

fn route_content() -> Element {
    let workspace = use_context::<WorkspaceState>();
    let transaction_service = &workspace.transaction_service_state;
    match &*transaction_service.real_handling_data.read_unchecked() {
        None => {
            rsx! {
                "没有数据"
            }
        }
        Some(resp) => match resp {
            HandlingData::Query(_) => {
                rsx! {
                    div {
                        style: "height: 100%; display: flex; flex-direction: column;justify-content: center;",
                        div {
                            "无额外数据"
                        }
                    }
                }
            }
            HandlingData::QueryNearby(all) => {
                rsx! {
                    BlockTransactionList {data: all.clone()}
                }
            }
        },
    }
}
