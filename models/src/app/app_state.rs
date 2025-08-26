use dioxus::prelude::*;
use crate::state_provider::StateProvider;

#[derive(Copy, Clone)]
pub struct AppState {
    pub doing: Signal<Option<String>>,
}

impl AppState {
    pub fn engaged(&mut self, content: String) {
        self.doing.set(Some(content));
    }

    pub fn unengaged(&mut self) {
        self.doing.set(None);
    }

    pub fn use_context_provider() -> Self {
        use_context_provider(||{
            let state = AppState {
                doing: Signal::new(None),
            };
            state
        })
    }
}

// impl StateProvider for AppState {
//     fn use_context() -> Self {
//         use_context()
//     }
// }