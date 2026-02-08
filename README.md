# TimeTracker Plugin Template

A template repository for creating plugins for the [TimeTracker](https://github.com/bthos/time-tracker-app) application.

## Overview

This template provides a complete foundation for building TimeTracker plugins, including:

- **Rust Backend**: Dynamic library that integrates with TimeTracker's plugin system
- **React Frontend**: UI components that can be embedded in TimeTracker's interface
- **Build System**: GitHub Actions workflow for automated cross-platform builds
- **Plugin Manifest**: TOML configuration file defining plugin metadata
- **Extension API**: Extend Core entities (activities, manual_entries, categories) with custom fields

## Quick Start

### 1. Use This Template

Click "Use this template" on GitHub to create your own plugin repository, or:

```bash
git clone https://github.com/bthos/time-tracker-plugin-template.git my-plugin
cd my-plugin
rm -rf .git
git init
```

### 2. Configure Your Plugin

Edit `plugin.toml` with your plugin information:

```toml
[plugin]
name = "my-plugin"                    # Unique plugin ID
display_name = "My Plugin"            # Display name
version = "1.0.0"
author = "Your Name"
description = "Description of your plugin"
repository = "https://github.com/your-username/my-plugin"
license = "MIT"
api_version = "1.0"
min_core_version = "0.2.8"           # Minimum required TimeTracker version
```

### 3. Update Cargo.toml

Update the `[package]` section in `Cargo.toml`:

```toml
[package]
name = "my-plugin"
version = "1.0.0"
authors = ["Your Name <your.email@example.com>"]
description = "Description of your plugin"
```

**Important**: Add the plugin SDK dependency. You can either:

1. Use the SDK from the main repository pinned to a specific version (recommended for production plugins):
```toml
time-tracker-plugin-sdk = { git = "https://github.com/bthos/time-tracker-app", package = "time-tracker-plugin-sdk", rev = "003d4143c50283016206b893ce57e2cf547355d0" }
```

2. Or use a local path if developing alongside the main app:
```toml
time-tracker-plugin-sdk = { path = "../time-tracker-app/plugin-sdk" }
```

**Note**: Always pin the SDK to a specific `rev` (commit hash) in production to ensure reproducible builds and avoid breaking changes from upstream updates.

### 4. Update plugin.toml Backend Section

Make sure the backend section matches your Cargo.toml:

```toml
[backend]
crate_name = "my-plugin"              # Must match Cargo.toml [package].name
library_name = "my_plugin_backend"    # Must match Cargo.toml [lib].name
entry_point = "_plugin_create"        # Must match exported function in lib.rs
```

### 5. Implement Your Plugin

Edit `src/plugin.rs` to implement your plugin logic. The plugin must implement the `Plugin` trait from `time-tracker-plugin-sdk`:

```rust
use time_tracker_plugin_sdk::{Plugin, PluginInfo, PluginAPIInterface};

pub struct MyPlugin {
    info: PluginInfo,
}

impl Plugin for MyPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
        // Register schema extensions, model extensions, etc.
        Ok(())
    }
    
    fn invoke_command(&self, command: &str, params: serde_json::Value, api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String> {
        // Handle plugin commands
        Ok(serde_json::json!({}))
    }
    
    fn shutdown(&self) -> Result<(), String> {
        // Clean up resources
        Ok(())
    }
    
    fn get_schema_extensions(&self) -> Vec<SchemaExtension> {
        vec![]
    }
    
    fn get_frontend_bundle(&self) -> Option<Vec<u8>> {
        None
    }
}
```

### 6. Export FFI Functions

In `src/lib.rs`, export the required FFI functions:

```rust
use time_tracker_plugin_sdk::Plugin;

#[no_mangle]
pub extern "C" fn _plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(plugin::MyPlugin::new()))
}

#[no_mangle]
pub extern "C" fn _plugin_destroy(plugin: *mut dyn Plugin) {
    unsafe {
        let _ = Box::from_raw(plugin);
    }
}
```

### 7. Build Your Plugin

#### Local Development

```bash
# Build Rust backend
cargo build --release

# Build frontend (if you have frontend components)
cd frontend
npm install
npm run build
```

#### Automated Builds

Create a GitHub Release to trigger automatic builds for all platforms:

1. Create a new release tag (e.g., `v1.0.0`)
2. GitHub Actions will automatically build for Windows, macOS, and Linux
3. Download the build artifacts from the release page

## Project Structure

```
time-tracker-plugin-template/
├── .github/
│   └── workflows/
│       └── build.yml              # CI/CD workflow for building plugins
├── src/
│   ├── lib.rs                    # FFI exports (_plugin_create, _plugin_destroy)
│   └── plugin.rs                 # Your plugin implementation
├── frontend/
│   ├── src/
│   │   └── index.tsx            # React components
│   ├── package.json
│   ├── vite.config.ts
│   └── tsconfig.json
├── migrations/
│   └── 001_initial.sql          # Database migrations (optional)
├── plugin.toml                   # Plugin manifest
├── Cargo.toml                    # Rust dependencies
├── .gitignore
└── README.md
```

## Plugin Manifest (plugin.toml)

The `plugin.toml` file defines your plugin's metadata and configuration:

### [plugin] Section

- `name`: Unique identifier for your plugin (used in URLs and internal references)
- `display_name`: Human-readable name shown in the UI (optional)
- `version`: Semantic version (e.g., "1.0.0")
- `author`: Plugin author name
- `description`: Brief description of what your plugin does
- `repository`: GitHub repository URL
- `license`: License identifier (MIT, Apache-2.0, etc.)
- `api_version`: Plugin API version your plugin targets (currently "1.0")
- `min_core_version`: Minimum TimeTracker version required (e.g., "0.2.7")

### [backend] Section

- `crate_name`: Rust crate name (must match `Cargo.toml` `[package].name`)
- `library_name`: Name of the compiled library (must match `Cargo.toml` `[lib].name`)
- `entry_point`: Function name exported from lib.rs (must be `_plugin_create`)

### [frontend] Section (Optional)

- `entry`: Path to compiled JavaScript bundle
- `components`: List of React component names to export

### [build] Section

- `targets`: List of Rust target triples to build for

## Plugin API

### Plugin Trait

All plugins must implement the `Plugin` trait from `time-tracker-plugin-sdk`:

```rust
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn info(&self) -> &PluginInfo;
    
    /// Initialize the plugin
    fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String>;
    
    /// Invoke a command on the plugin
    fn invoke_command(&self, command: &str, params: serde_json::Value, api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String>;
    
    /// Shutdown the plugin
    fn shutdown(&self) -> Result<(), String>;
    
    /// Get schema extensions that this plugin requires
    fn get_schema_extensions(&self) -> Vec<SchemaExtension>;
    
    /// Get frontend bundle bytes (if plugin provides UI)
    fn get_frontend_bundle(&self) -> Option<Vec<u8>>;
}
```

### Plugin API Interface

The `PluginAPIInterface` provides access to TimeTracker functionality:

```rust
pub trait PluginAPIInterface: Send + Sync {
    /// Register a database schema extension
    fn register_schema_extension(
        &self,
        entity_type: EntityType,
        schema_changes: Vec<SchemaChange>,
    ) -> Result<(), String>;
    
    /// Register a model extension
    fn register_model_extension(
        &self,
        entity_type: EntityType,
        model_fields: Vec<ModelField>,
    ) -> Result<(), String>;
    
    /// Register query filters
    fn register_query_filters(
        &self,
        entity_type: EntityType,
        query_filters: Vec<QueryFilter>,
    ) -> Result<(), String>;
    
    /// Call a database method by name with JSON parameters
    fn call_db_method(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value, String>;
}
```

### Extension API

Plugins can extend Core entities (activities, manual_entries, categories) using the Extension API:

#### 1. Database Schema Extensions

Add columns to Core tables or create new tables:

```rust
api.register_schema_extension(
    EntityType::Activity,
    vec![
        // Create a new table
        SchemaChange::CreateTable {
            table: "my_plugin_data".to_string(),
            columns: vec![
                TableColumn {
                    name: "id".to_string(),
                    column_type: "INTEGER".to_string(),
                    primary_key: true,
                    nullable: false,
                    default: None,
                    foreign_key: None,
                },
                // ... more columns
            ],
        },
        // Add a column to activities table
        SchemaChange::AddColumn {
            table: "activities".to_string(),
            column: "custom_field".to_string(),
            column_type: "TEXT".to_string(),
            default: None,
            foreign_key: None,
        },
        // Add an index
        SchemaChange::AddIndex {
            table: "activities".to_string(),
            index: "idx_activities_custom_field".to_string(),
            columns: vec!["custom_field".to_string()],
        },
    ],
)?;
```

#### 2. Model Extensions

Add fields to Core data structures:

```rust
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
```

#### 3. Query Filters

Add custom query filters for activities:

```rust
api.register_query_filters(
    EntityType::Activity,
    vec![
        QueryFilter {
            name: "by_custom_field".to_string(),
            filter_fn: Box::new(|activities, params| {
                // Filter logic
                Ok(activities)
            }),
        },
    ],
)?;
```

### Lifecycle

- **`initialize`**: Called when the plugin is first loaded. Use this to register extensions and set up your plugin.
- **`invoke_command`**: Called when a command is invoked on your plugin. Handle your plugin's commands here.
- **`shutdown`**: Called when the plugin is unloaded. Clean up any resources here.

## Frontend Components

Export React components from `frontend/src/index.tsx`:

```tsx
export const MySettings: React.FC = () => {
  return <div>Plugin Settings UI</div>;
};

export default {
  MySettings,
};
```

List exported components in `plugin.toml`:

```toml
[frontend]
components = ["MySettings"]
```

## Database Operations

Plugins can interact with the database through the `call_db_method` API:

```rust
fn invoke_command(&self, command: &str, params: serde_json::Value, api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String> {
    match command {
        "get_my_data" => {
            // Call a database method
            api.call_db_method("get_my_data", params)
        }
        "create_my_data" => {
            api.call_db_method("create_my_data", params)
        }
        _ => Err(format!("Unknown command: {}", command)),
    }
}
```

**Note**: Database methods must be implemented in the core app's `database.rs`. For custom tables created by your plugin, you'll need to add corresponding methods to the core app or use raw SQL through the API.

## Building and Distribution

### Manual Build

Build for your current platform:

```bash
cargo build --release
cd frontend && npm run build
```

### Cross-Platform Build

Use GitHub Actions (recommended) or build locally with cross-compilation:

```bash
# Install cross-compilation tools
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add x86_64-unknown-linux-gnu

# Build for each platform
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu
```

### Creating a Release

1. Update version in `plugin.toml` and `Cargo.toml`
2. Commit changes
3. Create a git tag: `git tag v1.0.0`
4. Push tag: `git push origin v1.0.0`
5. Create a GitHub Release with the same tag
6. GitHub Actions will automatically build and attach artifacts

## Testing Your Plugin

### Local Testing

1. Build your plugin (see Building section)
2. Copy the built library to TimeTracker's plugins directory:
   - Windows: `%APPDATA%\timetracker\plugins\your-plugin-name\`
   - macOS: `~/Library/Application Support/timetracker/plugins/your-plugin-name/`
   - Linux: `~/.local/share/timetracker/plugins/your-plugin-name/`
3. Copy `plugin.toml` and frontend build (if any) to the same directory
4. Restart TimeTracker
5. Enable your plugin in Settings → Plugins

### Debugging

Enable debug logging in TimeTracker to see plugin logs:

```bash
# Windows
set RUST_LOG=debug
time-tracker-app.exe

# macOS/Linux
RUST_LOG=debug ./time-tracker-app
```

## Examples

### Example: Extending Activities with Custom Fields

```rust
fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
    // Add a custom field to activities
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
```

### Example: Creating a Custom Table

```rust
fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
    api.register_schema_extension(
        EntityType::Activity,
        vec![
            SchemaChange::CreateTable {
                table: "notes".to_string(),
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
                        name: "content".to_string(),
                        column_type: "TEXT".to_string(),
                        primary_key: false,
                        nullable: false,
                        default: None,
                        foreign_key: None,
                    },
                ],
            },
        ],
    )?;
    
    Ok(())
}
```

### Example: Custom Settings Component

```tsx
export const MyPluginSettings: React.FC = () => {
  const [enabled, setEnabled] = React.useState(false);
  
  return (
    <div className="p-4">
      <h2>My Plugin Settings</h2>
      <label>
        <input
          type="checkbox"
          checked={enabled}
          onChange={(e) => setEnabled(e.target.checked)}
        />
        Enable feature
      </label>
    </div>
  );
};
```

## Best Practices

### Versioning

- Use [Semantic Versioning](https://semver.org/)
- Increment major version for breaking API changes
- Increment minor version for new features
- Increment patch version for bug fixes

### Error Handling

Always return proper errors from trait methods:

```rust
fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
    api.register_schema_extension(...)
        .map_err(|e| format!("Failed to register schema: {}", e))?;
    Ok(())
}
```

### Resource Management

- Clean up resources in `shutdown`
- Don't hold references to API after shutdown
- Use proper error handling for all operations

### Security

- Validate all user input
- Don't expose sensitive data in frontend components
- Use parameterized queries for database operations (handled by core app)

### Extension API Best Practices

- Always register schema extensions before model extensions
- Use foreign keys for referential integrity
- Add indexes for frequently queried columns
- Keep extension fields optional when possible for compatibility

## Contributing

When contributing to this template:

1. Keep it simple and focused
2. Provide clear examples
3. Document all public APIs
4. Test on all platforms

## License

This template is licensed under the MIT License. See LICENSE file for details.

## Resources

- [TimeTracker Repository](https://github.com/bthos/time-tracker-app)
- [Rust Plugin Development Guide](https://doc.rust-lang.org/book/)
- [React Documentation](https://react.dev/)
- [Tauri Documentation](https://tauri.app/)

## Support

For issues and questions:

- Open an issue in this repository for template-related questions
- Open an issue in [TimeTracker](https://github.com/bthos/time-tracker-app) for plugin API questions