//! TimeTracker Plugin Template
//! 
//! This is a template for creating plugins for the TimeTracker application.
//! 
//! To create your own plugin:
//! 1. Update plugin.toml with your plugin information
//! 2. Rename the crate in Cargo.toml
//! 3. Implement the Plugin trait in src/plugin.rs
//! 4. Export the plugin_init function from this file

use std::sync::Arc;

// Plugin API types (these should match TimeTracker core)
// In a real implementation, these would be provided by a plugin-sdk crate
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Plugin trait that all TimeTracker plugins must implement
pub trait Plugin: Send + Sync {
    /// Returns the plugin name
    fn name(&self) -> &str;
    
    /// Returns the plugin version
    fn version(&self) -> &str;
    
    /// Called when the plugin is initialized
    /// Provides access to the PluginAPI for interacting with TimeTracker
    fn on_init(&mut self, api: &PluginAPI) -> Result<()>;
    
    /// Called when the plugin is started
    fn on_start(&mut self) -> Result<()>;
    
    /// Called when the plugin is stopped
    fn on_stop(&mut self) -> Result<()>;
    
    /// Called when a new activity is recorded
    fn on_activity_recorded(&mut self, activity: &Activity) -> Result<()>;
    
    /// Called when a category is created
    fn on_category_created(&mut self, category: &Category) -> Result<()>;
    
    /// Returns database migrations for this plugin
    fn migrations(&self) -> Vec<Migration>;
    
    /// Returns Tauri commands exposed by this plugin
    fn commands(&self) -> Vec<String>;
}

/// Plugin API for accessing TimeTracker functionality
pub struct PluginAPI {
    // Database connection (provided by TimeTracker core)
    // pub db: Arc<Database>,
    
    // Event emitter for subscribing to events
    // pub events: EventEmitter,
}

impl PluginAPI {
    /// Get activities within a time range
    pub fn get_activities(&self, _start: i64, _end: i64) -> Result<Vec<Activity>> {
        // Implementation provided by TimeTracker core
        Ok(vec![])
    }
    
    /// Create a new activity
    pub fn create_activity(&self, _activity: Activity) -> Result<i64> {
        // Implementation provided by TimeTracker core
        Ok(0)
    }
    
    /// Subscribe to events
    pub fn subscribe(&self, _event: EventType, _callback: Box<dyn Fn(Event)>) -> SubscriptionId {
        // Implementation provided by TimeTracker core
        SubscriptionId(0)
    }
}

/// Activity record
#[derive(Debug, Clone)]
pub struct Activity {
    pub id: Option<i64>,
    pub app_name: String,
    pub window_title: String,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub category_id: Option<i64>,
}

/// Category record
#[derive(Debug, Clone)]
pub struct Category {
    pub id: Option<i64>,
    pub name: String,
    pub color: String,
}

/// Database migration
pub struct Migration {
    pub version: i32,
    pub sql: String,
}

/// Event types
pub enum EventType {
    ActivityRecorded,
    CategoryCreated,
    ProjectCreated,
}

/// Event
pub struct Event {
    pub event_type: EventType,
    pub data: serde_json::Value,
}

/// Subscription ID
#[derive(Debug, Clone, Copy)]
pub struct SubscriptionId(pub i64);

// Import the plugin implementation
mod plugin;

/// Plugin initialization function
/// This function is called by TimeTracker when loading the plugin
/// 
/// # Safety
/// This function must be exported with the exact name specified in plugin.toml
#[no_mangle]
pub extern "C" fn plugin_init() -> *mut dyn Plugin {
    Box::into_raw(Box::new(plugin::ExamplePlugin::new()))
}
