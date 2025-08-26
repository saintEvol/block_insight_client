use dioxus::core_macro::component;
use dioxus::prelude::{*};

#[component]
pub fn BlockComponent(slot: u64, block_time: Option<i64>) -> Element {
    let block_time = block_time.unwrap_or(0);
    rsx!{
        div {
            style: "border:3px solid red; text-align: start",
            label{
                "区块: {slot}, 区块时间: {block_time}"
            }
        }
    }
}