use dioxus::core_macro::component;
use dioxus::prelude::*;
use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
};
use std::rc::Rc;

const SIGNATURE_STYLE: Asset = asset!("/assets/styling/workspace/signature_component.css");

#[component]
pub fn SignatureComponent(data: Rc<EncodedConfirmedTransactionWithStatusMeta>) -> Element {
    let signatures = match &data.transaction.transaction {
        EncodedTransaction::Json(tx) => &tx.signatures,
        _ => {
            return rsx! {
                "错误的交易格式: 目前只处理Json编码的"
            }
        }
    };
    rsx! {
        document::Stylesheet{href: SIGNATURE_STYLE},
        div {
            id: "sig_container",

            label {
                "签名: "
            }

            div {
                for signature in signatures {
                    div {
                        "{signature}"
                    }
                }
            }
        }

    }
}
