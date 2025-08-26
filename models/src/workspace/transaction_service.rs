use crate::WorkspaceState;
use crate::workspace::cheap_block_transaction::CheapBlockTransaction;
use anyhow::anyhow;
use block_insight_cross::api::api_error::ApiError;
use block_insight_cross::api::transaction::client::fetch_transactions_near_by;
use block_insight_cross::api::transaction::{FetchTransactionParam, FetchTransactionsNearByParam};
use block_insight_cross::parsed_instruction::{ParsedInstruction, ParsedInstructionList};
use block_insight_cross::transaction::transaction_filter::{
    TransactionFilter, TransactionFilterContext, TransactionMeta, TransactionPropsProvider,
};
use block_insight_cross::utils::TransactionAccounts;
use dioxus::hooks::UnboundedReceiver;
use dioxus::logger::tracing::{error, info};
use dioxus::prelude::*;
use dioxus::prelude::{Coroutine, Readable, Signal, Writable, use_coroutine};
use futures_util::StreamExt;
use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
    EncodedTransactionWithStatusMeta, UiMessage,
};
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum TransactionServiceModule {
    Query,
    QueryNearby,
}

impl Display for TransactionServiceModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            TransactionServiceModule::Query => {
                write!(f, "查询交易")
            }
            TransactionServiceModule::QueryNearby => {
                write!(f, "查询邻近交易")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedEncodedConfirmedTransactionWithStatusMeta {
    pub transaction: Rc<EncodedConfirmedTransactionWithStatusMeta>,
    pub parsed_instructions: Rc<ParsedInstructionList>,
}

impl ParsedEncodedConfirmedTransactionWithStatusMeta {
    pub fn new(transaction: EncodedConfirmedTransactionWithStatusMeta) -> Self {
        let parsed_instructions = ParsedInstructionList::from(&transaction.transaction);
        ParsedEncodedConfirmedTransactionWithStatusMeta {
            transaction: Rc::new(transaction),
            parsed_instructions: Rc::new(parsed_instructions),
        }
    }

    pub fn from_encoded_transaction_with_status_meta(
        transaction: EncodedTransactionWithStatusMeta,
        slot: u64,
        block_time: Option<i64>,
    ) -> Self {
        let parsed_instructions = Rc::new(ParsedInstructionList::from(&transaction));
        let transaction = EncodedConfirmedTransactionWithStatusMeta {
            slot,
            transaction,
            block_time,
        };
        let transaction = Rc::new(transaction);
        ParsedEncodedConfirmedTransactionWithStatusMeta {
            transaction,
            parsed_instructions,
        }
    }
}

impl TransactionPropsProvider for ParsedEncodedConfirmedTransactionWithStatusMeta {
    fn get_accounts(&self) -> TransactionAccounts<'_, String> {
        let (w, r) = &self
            .transaction
            .transaction
            .meta
            .as_ref()
            .map(|m| {
                m.loaded_addresses
                    .as_ref()
                    .map(|l| (&l.writable, &l.readonly))
            })
            .flatten()
            .map(|(w, r)| (Some(w), Some(r)))
            .unwrap_or((None, None));
        let account_keys = match &self.transaction.transaction.transaction {
            EncodedTransaction::Json(tx) => match &tx.message {
                UiMessage::Parsed(_) => {
                    error!("当前只支持Raw格式的Message, 实际为Parsed");
                    None
                }
                UiMessage::Raw(raw) => Some(raw.account_keys.as_slice()),
            },
            _ => {
                error!("目前只支持Json格式的交易");
                None
            }
        };
        TransactionAccounts::from_accounts(
            account_keys,
            w.map(|w| w.as_slice()),
            r.map(|r| r.as_slice()),
        )
    }

    fn get_signatures(&self) -> Option<&[String]> {
        match &self.transaction.transaction.transaction {
            EncodedTransaction::Json(tx) => Some(tx.signatures.as_ref()),
            _ => {
                error!("当前只支持Json格式的交易数据");
                None
            }
        }
    }

    fn get_parsed_instructions(&self) -> Option<&[ParsedInstruction]> {
        Some(self.parsed_instructions.as_slice())
    }

    fn get_meta(&self) -> Option<TransactionMeta> {
        match &self.transaction.transaction.meta {
            None => None,
            Some(m) => {
                let meta = TransactionMeta {
                    err: m.err.as_ref(),
                    status: &m.status,
                    fee: m.fee,
                    pre_balances: m.pre_balances.as_slice(),
                    post_balances: m.post_balances.as_slice(),
                    log_messages: m.log_messages.as_ref().map(|m| m.as_slice()),
                    compute_units_consumed: m.compute_units_consumed.as_ref().map(|c| *c),
                };

                Some(meta)
            }
        }
    }
}

// #[derive(Clone)]
#[derive(Debug, Clone)]
pub enum HandlingData {
    Query(ParsedEncodedConfirmedTransactionWithStatusMeta),
    QueryNearby(Rc<Vec<CheapBlockTransaction>>),
}

// pub enum WorkspaceData {
//     TransactionInspection(ParsedEncodedConfirmedTransactionWithStatusMeta),
//     TransactionsAnalyzing {
//         current: Option<Rc<ParsedEncodedTransactionWithStatusMeta>>,
//         all: Rc<Vec<CheapBlockTransaction>>,
//     },
// }

/// 要检查的数据的当前状态（激活与非激活状态，比如，数据可能不符合过滤条件，但在过滤之前被选中了，过滤之后，如果没有更换成新的符合条件的检视条件的数据，则数据被切换为InActive状态
pub enum InspectingDataStatus {
    Active(InspectingData),
    InActive(InspectingData),
}

impl InspectingDataStatus {
    pub fn map(self, f: impl FnOnce(InspectingData) -> InspectingData) -> Self {
        match self {
            InspectingDataStatus::Active(data) => InspectingDataStatus::Active(f(data)),
            InspectingDataStatus::InActive(data) => InspectingDataStatus::InActive(f(data)),
        }
    }

    // pub fn take(&mut self) -> Self {
    //     mem::take(&mut self)
    // }

    pub fn data(self) -> InspectingData {
        match self {
            InspectingDataStatus::Active(d) => d,
            InspectingDataStatus::InActive(d) => d,
        }
    }

    pub fn set_active_status(mut self, is_active: bool) -> Self {
        // 状态一样，直接返回
        if is_active == self.is_active() {
            return self;
        }

        let data = self.data();
        match is_active {
            true => InspectingDataStatus::Active(data),
            false => InspectingDataStatus::InActive(data),
        }
    }

    pub fn is_active(&self) -> bool {
        match &self {
            InspectingDataStatus::Active(_) => true,
            InspectingDataStatus::InActive(_) => false,
        }
    }

    pub fn data_ref(&self) -> &InspectingData {
        match &self {
            InspectingDataStatus::Active(d) => d,
            InspectingDataStatus::InActive(d) => d,
        }
    }
}

pub enum InspectingData {
    SingleTransaction(ParsedEncodedConfirmedTransactionWithStatusMeta),
}

#[derive(Clone, Debug)]
pub enum TransactionServiceStatus {
    Idle,
    Processing(TransactionServiceModule),
    Finish(TransactionServiceModule),
}

#[derive(Clone, Copy, PartialEq)]
pub struct TransactionServiceState {
    /// 当前工作空间关注的模块
    pub transaction_focus: Signal<TransactionServiceModule>,
    /// 状态
    pub transaction_service_status: Signal<TransactionServiceStatus>,
    /// 当前正在处理的数据
    pub handling_data: Signal<Option<HandlingData>>,
    /// 真正的正在处理的数据，即原原始数据上进行过滤筛选后得到数据集,即用户可见数据集
    pub real_handling_data: Signal<Option<HandlingData>>,
    /// 正在进行检视的数据,用户选择的正在进行详细检视的数据
    pub inspecting_data: Signal<Option<InspectingDataStatus>>,
    /// 当前错误
    pub transaction_service_error: Signal<Option<ApiError>>,
    /// 当前的过滤上下文
    pub transaction_filter_context: Signal<TransactionFilterContext>,
    /// 过滤器集合
    pub transaction_filter:
        Signal<HashMap<TypeId, Box<dyn TransactionFilter<ContextType = TransactionFilterContext>>>>,
    // pub filters:
}

impl TransactionServiceState {
    pub fn insert_filter(
        &mut self,
        filter: impl TransactionFilter<ContextType = TransactionFilterContext> + 'static,
    ) {
        let id = filter.id();
        let filter =
            Box::new(filter) as Box<dyn TransactionFilter<ContextType = TransactionFilterContext>>;
        self.transaction_filter.write().insert(id, filter);
    }

    pub fn remove_filter(
        &mut self,
        filter: impl TransactionFilter<ContextType = TransactionFilterContext> + 'static,
    ) {
        let id = filter.id();
        self.transaction_filter.write().remove(&id);
    }

    pub fn set_and_apply_filters(
        &mut self,
        filters: HashMap<
            TypeId,
            Box<dyn TransactionFilter<ContextType = TransactionFilterContext>>,
        >,
    ) {
        self.transaction_filter.set(filters);
        self.apply_filters();
    }

    pub(super) fn new() -> Self {
        let handling_data = use_signal(|| None);
        let real_handling_data = use_signal(|| None);
        let transaction_service_error = use_signal(|| None);
        let transaction_filter = use_signal(|| HashMap::new());
        let inspecting_data = use_signal(|| None);

        let mut state = TransactionServiceState {
            transaction_focus: use_signal(|| TransactionServiceModule::Query),
            transaction_service_status: use_signal(|| TransactionServiceStatus::Idle),
            handling_data,
            real_handling_data,
            inspecting_data,
            transaction_service_error,
            transaction_filter_context: use_signal(|| TransactionFilterContext::default()),
            transaction_filter,
        };
        state
    }

    async fn execute_query_near_by(
        &mut self,
        signature: String,
        backward: Option<u32>,
        forward: Option<u32>,
    ) {
        self.transaction_service_status
            .set(TransactionServiceStatus::Processing(
                TransactionServiceModule::QueryNearby,
            ));
        let ret = fetch_transactions_near_by(FetchTransactionsNearByParam {
            signature: signature.clone(),
            backward,
            forward,
        })
        .await;
        match ret {
            Ok(resp) => {
                let resp = resp.expect("交易块数据必须不为空");
                let mut current = None;
                let raw_resp = resp
                    .into_iter()
                    .map(|ret| {
                        let (cheap, temp) =
                            CheapBlockTransaction::from_block_transaction(ret, &signature);
                        if temp.is_some() {
                            current = temp;
                        }
                        cheap
                    })
                    .collect::<Vec<_>>();
                // 应用过滤器
                let mut context = TransactionFilterContext::default();
                let data = Self::do_apply_filters(
                    &raw_resp,
                    &mut context,
                    &*self.transaction_filter.read_unchecked(),
                );
                info!("after filter , len: {}", data.len());
                self.handling_data
                    .set(Some(HandlingData::QueryNearby(Rc::new(raw_resp))));
                self.real_handling_data
                    .set(Some(HandlingData::QueryNearby(Rc::new(data))));
                self.inspecting_data.set(
                    current.map(|c| {
                        InspectingDataStatus::Active(InspectingData::SingleTransaction(c))
                    }),
                );
                self.transaction_filter_context.set(context);
                self.transaction_service_error.set(None);
            }
            Err(e) => {
                self.transaction_service_error.set(Some(e));
                self.handling_data.set(None);
                self.real_handling_data.set(None);
            }
        }
        self.transaction_service_status
            .set(TransactionServiceStatus::Finish(
                TransactionServiceModule::QueryNearby,
            ))
    }

    async fn execute_query(&mut self, signature: String) {
        // 修改状态
        self.transaction_service_status
            .set(TransactionServiceStatus::Processing(
                TransactionServiceModule::Query,
            ));

        // 请求数据
        let ret = block_insight_cross::api::transaction::client::fetch_transaction(
            FetchTransactionParam { signature },
        )
        .await;
        match ret {
            Ok(resp) => {
                let resp = resp.expect("数据不能为空");
                self.transaction_service_error.set(None);
                let data = ParsedEncodedConfirmedTransactionWithStatusMeta::new(resp);
                self.handling_data
                    .set(Some(HandlingData::Query(data.clone())));
                self.real_handling_data
                    .set(Some(HandlingData::Query(data.clone())));
            }
            Err(e) => {
                error!("error when req: {e:?}");
                self.transaction_service_error.set(Some(e));
                self.handling_data.set(None);
                self.real_handling_data.set(None);
            }
        }

        // 修改状态
        self.transaction_service_status
            .set(TransactionServiceStatus::Finish(
                TransactionServiceModule::Query,
            ));
    }

    fn apply_filters(&mut self) {
        match &*self.handling_data.peek_unchecked() {
            None => {}
            Some(resp) => match resp {
                HandlingData::Query(_) => {}
                HandlingData::QueryNearby(data) => {
                    let mut ctx = TransactionFilterContext::default();
                    let filters = &*self.transaction_filter.read_unchecked();
                    let real_handling_data = Self::do_apply_filters(data, &mut ctx, filters);
                    let inspecting_data = self.inspecting_data.write_unchecked().take();
                    if let Some(mut inspecting_data) = inspecting_data {
                        match inspecting_data.data_ref() {
                            InspectingData::SingleTransaction(inspecting_tx) => {
                                let is_active =
                                    Self::filter_transaction(inspecting_tx, &mut ctx, filters);
                                let inspecting_data = inspecting_data.set_active_status(is_active);
                                self.inspecting_data.set(Some(inspecting_data))
                            }
                        }
                    }
                    self.real_handling_data
                        .set(Some(HandlingData::QueryNearby(Rc::new(real_handling_data))));
                }
            },
        }
    }

    fn do_apply_filters(
        data: &[CheapBlockTransaction],
        context: &mut TransactionFilterContext,
        filters: &HashMap<
            TypeId,
            Box<dyn TransactionFilter<ContextType = TransactionFilterContext>>,
        >,
    ) -> Vec<CheapBlockTransaction> {
        if filters.is_empty() {
            return data.to_vec();
        }

        let mut ret = Vec::new();
        for d in data {
            let r = d
                .transactions
                .iter()
                .filter_map(|t| {
                    let keep = Self::filter_transaction(t, context, filters);
                    if keep { Some(t.clone()) } else { None }
                })
                .collect::<Vec<_>>();
            let new = CheapBlockTransaction {
                slot: d.slot,
                block_time: d.block_time,
                block_height: d.block_height,
                transactions: Rc::new(r),
            };
            ret.push(new);
        }
        ret
    }

    fn filter_transaction(
        transaction: &ParsedEncodedConfirmedTransactionWithStatusMeta,
        context: &mut TransactionFilterContext,
        filters: &HashMap<
            TypeId,
            Box<dyn TransactionFilter<ContextType = TransactionFilterContext>>,
        >,
    ) -> bool {
        filters
            .iter()
            .all(|(_, filter)| filter.filter(&*transaction, context))
    }
}

#[derive(Clone)]
pub enum TransactionCmd {
    Query(String),
    QueryNearBy {
        signature: String,
        backward: Option<u32>,
        forward: Option<u32>,
    },
}

impl TransactionServiceState {
    pub fn focus(&mut self, module: TransactionServiceModule) {
        self.transaction_focus.set(module);
    }
}

impl WorkspaceState {
    pub fn start_transaction_service(
        mut state: TransactionServiceState,
    ) -> Coroutine<TransactionCmd> {
        let transaction_service =
            use_coroutine(move |mut r: UnboundedReceiver<TransactionCmd>| async move {
                while let Some(cmd) = r.next().await {
                    match cmd {
                        TransactionCmd::Query(signature) => {
                            state.execute_query(signature).await;
                        }
                        TransactionCmd::QueryNearBy {
                            signature,
                            backward,
                            forward,
                        } => {
                            state
                                .execute_query_near_by(signature, backward, forward)
                                .await;
                        }
                    }
                }
            });

        transaction_service
    }

    /// 请求交易数据
    pub fn query_transaction(&self, signature: String) -> anyhow::Result<()> {
        // 先检查签名是否正确
        match solana_signature::Signature::from_str(&signature) {
            Ok(_) => {
                let cmd = TransactionCmd::Query(signature);
                self.transaction_service.send(cmd);
                Ok(())
            }
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn query_transaction_near_by(
        &self,
        signature: String,
        backward_slot: Option<u32>,
        forward_slot: Option<u32>,
    ) -> anyhow::Result<()> {
        // 先检查签名是否正确
        match solana_signature::Signature::from_str(&signature) {
            Ok(_) => {
                let cmd = TransactionCmd::QueryNearBy {
                    signature,
                    backward: backward_slot,
                    forward: forward_slot,
                };
                self.transaction_service.send(cmd);
                Ok(())
            }
            Err(e) => Err(anyhow!(e)),
        }
    }
}
