use libloading::{Library, Symbol};
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Failed to load library: {0}")]
    LibraryLoad(String),

    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),

    #[error("Registration failed: {0}")]
    RegistrationFailed(String),

    #[error("Invalid plugin entry point")]
    InvalidEntryPoint,
}

pub type StepFunction = Arc<dyn Fn(Value) -> Result<Value, String> + Send + Sync>;

pub struct Plugin {
    pub name: String,
    pub library: Library,
    pub functions: Vec<String>,
}

pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugin<P: AsRef<Path>>(&mut self, path: P) -> Result<&Plugin, PluginError> {
        let path = path.as_ref();
        let lib =
            unsafe { Library::new(path).map_err(|e| PluginError::LibraryLoad(e.to_string()))? };

        let name_sym: Symbol<unsafe extern "C" fn() -> *const i8> = unsafe {
            lib.get(b"_tupa_plugin_name\0")
                .map_err(|_| PluginError::SymbolNotFound("_tupa_plugin_name".to_string()))?
        };

        let name = unsafe {
            let c_str = CStr::from_ptr(name_sym());
            c_str.to_string_lossy().into_owned()
        };

        let register_sym: Symbol<unsafe extern "C" fn(*mut PluginRegisterContext)> = unsafe {
            lib.get(b"_tupa_plugin_register\0")
                .map_err(|_| PluginError::SymbolNotFound("_tupa_plugin_register".to_string()))?
        };

        let mut ctx = PluginRegisterContext {
            register_step: Some(step_register_trampoline),
            functions: Vec::new(),
        };

        unsafe { register_sym(&mut ctx) };

        let functions = ctx.functions.clone();
        let plugin = Plugin {
            name,
            library: lib,
            functions,
        };

        self.plugins.push(plugin);
        Ok(self.plugins.last().unwrap())
    }

    pub fn list_functions(&self) -> Vec<(String, Vec<String>)> {
        self.plugins
            .iter()
            .map(|p| (p.name.clone(), p.functions.clone()))
            .collect()
    }

    /// Call a plugin function by name.
    ///
    /// Looks up the function in loaded plugins and executes it with the provided JSON input.
    /// Returns the JSON result or an error if the plugin/function is not found.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut pm = PluginManager::new();
    /// pm.load_plugin("./my_plugin.so")?;
    /// let result = pm.call("my_step", json!({"x": 42}))?;
    /// ```
    pub fn call(&self, name: &str, input: Value) -> Result<Value, PluginError> {
        for plugin in &self.plugins {
            if plugin.functions.iter().any(|fn_name| fn_name == name) {
                let cname = CString::new(name).map_err(|_| PluginError::InvalidEntryPoint)?;
                unsafe {
                    type RawStepFn = unsafe extern "C" fn(Value) -> Value;
                    // Use cname.as_bytes() (no trailing null) to look up symbol
                    #[allow(improper_ctypes_definitions)]
                    let func = plugin
                        .library
                        .get::<RawStepFn>(cname.as_bytes())
                        .map_err(|_| PluginError::SymbolNotFound(name.to_string()))?;
                    return Ok(func(input));
                }
            }
        }
        Err(PluginError::SymbolNotFound(name.to_string()))
    }
}

#[repr(C)]
pub struct PluginRegisterContext {
    pub register_step: Option<extern "C" fn(name: *const i8, func: *const u8)>,
    pub functions: Vec<String>,
}

extern "C" fn step_register_trampoline(_name: *const i8, _func: *const u8) {
    let _ = unsafe { CStr::from_ptr(_name).to_string_lossy().into_owned() };
}

pub fn create_plugin_template() -> &'static str {
    r#"// TupaLang Plugin Template
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
"#
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for PluginManager {}
unsafe impl Sync for PluginManager {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_new() {
        let pm = PluginManager::new();
        assert!(pm.plugins.is_empty());
    }

    #[test]
    fn test_plugin_error_display() {
        let err = PluginError::LibraryLoad("test error".to_string());
        assert!(err.to_string().contains("test error"));

        let err = PluginError::SymbolNotFound("missing_symbol".to_string());
        assert!(err.to_string().contains("missing_symbol"));
    }

    #[test]
    fn test_plugin_manager_list_functions_empty() {
        let pm = PluginManager::new();
        assert!(pm.list_functions().is_empty());
    }

    #[test]
    fn test_create_plugin_template_not_empty() {
        let template = create_plugin_template();
        assert!(template.contains("_tupa_plugin_name"));
        assert!(template.contains("my_step"));
    }
}
