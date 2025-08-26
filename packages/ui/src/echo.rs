use dioxus::logger::tracing::{error, info};
use dioxus::prelude::*;
use std::str::FromStr;
use block_insight_cross::api::transaction::client::fetch_transaction;
use block_insight_cross::api::transaction::FetchTransactionParam;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut signature: Signal<Option<String>> = use_signal(|| None);
    let fetch_transaction = use_resource(move || async move {
        if let Some(signature_str) = &signature() {
            let signature = match solana_signature::Signature::from_str(&signature_str) {
                Ok(s) => s,
                Err(e) => {
                    error!("签名错误: {e:?}, str: {}", signature_str);
                    return Err(e.into());
                }
            };
            info!("sig: {}", signature);
            let param = FetchTransactionParam {
                signature: signature_str.to_string(),
            };
            match fetch_transaction(param).await {
                Ok(r) => {
                    return Ok(Some(r));
                }
                Err(e) => return Err(anyhow::anyhow!(format!("{:?}", e))),
            }
        } else {
            return Ok(None);
        }
    });
    let on_input = move |event: Event<FormData>| async move {
        signature.set(Some(event.value()));
    };

    let tx = &*fetch_transaction.read_unchecked();
    let resp = if let Some(Ok(Some(tx))) = tx {
        serde_json::to_string(&tx).unwrap_or_else(|e| format!("{e:?}"))
    } else {
        "无回复".to_string()
    };
    // let tx = fetch_transaction().flatten().unwrap_or("Empty".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  on_input,
            }

            if let Some(Ok(Some(_tx))) = tx {
                p{
                    "Server echoed:"
                }
                i { "{resp}" }
            }

        }
    }
}
