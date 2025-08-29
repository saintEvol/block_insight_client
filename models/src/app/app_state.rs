use dioxus::prelude::*;

use crate::service::service_provider::Service;

#[derive(Copy, Clone)]
pub struct GlobalService {
    pub doing: Signal<Option<String>>,
}

impl Service for GlobalService {
    fn instance() -> Self {
        let doing = use_signal(|| None);
        GlobalService { doing }
    }
}

impl GlobalService {
    pub fn engaged(&mut self, content: String) {
        self.doing.set(Some(content));
    }

    pub fn unengaged(&mut self) {
        self.doing.set(None);
    }
}

// impl StateProvider for AppState {
//     fn use_context() -> Self {
//         use_context()
//     }
// }
