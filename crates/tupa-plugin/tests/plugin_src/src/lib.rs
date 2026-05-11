use serde_json::{Value, json};
use tupa_plugin::PluginRegisterContext;

#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const i8 {
    static NAME: &str = "integration_test_plugin";
    NAME.as_ptr() as *const i8
}

#[no_mangle]
pub extern "C" fn _tupa_plugin_register(ctx: *mut PluginRegisterContext) {
    unsafe {
        let name = b"identity\0".as_ptr() as *const i8;
        let func: extern "C" fn(Value) -> Value = identity;
        let func_ptr = func as *const ();
        (*ctx).register_step.unwrap()(name, func_ptr as *const u8);
        (*ctx).functions.push("identity".to_string());

        let name2 = b"double\0".as_ptr() as *const i8;
        let func2: extern "C" fn(Value) -> Value = double;
        let func_ptr2 = func2 as *const ();
        (*ctx).register_step.unwrap()(name2, func_ptr2 as *const u8);
        (*ctx).functions.push("double".to_string());

        let name3 = b"const42\0".as_ptr() as *const i8;
        let func3: extern "C" fn(Value) -> Value = const_42;
        let func_ptr3 = func3 as *const ();
        (*ctx).register_step.unwrap()(name3, func_ptr3 as *const u8);
        (*ctx).functions.push("const42".to_string());
    }
}

#[no_mangle]
pub extern "C" fn identity(input: Value) -> Value {
    input
}

#[no_mangle]
pub extern "C" fn double(input: Value) -> Value {
    if let Some(num) = input.as_i64() {
        json!(num * 2)
    } else {
        json!(null)
    }
}

#[no_mangle]
pub extern "C" fn const_42(_input: Value) -> Value {
    json!(42)
}
