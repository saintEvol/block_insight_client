use crate::workspace::block_transaction_list_state::BlockTransactionListState;
use dioxus::prelude::*;
use models::workspace::cheap_block_transaction::CheapBlockTransaction;
use models::workspace::transaction_service::ParsedEncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status_client_types::EncodedTransaction;
use std::rc::Rc;

const STYLE: Asset = asset!("/assets/styling/workspace/block_transaction_component.css");
#[component]
pub fn BlockTransactionList(data: Rc<Vec<CheapBlockTransaction>>) -> Element {
    let focus = use_signal(|| None);
    use_context_provider(|| BlockTransactionListState { focus_slot: focus });
    rsx! {
        for block in data.iter() {
            div {
                key: "{block.slot.to_string()}",
                BlockTransactionComponent {
                    data: block.clone(),
                }
            }
        }
    }
}

#[component]
pub fn BlockTransactionComponent(data: CheapBlockTransaction) -> Element {
    let mut state = use_context::<BlockTransactionListState>();
    let need_show_children = state.focus_slot.read_unchecked().unwrap_or(0) == data.slot;
    let block_time = data.block_time.unwrap_or(0);
    let block_height = data.block_height.unwrap_or(0);
    let my_slot = data.slot;
    let on_click = move |_| {
        if *state.focus_slot.peek_unchecked() == Some(my_slot) {
            state.focus_slot.set(None);
        } else {
            state.focus_slot.set(Some(my_slot));
        }
    };

    rsx! {
        document::Stylesheet{href: STYLE}
        div {
            onclick: on_click,
            div {
                id: "block_transaction_component_container",
                label {
                    "slot: {data.slot}"
                }
                label {
                    "区块时间: {block_time}"
                }
                label {
                    "区块高度: {block_height}"
                }
                label {
                    "交易数: {data.transactions.len()}"
                }
            }

            if need_show_children {
                div {
                    TransactionList{transactions: data.transactions.clone()}
                }
            }
        }
    }
}

#[component]
fn TransactionList(
    transactions: Rc<Vec<ParsedEncodedConfirmedTransactionWithStatusMeta>>,
) -> Element {
    rsx! {
        for tx in &*transactions {
            Transaction {transaction: tx.clone()}
        }
    }
}

#[component]
fn Transaction(transaction: ParsedEncodedConfirmedTransactionWithStatusMeta) -> Element {
    let real_transaction = &transaction.transaction.transaction;
    let sig = match &real_transaction.transaction {
        EncodedTransaction::Json(json) => &json.signatures[0],
        _ => return rsx! {"交易格式错误(只支持Json)"},
    };
    let on_click_details = |_| {};
    let nav = navigator();
    // let on_click_solscan = |e| {
    //     nav.push(format!("https://solscan.io/tx/{sig}"));
    // };
    rsx! {
        div {
            class: "sub_tx_container",
            onclick: |e|e.stop_propagation(),
            label {
                class: "ellipsis-label",
                "签名: {sig}"
            }
            div {
                class: "sub_tx_buttons_container",
                button {
                    onclick: on_click_details,
                    "详情"
                }
                Link{
                    class: "solscan_link",
                    new_tab: true,
                    to: "https://solscan.io/tx/{sig}",
                    "在solcan上查看"
                }
                // button {
                //     onclick: |e|{
                //
                //     },
                //     "前往[solscan]"
                // }
            }
        }
    }
}
