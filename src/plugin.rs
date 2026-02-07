//! Example Plugin Implementation
//! 
//! This is a simple example plugin that demonstrates the Plugin trait.
//! Replace this with your own plugin logic.

use super::*;

/// Example plugin implementation
pub struct ExamplePlugin {
    name: String,
    version: String,
    initialized: bool,
}

impl ExamplePlugin {
    /// Create a new instance of the example plugin
    pub fn new() -> Self {
        Self {
            name: "example-plugin".to_string(),
            version: "1.0.0".to_string(),
            initialized: false,
        }
    }
}

impl Plugin for ExamplePlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn on_init(&mut self, _api: &PluginAPI) -> Result<()> {
        self.initialized = true;
        println!("ExamplePlugin: Initialized");
        Ok(())
    }
    
    fn on_start(&mut self) -> Result<()> {
        println!("ExamplePlugin: Started");
        Ok(())
    }
    
    fn on_stop(&mut self) -> Result<()> {
        println!("ExamplePlugin: Stopped");
        Ok(())
    }
    
    fn on_activity_recorded(&mut self, activity: &Activity) -> Result<()> {
        println!("ExamplePlugin: Activity recorded - {} - {}", activity.app_name, activity.window_title);
        Ok(())
    }
    
    fn on_category_created(&mut self, category: &Category) -> Result<()> {
        println!("ExamplePlugin: Category created - {}", category.name);
        Ok(())
    }
    
    fn migrations(&self) -> Vec<Migration> {
        // Return any database migrations your plugin needs
        // See migrations/001_initial.sql for an example
        vec![]
    }
    
    fn commands(&self) -> Vec<String> {
        // Return list of Tauri command names exposed by this plugin
        // These commands will be registered with Tauri
        vec![]
    }
}
