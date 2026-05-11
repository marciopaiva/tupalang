// TupaLang Plugin Template
// Build as: cargo build --crate-type=cdylib

use serde_json::Value;
use tupa_plugin::PluginRegisterContext;

#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const i8 {
    static NAME: &str = "my_plugin";
    NAME.as_ptr() as *const i8
}

#[no_mangle]
pub extern "C" fn _tupa_plugin_register(ctx: *mut PluginRegisterContext) {
    unsafe {
        let name = b"my_step\0".as_ptr() as *const i8;
        // Cast function pointer to raw bytes
        let func: extern "C" fn(Value) -> Value = my_step;
        let func_ptr = func as *const ();
        (*ctx).register_step.unwrap()(name, func_ptr as *const u8);
        (*ctx).functions.push("my_step".to_string());
    }
}

#[no_mangle]
pub extern "C" fn my_step(input: Value) -> Value {
    // Transform input or return a computed value
    input
}
