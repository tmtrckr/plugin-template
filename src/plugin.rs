//! Example Plugin Implementation
//! 
//! This is a simple example plugin that demonstrates the Plugin trait.
//! Replace this with your own plugin logic.

use time_tracker_plugin_sdk::{
    Plugin, 
    PluginInfo, 
    PluginAPIInterface,
    EntityType,
    SchemaChange,
    ModelField,
    ForeignKey,
    SchemaExtension,
};
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
                is_builtin: false,
            },
        }
    }
}

impl Plugin for ExamplePlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
        // Register schema extensions if your plugin needs to create tables
        // or extend Core entities (activities, manual_entries, categories)
        
        // Example: Create a custom table for your plugin
        api.register_schema_extension(
            EntityType::Activity, // This doesn't affect the entity, just groups the changes
            vec![
                SchemaChange::CreateTable {
                    table: "example_plugin_data".to_string(),
                    columns: vec![
                        time_tracker_plugin_sdk::TableColumn {
                            name: "id".to_string(),
                            column_type: "INTEGER".to_string(),
                            primary_key: true,
                            nullable: false,
                            default: None,
                            foreign_key: None,
                        },
                        time_tracker_plugin_sdk::TableColumn {
                            name: "key".to_string(),
                            column_type: "TEXT".to_string(),
                            primary_key: false,
                            nullable: false,
                            default: None,
                            foreign_key: None,
                        },
                        time_tracker_plugin_sdk::TableColumn {
                            name: "value".to_string(),
                            column_type: "TEXT".to_string(),
                            primary_key: false,
                            nullable: true,
                            default: None,
                            foreign_key: None,
                        },
                        time_tracker_plugin_sdk::TableColumn {
                            name: "created_at".to_string(),
                            column_type: "INTEGER".to_string(),
                            primary_key: false,
                            nullable: false,
                            default: None,
                            foreign_key: None,
                        },
                    ],
                },
                SchemaChange::AddIndex {
                    table: "example_plugin_data".to_string(),
                    index: "idx_example_plugin_data_key".to_string(),
                    columns: vec!["key".to_string()],
                },
            ],
        )?;
        
        // Example: Extend activities with a custom field
        // Uncomment and modify if you need to extend Core entities:
        /*
        api.register_schema_extension(
            EntityType::Activity,
            vec![
                SchemaChange::AddColumn {
                    table: "activities".to_string(),
                    column: "custom_field".to_string(),
                    column_type: "TEXT".to_string(),
                    default: None,
                    foreign_key: None,
                },
            ],
        )?;
        
        api.register_model_extension(
            EntityType::Activity,
            vec![
                ModelField {
                    name: "custom_field".to_string(),
                    type_: "Option<String>".to_string(),
                    optional: true,
                },
            ],
        )?;
        */
        
        Ok(())
    }
    
    fn invoke_command(&self, command: &str, params: serde_json::Value, api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String> {
        match command {
            "get_example_data" => {
                // Example: Call a database method
                // Note: You need to implement these methods in the core app's database.rs
                // For now, this is just an example of how to structure commands
                api.call_db_method("get_example_data", params)
            }
            "set_example_data" => {
                api.call_db_method("set_example_data", params)
            }
            _ => Err(format!("Unknown command: {}", command)),
        }
    }
    
    fn shutdown(&self) -> Result<(), String> {
        // Clean up any resources here
        Ok(())
    }
    
    fn get_schema_extensions(&self) -> Vec<SchemaExtension> {
        // Schema extensions are registered in initialize()
        // This method can return pre-defined extensions if needed
        vec![]
    }
    
    fn get_frontend_bundle(&self) -> Option<Vec<u8>> {
        // Return frontend bundle bytes if your plugin provides UI
        // The bundle should be loaded from the compiled frontend build
        None
    }
}