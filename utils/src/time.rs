#[cfg(target_arch = "wasm32")]
pub fn now_timestamp_ms() -> i64 {
    // 在 wasm 环境下用 js_sys::Date
    (js_sys::Date::now()) as i64
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now_timestamp_ms() -> i64 {
    // 在 native 环境下用 SystemTime
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
