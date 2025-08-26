use dioxus::logger::tracing::{error, info};
use dioxus::prelude::*;
use models::workspace::transaction_service::TransactionServiceModule;
use models::WorkspaceState;

const STYLE: Asset = asset!("/assets/styling/workspace/transaction_info_input_panel.css");

#[derive(Debug, Clone)]
enum TransactionQueryType {
    Query(String),
    QueryNearby {
        signature: String,
        backward: Option<u64>,
        forward: Option<u64>,
    },
}

#[component]
pub fn TransactionInfoInputPanel() -> Element {
    let workspace_state = use_context::<WorkspaceState>();
    let focus = workspace_state.transaction_service_state.transaction_focus;
    let mut signature = use_signal(|| String::new());
    let mut backward_slot = use_signal(|| 0);
    let mut forward_slot = use_signal(|| 0);
    let on_backward_slot_input = move |data: Event<FormData>| {
        let s: String = data.value();
        let slot = s.parse::<u32>().unwrap_or(0);
        backward_slot.set(slot);
    };
    let on_forward_slot_input = move |data: Event<FormData>| {
        let s: String = data.value();
        let slot = s.parse::<u32>().unwrap_or(0);
        forward_slot.set(slot);
    };
    let need_slot = match &*focus.read_unchecked() {
        TransactionServiceModule::Query => false,
        TransactionServiceModule::QueryNearby => true,
    };

    let on_click = move |_| match focus() {
        TransactionServiceModule::Query => {
            if let Err(e) = workspace_state.query_transaction(signature()) {
                error!("error: {e:?}");
            } else {
                info!("transaction successfully queried");
            }
        }
        TransactionServiceModule::QueryNearby => {
            // 获取slot
            if let Err(e) = workspace_state.query_transaction_near_by(
                signature(),
                Some(backward_slot()),
                Some(forward_slot()),
            ) {
                error!("{e:?}");
            }
        }
    };

    rsx! {
        document::Stylesheet{href: STYLE},
        div {
            id: "tx_info_input_panel_container",
            div{
                style: "font-size: 1.5rem",
                "{focus}"
            }
            div {
                id: "tx_input_items_container",
                div {
                    label {
                        for: "signature",
                        {"签名"}
                    }
                    input {
                        id: "signature",
                        oninput: move |data| signature.set(data.value()) ,
                        placeholder: "请输入交易签名",
                    }
                }
                if need_slot {
                    div {
                        label {
                            r#for: "forward_slot",
                            {"往回slot "}
                        }
                        input {
                            id: "backward_slot",
                            type: "number",
                            placeholder: "1",
                            oninput: on_backward_slot_input, }
                    }
                    div {
                        label {
                            r#for: "forward_slot",
                            {"往前slot "}
                        }
                        input {
                            id: "forward_slot",
                            type: "number",
                            placeholder: "0",
                            oninput: on_forward_slot_input,
                        }
                    }
                }

            }
            button {
                class: "button",
                onclick: on_click,
                "提交"
            }
        }
    }
}

// async fn on_click_submit(query: TransactionQueryType) {}
