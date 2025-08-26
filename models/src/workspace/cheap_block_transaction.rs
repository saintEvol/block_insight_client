use crate::workspace::transaction_service::ParsedEncodedConfirmedTransactionWithStatusMeta;
use dioxus::logger::tracing::error;
use serde::{Deserialize, Serialize};
use solana_transaction::versioned::{TransactionVersion, VersionedTransaction};
use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
    EncodedTransactionWithStatusMeta, UiTransactionStatusMeta,
};
use std::rc::Rc;
use block_insight_cross::api::transaction::BlockTransaction;
use block_insight_cross::parsed_instruction::ParsedInstructionList;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct VersionedTransactionWithStatusMeta {
    pub slot: u64,
    pub transaction: Option<VersionedTransaction>,
    pub meta: Option<UiTransactionStatusMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<TransactionVersion>,
    pub block_time: Option<i64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedVersionedTransactionWithStatusMeta {
    pub transaction: Rc<VersionedTransactionWithStatusMeta>,
    pub parsed_instructions: Rc<ParsedInstructionList>,
}

impl ParsedVersionedTransactionWithStatusMeta {
    pub fn new(transaction: EncodedConfirmedTransactionWithStatusMeta) -> Self {
        let EncodedConfirmedTransactionWithStatusMeta {
            slot,
            transaction,
            block_time,
        } = transaction;
        let EncodedTransactionWithStatusMeta {
            transaction,
            meta,
            version,
        } = transaction;
        let transaction = transaction.decode();
        let versioned = VersionedTransactionWithStatusMeta {
            slot,
            transaction,
            meta,
            version,
            block_time,
        };

        todo!("new parsed versioned");
    }
}

// #[derive(Debug, PartialEq, Clone)]
// pub struct ParsedEncodedTransactionWithStatusMeta {
//     pub transaction: EncodedTransactionWithStatusMeta,
//     pub parsed_instructions: ParsedInstructionList,
// }
//
// impl ParsedEncodedTransactionWithStatusMeta {
//     pub fn new(transaction: EncodedTransactionWithStatusMeta) -> Self {
//         let parsed_instructions = ParsedInstructionList::from(&transaction);
//         ParsedEncodedTransactionWithStatusMeta {
//             transaction,
//             parsed_instructions,
//         }
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// pub struct ParsedEncodedConfirmedTransactionWithStatusMeta {
//     pub transaction: Rc<EncodedConfirmedTransactionWithStatusMeta>,
//     pub parsed_instructions: Rc<ParsedInstructionList>,
// }

// impl ParsedEncodedConfirmedTransactionWithStatusMeta {
//     pub fn new(transaction: EncodedConfirmedTransactionWithStatusMeta) -> Self {
//         let parsed_instructions = Rc::new(ParsedInstructionList::from(&transaction.transaction));
//         ParsedEncodedConfirmedTransactionWithStatusMeta {
//             transaction: Rc::new(transaction),
//             parsed_instructions,
//         }
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub struct CheapBlockTransaction {
    pub slot: u64,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
    pub transactions: Rc<Vec<ParsedEncodedConfirmedTransactionWithStatusMeta>>,
}

impl CheapBlockTransaction {
    /// BlockTransaction转化为CheapBlockTransaction,并检查指定的签名是否在该区别中，如果在，则返回
    pub fn from_block_transaction(
        block_transaction: BlockTransaction,
        signature: &String,
    ) -> (
        Self,
        Option<ParsedEncodedConfirmedTransactionWithStatusMeta>,
    ) {
        let BlockTransaction {
            slot,
            block_time,
            block_height,
            transactions,
        } = block_transaction;
        let mut looking_tx = Option::None;
        let transactions = if let Some(transactions) = transactions {
            let tx = transactions
                .into_iter()
                .map(|tx| {
                    let v =
                        ParsedEncodedConfirmedTransactionWithStatusMeta::from_encoded_transaction_with_status_meta(tx, slot, block_time);
                    match &v.transaction.transaction.transaction {
                        EncodedTransaction::Json(tx) => {
                            if tx.signatures.contains(signature) {
                                looking_tx.replace(v.clone());
                            }
                        }
                        _ => {
                            error!("意外的文件格式，目前只支持Json编码的交易数据")
                        }
                    }
                    v
                })
                .collect();
            Rc::new(tx)

        } else {
            Rc::new(vec![])
        };
        let cheap = CheapBlockTransaction {
            slot,
            block_time,
            block_height,
            transactions,
        };
        (cheap, looking_tx)
    }
}

// impl TransactionPropsProvider for CheapBlockTransaction {}
