//! Runtime extensions for TupaLang
//!
//! This module provides the `TupaExtension` trait for projects to register
//! custom step functions with the runtime.

use crate::Runtime;
use std::sync::Arc;

/// Trait for registering custom step functions with the runtime.
///
/// Projects can implement this trait to provide their own helpers
/// that can be called from TupaLang pipeline steps.
///
/// # Example
///
/// ```rust,ignore
/// use tupa_runtime::{TupaExtension, Runtime};
///
/// struct MyExtension;
/// impl TupaExtension for MyExtension {
///     fn name(&self) -> &str {
///         "my-project"
///     }
///
///     fn register(&self, runtime: &Runtime) {
///         runtime.register_step("my::weighted", |input| {
///             let score = input.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0);
///             let weight = input.get("weight").and_then(|v| v.as_f64()).unwrap_or(1.0);
///             Ok(serde_json::json!({
///                 "score": score,
///                 "weight": weight,
///                 "reason": input.get("reason").and_then(|v| v.as_str()).unwrap_or("custom")
///             }))
///         });
///     }
/// }
/// ```
pub trait TupaExtension: Send + Sync {
    /// Returns the name of this extension
    fn name(&self) -> &str;

    /// Register step functions with the runtime
    fn register(&self, runtime: &Runtime);
}

/// Extension registry for managing multiple extensions
#[derive(Default)]
pub struct ExtensionRegistry {
    extensions: Vec<Arc<dyn TupaExtension>>,
}

impl ExtensionRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    /// Register an extension
    pub fn register(&mut self, extension: Arc<dyn TupaExtension>) {
        self.extensions.push(extension);
    }

    /// Apply all registered extensions to a runtime
    pub fn apply_all(&self, runtime: &Runtime) {
        for ext in &self.extensions {
            ext.register(runtime);
        }
    }
}

impl Runtime {
    /// Register an extension with this runtime
    pub fn register_extension(&self, extension: Arc<dyn TupaExtension>) {
        extension.register(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct TestExtension {
        name: &'static str,
    }

    impl TupaExtension for TestExtension {
        fn name(&self) -> &str {
            self.name
        }

        fn register(&self, runtime: &Runtime) {
            runtime.register_step("test_ext::hello", |input: serde_json::Value| {
                let name = input
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("world");
                Ok(json!({ "message": format!("hello, {}!", name) }))
            });
        }
    }

    #[test]
    fn test_extension_registry_default() {
        let registry = ExtensionRegistry::default();
        assert!(registry.extensions.is_empty());
    }

    #[test]
    fn test_extension_registry_register() {
        let mut registry = ExtensionRegistry::new();
        registry.register(Arc::new(TestExtension { name: "test1" }));
        assert_eq!(registry.extensions.len(), 1);
    }

    #[test]
    fn test_extension_registry_apply_all() {
        let mut registry = ExtensionRegistry::new();
        registry.register(Arc::new(TestExtension { name: "test1" }));
        registry.register(Arc::new(TestExtension { name: "test2" }));

        let runtime = Runtime::new();
        registry.apply_all(&runtime);

        // The extension function should be registered
        let result = runtime.call_step_function("test_ext::hello", json!({"name": "tester"}));
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output["message"], "hello, tester!");
    }

    #[test]
    fn test_runtime_register_extension() {
        let runtime = Runtime::new();
        runtime.register_extension(Arc::new(TestExtension { name: "direct" }));

        let result = runtime.call_step_function("test_ext::hello", json!({"name": "direct"}));
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output["message"], "hello, direct!");
    }
}
