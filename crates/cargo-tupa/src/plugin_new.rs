use anyhow::Result;

/// Generate a new plugin scaffold: `cargo tupa plugin new [filename]`
pub fn run(filename: Option<String>) -> Result<()> {
    let out_name = filename.unwrap_or_else(|| "my_plugin.rs".to_string());
    let out_path = std::path::Path::new(&out_name);

    if out_path.exists() {
        anyhow::bail!("File '{}' already exists", out_name);
    }

    let template = r#"// Tupã Plugin — dynamic step function for tupa-engine
// Build: cargo build --release --crate-type cdylib
// The plugin exports two C symbols required by tupa-plugin:
//   - _tupa_plugin_name() -> *const c_char
//   - _tupa_plugin_register(ctx: &mut PluginRegisterContext)
//
// Dependencies in Cargo.toml:
// [dependencies]
// tupa-plugin = { version = "0.9", features = ["ffi"] }

use std::ffi::{CString, CStr};
use tupa_plugin::PluginRegisterContext;

/// Example step: double the input value
#[no_mangle]
pub extern "C" fn my_step(input_json: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let input_str = unsafe { CStr::from_ptr(input_json) }
        .to_string_lossy()
        .into_owned();

    let input: serde_json::Value = serde_json::from_str(&input_str).unwrap_or_default();
    let output = serde_json::json!({
        "doubled": input.as_f64().unwrap_or(0.0) * 2.0
    });

    let out_str = serde_json::to_string(&output).unwrap();
    CString::new(out_str).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn _tupa_plugin_register(ctx: &mut PluginRegisterContext) {
    ctx.register_step("my_step", my_step);
}

#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const std::os::raw::c_char {
    "my_plugin\0".as_ptr() as *const _
}
"#;

    std::fs::write(out_path, template)?;
    println!("✓ Created plugin scaffold: {}", out_name);
    println!("  Build with: cargo build --release --crate-type cdylib");
    Ok(())
}
