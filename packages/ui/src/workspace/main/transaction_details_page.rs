use crate::workspace::main::block_component::BlockComponent;
use crate::workspace::main::signature_component::SignatureComponent;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, UiTransactionStatusMeta,
};
use block_insight_cross::parsed_instruction::{ParsedInstruction, ParsedInstructionData};
use models::workspace::transaction_service::ParsedEncodedConfirmedTransactionWithStatusMeta;

const STYLE: Asset = asset!("/assets/styling/workspace/transaction_details_page.css");
#[component]
pub fn TransactionDetailsPage(data: ParsedEncodedConfirmedTransactionWithStatusMeta) -> Element {
    let ParsedEncodedConfirmedTransactionWithStatusMeta {
        transaction,
        parsed_instructions,
    } = data;
    let EncodedConfirmedTransactionWithStatusMeta {
        slot, block_time, ..
    } = &*transaction;
    let result = result(transaction.transaction.meta.as_ref());
    rsx! {
        document::Stylesheet{href: STYLE}
        div {
            id: "tx_details_container",
            SignatureComponent{data: transaction.clone()}
            BlockComponent {slot: *slot, block_time: *block_time}
            div {
                style:"text-align: start",
                label{"结果: "}
                label {
                    "{result}"
                }
            }
            div {
                for ins in &parsed_instructions {
                    {instruction_view(ins)}
                }
            }
            // {content}
        }
    }
}

const INSTRUCTION_VIEW_STYLE: Asset = asset!("/assets/styling/workspace/instruction_view.css");
fn instruction_view(instruction: &ParsedInstruction) -> Element {
    let program_id = instruction.program_id_index;
    let program_name = match &instruction.instruction_data {
        ParsedInstructionData::System(_) => "系统".to_string(),
        ParsedInstructionData::SplToken(_) => "SPL TOKEN".to_string(),
        ParsedInstructionData::SplToken2022(_) => "SPL TOKEN 2022".to_string(),
        ParsedInstructionData::Error(e) => {
            format!("错误: {e}")
        }
        ParsedInstructionData::Unknown => "未知指令".to_string(),
    };
    let inner = if let Some(inner) = instruction.inner_instructions.as_ref() {
        rsx!{
            for inner in inner {
                div{
                    {instruction_view(inner)}
                }
            }
        }
    } else {
        rsx!{}
    };
    rsx! {
        document::Stylesheet{href: INSTRUCTION_VIEW_STYLE}
        div {
            id: "instruction-view-container",
            div {
                label{
                    "{program_id}"
                }
                label {
                    "{program_name}"
                }
            }
            div{
                id: "inner_instruction_container",
                {inner}
            }
        }
    }
}

fn result(meta: Option<&UiTransactionStatusMeta>) -> String {
    meta.map(|m| {
        if let Some(e) = m.err.as_ref() {
            format!("错误: {}", e)
        } else {
            "成功".to_string()
        }
    })
    .unwrap_or("未知".to_string())
}
