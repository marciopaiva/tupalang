use libloading::{Library, Symbol};
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
/// Errors that can occur when loading or using plugins.
pub enum PluginError {
    #[error("Failed to load library: {0}")]
    /// Failed to load the dynamic library.
    LibraryLoad(String),

    #[error("Symbol not found: {0}")]
    /// Required symbol not found in the plugin library.
    SymbolNotFound(String),

    #[error("Registration failed: {0}")]
    /// Plugin registration callback failed.
    RegistrationFailed(String),

    #[error("Invalid plugin entry point")]
    /// The plugin does not export the required entry points.
    InvalidEntryPoint,
}

/// A step function type: a thread-safe, sendable closure taking a JSON value and returning a JSON value.
pub type StepFunction = Arc<dyn Fn(Value) -> Result<Value, String> + Send + Sync>;

/// A dynamically loaded plugin.
pub struct Plugin {
    /// Plugin name (from `_tupa_plugin_name`).
    pub name: String,
    /// Handle to the loaded dynamic library.
    pub library: Library,
    /// List of function names registered by this plugin.
    pub functions: Vec<String>,
}

/// Manager for loading and calling plugin functions.
pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl PluginManager {
    /// Create a new empty plugin manager.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Load a plugin from a dynamic library file (`.so`/`.dll`/`.dylib`).
    ///
    /// The library must export `_tupa_plugin_name` and `_tupa_plugin_register` C symbols.
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

    /// List all loaded plugins and their available functions.
    ///
    /// Returns iterator of `(plugin_name, [function_names])`.
    pub fn list_functions(&self) -> Vec<(String, Vec<String>)> {
        self.plugins
            .iter()
            .map(|p| (p.name.clone(), p.functions.clone()))
            .collect()
    }

    /// Call a plugin function by name with a JSON argument.
    ///
    /// Looks up the function in loaded plugins and executes it.
    /// Returns the JSON result or an error if the plugin/function is not found.
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
/// Context passed to plugin registration callback.
///
/// The plugin fills `functions` with the names of exported step functions.
pub struct PluginRegisterContext {
    /// Optional registration callback (set by plugin manager).
    pub register_step: Option<extern "C" fn(name: *const i8, func: *const u8)>,
    /// Filled by plugin during registration with available function names.
    pub functions: Vec<String>,
}

/// Trampoline used to register steps from plugin C callbacks.
extern "C" fn step_register_trampoline(_name: *const i8, _func: *const u8) {
    let _ = unsafe { CStr::from_ptr(_name).to_string_lossy().into_owned() };
}

/// Generate a Rust source template for a new Tupã plugin.
///
/// The template exports the required C entry points and a sample step function.
/// Build as a `cdylib` and load via `PluginManager`.
pub fn create_plugin_template() -> &'static str {
    r#"// Tupã Plugin Template
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
