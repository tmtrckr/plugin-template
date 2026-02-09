//! TimeTracker Plugin Template
//! 
//! This is a template for creating plugins for the TimeTracker application.
//! 
//! To create your own plugin:
//! 1. Update plugin.toml with your plugin information
//! 2. Rename the crate in Cargo.toml
//! 3. Implement the Plugin trait in src/plugin.rs
//! 4. Export the _plugin_create and _plugin_destroy functions from this file

use time_tracker_plugin_sdk::{Plugin, PluginInfo};

// Import the plugin implementation
mod plugin;

/// Plugin creation function
/// This function is called by TimeTracker when loading the plugin
/// 
/// # Safety
/// This function must be exported with the exact name specified in plugin.toml
#[no_mangle]
pub extern "C" fn _plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(plugin::ExamplePlugin::new()))
}

/// Plugin destruction function
/// This function is called by TimeTracker when unloading the plugin
/// 
/// # Safety
/// This function must be exported with the exact name specified in plugin.toml
#[no_mangle]
pub extern "C" fn _plugin_destroy(plugin: *mut dyn Plugin) {
    unsafe {
        let _ = Box::from_raw(plugin);
    }
}
