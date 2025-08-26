use dioxus::prelude::*;
use crate::workspace::main::Main;
use super::right_panel::RightPanel;
use super::left_panel::LeftPanel;

const WORKSPACE_STYLE: Asset = asset!("/assets/styling/workspace/workspace.css");

pub enum WorkspaceDataSource {

}


#[component]
pub fn Workspace() -> Element {
    rsx! {
        document::Stylesheet{href: WORKSPACE_STYLE}
        div {
            id: "out_container",
            div {
                id: "left",
                LeftPanel {}
            }
            div {
                id: "main",
                Main{}
            }
            div {
                id: "right",
                RightPanel{
                }
            }
        }
    }
}
