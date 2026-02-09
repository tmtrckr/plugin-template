# Plugin Examples

This document provides practical examples for common plugin use cases using the `time-tracker-plugin-sdk` from crates.io.

## Example 1: Basic Plugin with Commands

A simple plugin that responds to commands.

### Backend (`src/plugin.rs`)

```rust
use time_tracker_plugin_sdk::{Plugin, PluginInfo, PluginAPIInterface};
use serde_json;

pub struct BasicPlugin {
    info: PluginInfo,
}

impl BasicPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: "basic-plugin".to_string(),
                name: "Basic Plugin".to_string(),
                version: "1.0.0".to_string(),
                description: Some("A basic example plugin".to_string()),
            },
        }
    }
}

impl Plugin for BasicPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn initialize(&mut self, _api: &dyn PluginAPIInterface) -> Result<(), String> {
        println!("BasicPlugin: Initialized");
        Ok(())
    }
    
    fn invoke_command(&self, command: &str, params: serde_json::Value, _api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String> {
        match command {
            "hello" => {
                Ok(serde_json::json!({
                    "message": "Hello from BasicPlugin!",
                    "params": params
                }))
            }
            _ => Err(format!("Unknown command: {}", command))
        }
    }
    
    fn shutdown(&self) -> Result<(), String> {
        println!("BasicPlugin: Shutdown");
        Ok(())
    }
    
    fn get_schema_extensions(&self) -> Vec<time_tracker_plugin_sdk::SchemaExtension> {
        vec![]
    }
    
    fn get_frontend_bundle(&self) -> Option<Vec<u8>> {
        None
    }
}
```

### Frontend (`frontend/src/index.tsx`)

```tsx
import React, { useState } from 'react';

export const BasicPluginWidget: React.FC = () => {
  const [message, setMessage] = useState<string>('');
  
  const handleCommand = async () => {
    // Call plugin command via Tauri
    // const result = await invoke('invoke_plugin_command', {
    //   pluginId: 'basic-plugin',
    //   command: 'hello',
    //   params: {}
    // });
    // setMessage(result.message);
  };
  
  return (
    <div className="p-4 bg-blue-50 rounded-lg">
      <h3 className="text-lg font-semibold mb-2">Basic Plugin</h3>
      <button onClick={handleCommand} className="px-4 py-2 bg-blue-500 text-white rounded">
        Call Command
      </button>
      {message && <p className="mt-2">{message}</p>}
    </div>
  );
};

export default {
  BasicPluginWidget,
};
```

## Example 2: Plugin with Schema Extensions

Extend activities with custom fields.

### Backend (`src/plugin.rs`)

```rust
use time_tracker_plugin_sdk::{Plugin, PluginInfo, PluginAPIInterface, EntityType, SchemaChange, ModelField, TableColumn};

pub struct ActivityExtensionPlugin {
    info: PluginInfo,
}

impl ActivityExtensionPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: "activity-extension".to_string(),
                name: "Activity Extension".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Adds priority field to activities".to_string()),
            },
        }
    }
}

impl Plugin for ActivityExtensionPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
        // Add a priority column to activities table
        api.register_schema_extension(
            EntityType::Activity,
            vec![
                SchemaChange::AddColumn {
                    table: "activities".to_string(),
                    column: "priority".to_string(),
                    column_type: "INTEGER".to_string(),
                    default: Some("0".to_string()),
                    foreign_key: None,
                },
            ],
        )?;
        
        // Register model extension so the field is available in Rust types
        api.register_model_extension(
            EntityType::Activity,
            vec![
                ModelField {
                    name: "priority".to_string(),
                    type_: "Option<i32>".to_string(),
                    optional: true,
                },
            ],
        )?;
        
        Ok(())
    }
    
    fn invoke_command(&self, command: &str, params: serde_json::Value, api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String> {
        match command {
            "set_priority" => {
                // Use call_db_method to interact with database
                api.call_db_method("update_activity", serde_json::json!({
                    "id": params["id"],
                    "priority": params["priority"]
                }))
            }
            _ => Err(format!("Unknown command: {}", command))
        }
    }
    
    fn shutdown(&self) -> Result<(), String> {
        Ok(())
    }
    
    fn get_schema_extensions(&self) -> Vec<time_tracker_plugin_sdk::SchemaExtension> {
        vec![]
    }
    
    fn get_frontend_bundle(&self) -> Option<Vec<u8>> {
        None
    }
}
```

## Example 3: Plugin with Custom Table

Create a custom table for plugin data.

### Backend (`src/plugin.rs`)

```rust
use time_tracker_plugin_sdk::{Plugin, PluginInfo, PluginAPIInterface, EntityType, SchemaChange, TableColumn, ForeignKey};

pub struct NotesPlugin {
    info: PluginInfo,
}

impl NotesPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: "notes-plugin".to_string(),
                name: "Notes Plugin".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Add notes to activities".to_string()),
            },
        }
    }
}

impl Plugin for NotesPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
        // Create a custom table for notes
        api.register_schema_extension(
            EntityType::Activity,
            vec![
                SchemaChange::CreateTable {
                    table: "activity_notes".to_string(),
                    columns: vec![
                        TableColumn {
                            name: "id".to_string(),
                            column_type: "INTEGER".to_string(),
                            primary_key: true,
                            nullable: false,
                            default: None,
                            foreign_key: None,
                        },
                        TableColumn {
                            name: "activity_id".to_string(),
                            column_type: "INTEGER".to_string(),
                            primary_key: false,
                            nullable: false,
                            default: None,
                            foreign_key: Some(ForeignKey {
                                table: "activities".to_string(),
                                column: "id".to_string(),
                            }),
                        },
                        TableColumn {
                            name: "note".to_string(),
                            column_type: "TEXT".to_string(),
                            primary_key: false,
                            nullable: false,
                            default: None,
                            foreign_key: None,
                        },
                        TableColumn {
                            name: "created_at".to_string(),
                            column_type: "INTEGER".to_string(),
                            primary_key: false,
                            nullable: false,
                            default: Some("(strftime('%s', 'now'))".to_string()),
                            foreign_key: None,
                        },
                    ],
                },
                SchemaChange::AddIndex {
                    table: "activity_notes".to_string(),
                    index: "idx_activity_notes_activity_id".to_string(),
                    columns: vec!["activity_id".to_string()],
                },
            ],
        )?;
        
        Ok(())
    }
    
    fn invoke_command(&self, command: &str, params: serde_json::Value, api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String> {
        match command {
            "add_note" => {
                // Note: You'll need to implement the database method in the core app
                // or use raw SQL through call_db_method
                api.call_db_method("add_activity_note", params)
            }
            "get_notes" => {
                api.call_db_method("get_activity_notes", params)
            }
            _ => Err(format!("Unknown command: {}", command))
        }
    }
    
    fn shutdown(&self) -> Result<(), String> {
        Ok(())
    }
    
    fn get_schema_extensions(&self) -> Vec<time_tracker_plugin_sdk::SchemaExtension> {
        vec![]
    }
    
    fn get_frontend_bundle(&self) -> Option<Vec<u8>> {
        None
    }
}
```

## Example 4: Settings Component

React component for plugin settings.

### Frontend (`frontend/src/index.tsx`)

```tsx
import React, { useState, useEffect } from 'react';

interface PluginSettings {
  enabled: boolean;
  threshold: number;
  notifications: boolean;
}

export const MyPluginSettings: React.FC = () => {
  const [settings, setSettings] = useState<PluginSettings>({
    enabled: false,
    threshold: 60,
    notifications: true,
  });
  
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    // Load settings via plugin command
    // const loadSettings = async () => {
    //   const saved = await invoke('invoke_plugin_command', {
    //     pluginId: 'my-plugin',
    //     command: 'get_settings',
    //     params: {}
    //   });
    //   if (saved) setSettings(saved);
    //   setLoading(false);
    // };
    // loadSettings();
    setLoading(false);
  }, []);
  
  const saveSettings = async () => {
    // Save settings via plugin command
    // await invoke('invoke_plugin_command', {
    //   pluginId: 'my-plugin',
    //   command: 'save_settings',
    //   params: { settings }
    // });
    alert('Settings saved!');
  };
  
  if (loading) {
    return <div>Loading...</div>;
  }
  
  return (
    <div className="p-4 space-y-4">
      <h2 className="text-xl font-bold">My Plugin Settings</h2>
      
      <label className="flex items-center space-x-2">
        <input
          type="checkbox"
          checked={settings.enabled}
          onChange={(e) => setSettings({ ...settings, enabled: e.target.checked })}
        />
        <span>Enable plugin</span>
      </label>
      
      <div>
        <label className="block mb-1">Threshold (seconds)</label>
        <input
          type="number"
          value={settings.threshold}
          onChange={(e) => setSettings({ ...settings, threshold: parseInt(e.target.value) })}
          className="border rounded px-2 py-1"
        />
      </div>
      
      <label className="flex items-center space-x-2">
        <input
          type="checkbox"
          checked={settings.notifications}
          onChange={(e) => setSettings({ ...settings, notifications: e.target.checked })}
        />
        <span>Enable notifications</span>
      </label>
      
      <button
        onClick={saveSettings}
        className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
      >
        Save Settings
      </button>
    </div>
  );
};

export default {
  MyPluginSettings,
};
```

## Tips

1. **SDK Version**: Always use the same SDK version that matches your target TimeTracker version
2. **Schema Extensions**: Register schema extensions in `initialize()` before using them
3. **Error Handling**: Always return proper `Result` types from trait methods
4. **Resource Cleanup**: Clean up resources in `shutdown()`
5. **Database Access**: Use `call_db_method()` to interact with the database through the core app
6. **Frontend Communication**: Use Tauri commands to communicate between frontend and backend
7. **Testing**: Test your plugin on all target platforms before releasing

## Using the SDK

Add the SDK to your `Cargo.toml`:

```toml
[dependencies]
time-tracker-plugin-sdk = "0.2.10"  # Use the published version from crates.io
```

The SDK is published on [crates.io](https://crates.io/crates/time-tracker-plugin-sdk) and should be used by all plugins instead of local path or git dependencies.
