use dioxus::core_macro::rsx;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use std::any::TypeId;
use std::collections::HashMap;
use block_insight_cross::transaction::transaction_filter::{TransactionFilter, TransactionFilterContext};
use block_insight_cross::transaction::transaction_filter::account_filter::AccountFilter;
use block_insight_cross::transaction::transaction_filter::circle_swap_filter::CircleSwapFilter;
use block_insight_cross::transaction::transaction_filter::signature_filter::SignatureFilter;
use block_insight_cross::transaction::transaction_filter::status_filter::TransactionStatusFilter;
use models::workspace::transaction_service::TransactionServiceState;
use models::WorkspaceState;

const FILTER_PANEL_STYLE: Asset = asset!("/assets/styling/workspace/transaction_filter_panel.css");

#[derive(Clone, Copy, PartialEq)]
struct FilterContext {
    need_filter_account: Signal<bool>,
    filtering_accounts: Signal<String>,

    need_filter_signature: Signal<bool>,
    filtering_signatures: Signal<String>,

    need_filter_status: Signal<bool>,
    // true: success, false: false
    filtering_status: Signal<bool>,

    need_filter_circle_swap: Signal<bool>,

    transaction_service_state: TransactionServiceState,
}

impl FilterContext {
    fn new(transaction_service_state: TransactionServiceState) -> Self {
        let need_filter_account = Signal::new(false);
        let filtering_accounts = Signal::new("".into());

        let need_filter_signature = Signal::new(false);

        let need_filter_circle_swap = Signal::new(false);
        let filtering_signatures = Signal::new("".into());

        let need_filter_status = Signal::new(false);
        let filtering_status = Signal::new(false);

        FilterContext {
            need_filter_account,
            need_filter_signature,
            filtering_accounts,
            need_filter_circle_swap,
            transaction_service_state,
            filtering_signatures,
            need_filter_status,
            filtering_status,
        }
    }

    fn apply_filters(&mut self) {
        // 收集过滤器
        let mut filters: HashMap<
            TypeId,
            Box<dyn TransactionFilter<ContextType = TransactionFilterContext>>,
        > = HashMap::new();

        self.collect_filter(&mut filters, self.make_status_filter());
        self.collect_filter(&mut filters, self.make_signature_filter());
        self.collect_filter(&mut filters, self.make_account_filter());
        self.collect_filter(&mut filters, self.make_circle_swap_filter());

        self.transaction_service_state
            .set_and_apply_filters(filters);
    }

    fn make_circle_swap_filter(&self) -> anyhow::Result<Option<CircleSwapFilter>> {
        if *self.need_filter_circle_swap.peek_unchecked() {
            Ok(Some(CircleSwapFilter))
        } else {
            Ok(None)
        }
    }

    fn make_account_filter(&self) -> anyhow::Result<Option<AccountFilter>> {
        if !*(self.need_filter_account.peek_unchecked()) {
            return Ok(None);
        }
        let content = &*self.filtering_accounts.peek_unchecked();

        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }

        let accounts = trimmed
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(Some(AccountFilter::Include(accounts)))
    }

    fn make_status_filter(&self) -> anyhow::Result<Option<TransactionStatusFilter>> {
        if *self.need_filter_status.peek_unchecked() {
            Ok(Some(if *self.filtering_status.peek_unchecked() {
                TransactionStatusFilter::success()
            } else {
                TransactionStatusFilter::fail()
            }))
        } else {
            Ok(None)
        }
    }

    fn make_signature_filter(&self) -> anyhow::Result<Option<SignatureFilter>> {
        if !*(self.need_filter_signature.peek_unchecked()) {
            return Ok(None);
        }
        let content = &*self.filtering_signatures.peek_unchecked();

        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }

        let accounts = trimmed
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(Some(SignatureFilter::Include(accounts)))
    }

    fn collect_filter(
        &self,
        container: &mut HashMap<
            TypeId,
            Box<dyn TransactionFilter<ContextType = TransactionFilterContext>>,
        >,
        filter: anyhow::Result<
            Option<impl TransactionFilter<ContextType = TransactionFilterContext>>,
        >,
    ) {
        if let Ok(filter_context) = filter {
            if let Some(filter_context) = filter_context {
                let filter = Box::new(filter_context)
                    as Box<dyn TransactionFilter<ContextType = TransactionFilterContext>>;
                container.insert(filter.id(), filter);
            }
        }
    }
}

#[component]
pub fn TransactionFilterPanel() -> Element {
    let workspace_state = use_context::<WorkspaceState>();
    let transaction_service_state = workspace_state.transaction_service_state;
    let mut filter_context = use_context_provider(|| FilterContext::new(transaction_service_state));
    let on_click_filter = move |_e| {
        filter_context.apply_filters();
    };

    rsx! {
        document::Stylesheet{href: FILTER_PANEL_STYLE},
        div {
            id: "transaction_filter_panel_container",
            // title
            div {
                id: "filter_panel_title",
                style: "font-size: 1.5rem; ",
                "交易筛选器"
            }

            TransactionFilterItems {}

            button {
                id: "tx_filter_button",
                onclick: on_click_filter,
                "筛选"
            }
        }
    }
}

#[component]
fn TransactionFilterItems() -> Element {
    let FilterContext {
        need_filter_account,
        filtering_accounts,
        need_filter_signature,
        filtering_signatures,
        need_filter_status,
        mut filtering_status,
        need_filter_circle_swap,

        transaction_service_state,
    } = use_context::<FilterContext>();
    let check_cb = |mut sig: Signal<bool>| {
        move |event: Event<FormData>| {
            sig.set(event.checked());
        }
    };
    let input_cb = |mut sig: Signal<String>| {
        move |event: Event<FormData>| {
            sig.set(event.value());
        }
    };
    let on_status_filter_submit = move |e: Event<FormData>| {
        info!("on cliick status filter radio: {}", e.data.value());
        if e.data.value() == "true" {
            filtering_status.set(true);
        } else {
            filtering_status.set(false);
        }
    };

    rsx! {
        div {
            id: "filter_items_container",
            // 签名过滤
            div {
                class: "two_line_filter_item_container",
               div {
                   class: "filter_item_title_container",
                   input{
                       r#type: "checkbox",
                       onchange: check_cb(need_filter_signature),
                   }
                   label{"包含签名:"}
               }
               input{
                   class: "filter_item_value",
                   oninput: input_cb(filtering_signatures),
               }

            }

            // 帐号过滤器
            div {
                class: "two_line_filter_item_container",
               div {
                   class: "filter_item_title_container",
                   input{
                       r#type: "checkbox",
                       onchange: check_cb(need_filter_account),
                   }
                   label{"包含帐号:"}
               }
               input{
                   class: "filter_item_value",
                   oninput: input_cb(filtering_accounts),
               }

            }
            div {
               div {
                   class: "tx_status_filter_title",
                   input{
                       r#type: "checkbox",
                       onchange: check_cb(need_filter_status),
                   }
                   label{"状态过滤(成功/失败): "}
               }
                form {
                    id: "tx_status_er",
                    label {
                        input {
                            onchange: on_status_filter_submit,
                            name: "status",
                            r#type: "radio",
                            value: "true"
                        }
                        "成功"
                    }
                    label {
                        input {
                            onchange: on_status_filter_submit,
                            name: "status",
                            r#type: "radio",
                            value: "false"
                        }
                        "失败"
                    }
                }
            }
            div {
                // 三角套利过滤器
               div {
                    id: "circle_filter_check_box",
                   class: "filter_item_title_container",
                   input{
                       r#type: "checkbox",
                       onchange: check_cb(need_filter_circle_swap),
                   }
                   label{
                        r#for: "circle_filter_check_box",
                        "三角套利过滤"
                    }
               }


            }
        }
    }
}
