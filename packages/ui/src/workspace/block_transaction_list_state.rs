use dioxus::prelude::Signal;

#[derive(Clone, Default)]
pub struct BlockTransactionListState {
    pub focus_slot: Signal<Option<u64>>,
}