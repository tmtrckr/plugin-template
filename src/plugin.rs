//! Example Plugin Implementation
//! 
//! This is a simple example plugin that demonstrates the Plugin trait.
//! Replace this with your own plugin logic.

use time_tracker_plugin_sdk::{Plugin, PluginInfo, PluginAPIInterface};
use serde_json;

/// Example plugin implementation
pub struct ExamplePlugin {
    info: PluginInfo,
}

impl ExamplePlugin {
    /// Create a new instance of the example plugin
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: "example-plugin".to_string(),
                name: "Example Plugin".to_string(),
                version: "1.0.0".to_string(),
                description: Some("An example plugin template for TimeTracker".to_string()),
            },
        }
    }
}

impl Plugin for ExamplePlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
        // Register schema extensions, model extensions, etc. here
        // Example: Add a custom field to activities
        // api.register_schema_extension(
        //     EntityType::Activity,
        //     vec![
        //         SchemaChange::AddColumn {
        //             table: "activities".to_string(),
        //             column: "custom_field".to_string(),
        //             column_type: "TEXT".to_string(),
        //             default: None,
        //             foreign_key: None,
        //         },
        //     ],
        // )?;
        
        println!("ExamplePlugin: Initialized");
        Ok(())
    }
    
    fn invoke_command(&self, command: &str, params: serde_json::Value, api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String> {
        match command {
            "example_command" => {
                // Handle your plugin commands here
                // You can use api.call_db_method() to interact with the database
                Ok(serde_json::json!({
                    "status": "success",
                    "message": "Example command executed",
                    "params": params
                }))
            }
            _ => Err(format!("Unknown command: {}", command))
        }
    }
    
    fn shutdown(&self) -> Result<(), String> {
        // Clean up resources here
        println!("ExamplePlugin: Shutdown");
        Ok(())
    }
    
    fn get_schema_extensions(&self) -> Vec<time_tracker_plugin_sdk::SchemaExtension> {
        // Return schema extensions if your plugin needs to create tables or add columns
        // See EXAMPLES.md for examples
        vec![]
    }
    
    fn get_frontend_bundle(&self) -> Option<Vec<u8>> {
        // Return frontend bundle bytes if your plugin provides UI
        // The frontend should be built and included here
        None
    }
}
