use crate::workspace::menu_panel::MenuPanel;
use crate::workspace::transaction_filter_panel::TransactionFilterPanel;
use crate::workspace::transaction_info_input_panel::TransactionInfoInputPanel;
use dioxus::prelude::*;
use models::WorkspaceState;
use models::workspace::transaction_service::TransactionServiceModule;

const RIGHT_PANEL: Asset = asset!("/assets/styling/workspace/right_panel.css");

#[component]
pub fn RightPanel() -> Element {
    let workspace_state = WorkspaceState::use_context();
    let focus = *workspace_state
        .transaction_service_state
        .transaction_focus
        .read_unchecked();
    let need_filters_panel = (focus == TransactionServiceModule::QueryNearby);
    rsx! {
        document::Stylesheet{href: RIGHT_PANEL}
        div {
            id: "right_panel_container",
            div{
                id: "menu_panel",
                MenuPanel {
                }
            }
            div {
                id: "right_panel_bottom_container",
                if need_filters_panel {
                    div {
                        TransactionFilterPanel {

                        }
                    }
                }
                div {
                    id: "input_panel",
                    TransactionInfoInputPanel {
                    }
                }
            }
        }
    }
}
