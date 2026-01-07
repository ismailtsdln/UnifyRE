use crate::core::traits::{AnalyzerComponent, BinaryProvider};
use crate::errors::Result;
use libloading::{Library, Symbol};
use std::path::Path;

#[allow(dead_code)]
pub struct PluginManager {
    plugins: Vec<Box<dyn AnalyzerComponent>>,
    libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            libraries: Vec::new(),
        }
    }

    /// Load a plugin from a shared library file
    pub unsafe fn load_plugin<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let lib = unsafe {
            Library::new(path.as_ref().as_os_str()).map_err(|e| {
                crate::errors::UnifyError::Internal(format!("Failed to load plugin library: {}", e))
            })?
        };

        // Plugins must export a function named `unifyre_plugin_init`
        let constructor: Symbol<unsafe extern "C" fn() -> *mut dyn AnalyzerComponent> = unsafe {
            lib.get(b"unifyre_plugin_init").map_err(|e| {
                crate::errors::UnifyError::Internal(format!("Plugin missing constructor: {}", e))
            })?
        };

        let plugin_ptr = unsafe { constructor() };
        let plugin = unsafe { Box::from_raw(plugin_ptr) };

        self.plugins.push(plugin);
        self.libraries.push(lib);

        Ok(())
    }

    pub fn run_all(&self, provider: &dyn BinaryProvider) -> Vec<serde_json::Value> {
        let mut results = Vec::new();
        for plugin in &self.plugins {
            if let Ok(res) = plugin.run(provider) {
                results.push(res);
            }
        }
        results
    }
}

/// Helper macro for plugins to export their constructor
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub unsafe extern "C" fn unifyre_plugin_init()
        -> *mut dyn $crate::core::traits::AnalyzerComponent {
            let constructor: fn() -> $plugin_type = $constructor;
            let object = constructor();
            let boxed: Box<dyn $crate::core::traits::AnalyzerComponent> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
