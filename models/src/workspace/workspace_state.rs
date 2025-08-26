use crate::state_provider::StateProvider;
use crate::workspace::transaction_service::{TransactionCmd, TransactionServiceState};
use dioxus::hooks::use_context;
use dioxus::prelude::{Coroutine, use_context_provider};

#[derive(Clone, Copy)]
pub struct WorkspaceState {
    pub transaction_service_state: TransactionServiceState,
    pub transaction_service: Coroutine<TransactionCmd>,
}

// impl Default for WorkspaceState {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl StateProvider for WorkspaceState {
//     fn use_context_provider() -> Self {
//         todo!()
//     }
//
//     fn use_context() -> Self {
//         todo!()
//     }
// }

impl WorkspaceState {
    pub fn start() -> WorkspaceState {
        // 交易服务
        let transaction_service_state = TransactionServiceState::new();
        let transaction_service = Self::start_transaction_service(transaction_service_state);
        use_context_provider(|| {
            let state = Self::new(transaction_service_state, transaction_service);
            state
        })
    }

    fn new(
        transaction_service_state: TransactionServiceState,
        transaction_service: Coroutine<TransactionCmd>,
    ) -> Self {
        WorkspaceState {
            transaction_service_state,
            transaction_service,
        }
    }
}
