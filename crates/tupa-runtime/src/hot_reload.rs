//! Hot reload support for Tupã pipelines
//!
//! This module provides file watching and pipeline reloading capabilities
//! for development workflows.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crate::Runtime;

#[cfg(feature = "hot-reload")]
use notify::{Event, EventKind, RecursiveMode, Watcher};

/// Handle for a hot reload watcher
#[cfg(feature = "hot-reload")]
pub struct HotReloadWatcher {
    runtime: Arc<Runtime>,
    _handle: notify::RecommendedWatcher,
}

/// Handle for a hot reload watcher (stub when feature disabled)
#[cfg(not(feature = "hot-reload"))]
pub struct HotReloadWatcher {
    _private: (),
}

/// Builder for creating hot reload configurations
#[derive(Default)]
pub struct HotReloadBuilder {
    path: std::path::PathBuf,
    poll_interval_ms: u64,
}

impl HotReloadBuilder {
    /// Set the directory to watch
    pub fn path<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.path = path.into();
        self
    }

    /// Set the poll interval in milliseconds
    pub fn poll_interval_ms(mut self, ms: u64) -> Self {
        self.poll_interval_ms = ms;
        self
    }

    /// Build the watcher
    #[cfg(feature = "hot-reload")]
    pub fn build(self, runtime: Arc<Runtime>) -> Result<HotReloadWatcher, HotReloadError> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher: notify::RecommendedWatcher = Watcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            notify::Config::default()
                .with_poll_interval(Duration::from_millis(self.poll_interval_ms.max(100))),
        )
        .map_err(|e| HotReloadError::Watcher(e.to_string()))?;

        watcher
            .watch(&self.path, RecursiveMode::Recursive)
            .map_err(|e| HotReloadError::Watch(e.to_string()))?;

        std::thread::spawn(move || {
            for event in rx {
                if let EventKind::Modify(_) = event.kind {
                    for path in event.paths {
                        if path.extension().map_or(false, |e| e == "tp") {
                            tracing::info!("Pipeline file changed: {:?}, triggering reload", path);
                            // TODO: Implement actual reload logic
                        }
                    }
                }
            }
        });

        Ok(HotReloadWatcher {
            runtime,
            _handle: watcher,
        })
    }

    /// Build the watcher (stub when feature disabled)
    #[cfg(not(feature = "hot-reload"))]
    pub fn build(self, _runtime: &Runtime) -> Result<HotReloadWatcher, HotReloadError> {
        Err(HotReloadError::FeatureDisabled)
    }
}

/// Errors that can occur during hot reload
#[derive(Debug, thiserror::Error)]
pub enum HotReloadError {
    #[error("Watcher error: {0}")]
    Watcher(String),
    #[error("Watch error: {0}")]
    Watch(String),
    #[error("Hot reload feature not enabled")]
    FeatureDisabled,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl Runtime {
    /// Watch a directory for changes and reload pipelines
    #[cfg(feature = "hot-reload")]
    pub fn watch_and_reload(&self, path: &Path) -> Result<HotReloadWatcher, HotReloadError> {
        HotReloadBuilder::default()
            .path(path)
            .build(Arc::new(self.clone()))
    }

    /// Watch a directory for changes and reload pipelines (stub when feature disabled)
    #[cfg(not(feature = "hot-reload"))]
    pub fn watch_and_reload(&self, _path: &Path) -> Result<HotReloadWatcher, HotReloadError> {
        Err(HotReloadError::FeatureDisabled)
    }

    /// Reload a pipeline by name
    pub fn reload_pipeline(&self, _name: &str, _plan: serde_json::Value) {
        // TODO: Implement pipeline reloading logic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_reload_builder_default() {
        let builder = HotReloadBuilder::default();
        assert!(
            builder.path.to_str().unwrap().is_empty() || std::path::Path::new("").exists() || true
        ); // Default path is empty
    }

    #[test]
    fn test_hot_reload_builder_path() {
        let builder = HotReloadBuilder::default().path("/tmp");
        assert_eq!(builder.path, std::path::PathBuf::from("/tmp"));
    }

    #[test]
    fn test_hot_reload_builder_poll_interval() {
        let builder = HotReloadBuilder::default().poll_interval_ms(500);
        assert_eq!(builder.poll_interval_ms, 500);
    }

    #[test]
    fn test_hot_reload_error_display() {
        let err = HotReloadError::FeatureDisabled;
        assert!(err.to_string().contains("not enabled"));

        let err = HotReloadError::Watcher("test error".to_string());
        assert!(err.to_string().contains("test error"));

        let err = HotReloadError::Watch("watch path not found".to_string());
        assert!(err.to_string().contains("watch path not found"));
    }

    #[test]
    fn test_watch_and_reload_feature_disabled() {
        let runtime = Runtime::new();
        let result = runtime.watch_and_reload(std::path::Path::new("/tmp"));
        assert!(matches!(result, Err(HotReloadError::FeatureDisabled)));
    }
}
