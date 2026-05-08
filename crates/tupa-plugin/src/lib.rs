use libloading::{Library, Symbol};
use serde_json::Value;
use std::ffi::CStr;
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;
use tupa_runtime::Runtime;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Failed to load library: {0}")]
    LibraryLoad(String),

    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),

    #[error("Plugin registration failed: {0}")]
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

    pub fn register_all(&self, runtime: &Runtime) {
        for plugin in &self.plugins {
            for func_name in &plugin.functions {
                let closure_name = func_name.clone();
                runtime.register_step(func_name.as_str(), move |input| {
                    let _ = input;
                    Err(format!("Plugin function {} not loaded", closure_name))
                });
            }
        }
    }

    pub fn list_functions(&self) -> Vec<(String, Vec<String>)> {
        self.plugins
            .iter()
            .map(|p| (p.name.clone(), p.functions.clone()))
            .collect()
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

#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const i8 {
    static NAME: &str = "my_plugin";
    NAME.as_ptr() as *const i8
}

#[no_mangle]
pub extern "C" fn _tupa_plugin_register(ctx: *mut PluginRegisterContext) {
    unsafe {
        let name = b"my_step\0".as_ptr() as *const i8;
        (*ctx).register_step.unwrap()(name, std::ptr::null());
        (*ctx).functions.push("my_step".to_string());
    }
}

#[no_mangle]
pub extern "C" fn my_step(input: Value) -> Value {
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
