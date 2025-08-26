use crate::workspace::main::transaction_details_page::TransactionDetailsPage;
use dioxus::hooks::{use_context};
use dioxus::prelude::*;
use models::workspace::transaction_service::{InspectingData, InspectingDataStatus};
use models::WorkspaceState;

#[component]
pub fn Content() -> Element {
    let workspace = use_context::<WorkspaceState>();
    let transaction_service_state = &workspace.transaction_service_state;
    let ele = route_content(
        &*transaction_service_state
            .inspecting_data
            .read_unchecked(),
    );
    rsx! {
        {ele}
    }
}

fn route_content(inspecting_data: &Option<InspectingDataStatus>) -> Element {
    match inspecting_data {
        None => {
            rsx! {
                "没有任何数据"
            }
        }
        Some(r) => match &r.data_ref() {
            InspectingData::SingleTransaction(data) => {
                rsx! {
                    TransactionDetailsPage {data: data.clone()}
                }
            }
        },
    }
}
