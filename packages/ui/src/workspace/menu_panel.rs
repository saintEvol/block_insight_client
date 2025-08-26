use dioxus::core_macro::component;
use dioxus::prelude::*;
use models::workspace::transaction_service::TransactionServiceModule;
use models::WorkspaceState;

const MENU_PANEL_STYLE: Asset = asset!("/assets/styling/workspace/menu_panel.css");

#[derive(Props, PartialEq, Clone)]
pub struct MenuPanelProps {}

#[component]
pub fn MenuPanel(props: MenuPanelProps) -> Element {
    let workspace = use_context::<WorkspaceState>();
    let mut transaction_service_state = workspace.transaction_service_state;
    rsx! {
        document::Stylesheet{href: MENU_PANEL_STYLE},
        div {
            id: "menu_panel_container",
            button {
                onclick: move |_self| {transaction_service_state.focus(TransactionServiceModule::Query);},
                "查询交易"
            }

            button {
                onclick: move |_| {
                    transaction_service_state.focus(TransactionServiceModule::QueryNearby);
                },
                "查询相邻交易"
            }
        }
    }
}
