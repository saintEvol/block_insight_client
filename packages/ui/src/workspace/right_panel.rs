use crate::workspace::menu_panel::MenuPanel;
use crate::workspace::transaction_info_input_panel::TransactionInfoInputPanel;
use dioxus::prelude::*;
use crate::workspace::transaction_filter_panel::TransactionFilterPanel;

const RIGHT_PANEL: Asset = asset!("/assets/styling/workspace/right_panel.css");

#[component]
pub fn RightPanel() -> Element {
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
                div {
                    TransactionFilterPanel {

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
