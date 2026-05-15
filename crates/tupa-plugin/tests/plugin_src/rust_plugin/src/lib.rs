// Minimal Tupã plugin — doubles a numeric value and classifies sentiment.
// Build: cargo build --crate-type cdylib --release
// Load from Rust via tupa_plugin::PluginManager

use serde_json::Value;
use tupa_plugin::PluginRegisterContext;

#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const i8 {
    static NAME: &str = "rust_demo_plugin";
    NAME.as_ptr() as *const i8
}

#[no_mangle]
pub extern "C" fn _tupa_plugin_register(ctx: *mut PluginRegisterContext) {
    unsafe {
        // register double_score
        let name1 = b"double_score\0".as_ptr() as *const i8;
        let func1: extern "C" fn(Value) -> Value = double_score;
        let func_ptr1 = func1 as *const ();
        (*ctx).register_step.unwrap()(name1, func_ptr1 as *const u8);
        (*ctx).functions.push("double_score".to_string());

        // register sentiment
        let name2 = b"sentiment\0".as_ptr() as *const i8;
        let func2: extern "C" fn(Value) -> Value = sentiment;
        let func_ptr2 = func2 as *const ();
        (*ctx).register_step.unwrap()(name2, func_ptr2 as *const u8);
        (*ctx).functions.push("sentiment".to_string());
    }
}

/// Multiplies the integer value inside the JSON input by 2.
/// Expects: `{"value": <number>}`
/// Returns: `{"value": <number * 2>}`
#[no_mangle]
pub extern "C" fn double_score(input: Value) -> Value {
    match input.get("value").and_then(|v| v.as_i64()) {
        Some(n) => json!({ "value": n * 2 }),
        None => json!({ "error": "expected {\"value\": <number>}" }),
    }
}

/// Classifies text sentiment as positive / negative / neutral.
/// Expects: `{"text": "<string>"}`
/// Returns: `{"sentiment": "positive" | "negative" | "neutral"}`
#[no_mangle]
pub extern "C" fn sentiment(input: Value) -> Value {
    let text = input
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_lowercase();
    let label = if text.contains("good")
        || text.contains("great")
        || text.contains("excellent")
    {
        "positive"
    } else if text.contains("bad") || text.contains("terrible") || text.contains("awful") {
        "negative"
    } else {
        "neutral"
    };
    json!({ "sentiment": label })
}
