# TimeTracker Plugin Template

A template repository for creating plugins for the [TimeTracker](https://github.com/bthos/time-tracker-app) application.

## Overview

This template provides a complete foundation for building TimeTracker plugins, including:

- **Rust Backend**: Dynamic library that integrates with TimeTracker's plugin system
- **React Frontend**: UI components that can be embedded in TimeTracker's interface
- **Build System**: GitHub Actions workflow for automated cross-platform builds
- **Plugin Manifest**: TOML configuration file defining plugin metadata

## Quick Start

### 1. Use This Template

Click "Use this template" on GitHub to create your own plugin repository, or:

```bash
git clone https://github.com/your-org/time-tracker-plugin-template.git my-plugin
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
min_core_version = "0.3.0"
max_core_version = "1.0.0"
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

### 4. Implement Your Plugin

Edit `src/plugin.rs` to implement your plugin logic:

```rust
impl Plugin for MyPlugin {
    fn on_init(&mut self, api: &PluginAPI) -> Result<()> {
        // Initialize your plugin
        Ok(())
    }
    
    fn on_activity_recorded(&mut self, activity: &Activity) -> Result<()> {
        // React to new activities
        Ok(())
    }
    
    // ... implement other trait methods
}
```

### 5. Build Your Plugin

#### Local Development

```bash
# Build Rust backend
cargo build --release

# Build frontend
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
│   ├── lib.rs                    # Plugin trait and API definitions
│   └── plugin.rs                 # Your plugin implementation
├── frontend/
│   ├── src/
│   │   └── index.tsx            # React components
│   ├── package.json
│   ├── vite.config.ts
│   └── tsconfig.json
├── migrations/
│   └── 001_initial.sql          # Database migrations
├── plugin.toml                   # Plugin manifest
├── Cargo.toml                    # Rust dependencies
├── .gitignore
└── README.md
```

## Plugin Manifest (plugin.toml)

The `plugin.toml` file defines your plugin's metadata and configuration:

### [plugin] Section

- `name`: Unique identifier for your plugin (used in URLs and internal references)
- `display_name`: Human-readable name shown in the UI
- `version`: Semantic version (e.g., "1.0.0")
- `author`: Plugin author name
- `description`: Brief description of what your plugin does
- `repository`: GitHub repository URL
- `license`: License identifier (MIT, Apache-2.0, etc.)
- `api_version`: Plugin API version your plugin targets
- `min_core_version`: Minimum TimeTracker version required
- `max_core_version`: Maximum TimeTracker version supported

### [backend] Section

- `crate_name`: Rust crate name (must match Cargo.toml)
- `library_name`: Name of the compiled library (without lib prefix)
- `entry_point`: Function name exported from lib.rs (usually `plugin_init`)

### [frontend] Section

- `entry`: Path to compiled JavaScript bundle
- `components`: List of React component names to export

### [build] Section

- `targets`: List of Rust target triples to build for

## Plugin API

### Plugin Trait

All plugins must implement the `Plugin` trait:

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn on_init(&mut self, api: &PluginAPI) -> Result<()>;
    fn on_start(&mut self) -> Result<()>;
    fn on_stop(&mut self) -> Result<()>;
    fn on_activity_recorded(&mut self, activity: &Activity) -> Result<()>;
    fn on_category_created(&mut self, category: &Category) -> Result<()>;
    fn migrations(&self) -> Vec<Migration>;
    fn commands(&self) -> Vec<String>;
}
```

### Plugin API

The `PluginAPI` provides access to TimeTracker functionality:

```rust
impl PluginAPI {
    pub fn get_activities(&self, start: i64, end: i64) -> Result<Vec<Activity>>;
    pub fn create_activity(&self, activity: Activity) -> Result<i64>;
    pub fn subscribe(&self, event: EventType, callback: Box<dyn Fn(Event)>) -> SubscriptionId;
}
```

### Lifecycle Hooks

- **`on_init`**: Called when the plugin is first loaded. Use this to set up your plugin.
- **`on_start`**: Called when the plugin is enabled/started.
- **`on_stop`**: Called when the plugin is disabled/stopped.
- **`on_activity_recorded`**: Called whenever a new activity is recorded.
- **`on_category_created`**: Called when a new category is created.

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

## Database Migrations

Create SQL migration files in the `migrations/` directory:

```
migrations/
├── 001_initial.sql
├── 002_add_user_preferences.sql
└── 003_add_analytics_table.sql
```

Migrations are executed automatically when the plugin is installed or updated.

Example migration:

```sql
CREATE TABLE IF NOT EXISTS my_plugin_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);
```

Return migrations from your plugin:

```rust
fn migrations(&self) -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            sql: include_str!("../migrations/001_initial.sql").to_string(),
        },
    ]
}
```

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
3. Copy `plugin.toml` and frontend build to the same directory
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

## Best Practices

### Versioning

- Use [Semantic Versioning](https://semver.org/)
- Increment major version for breaking API changes
- Increment minor version for new features
- Increment patch version for bug fixes

### Error Handling

Always return proper errors from trait methods:

```rust
fn on_init(&mut self, api: &PluginAPI) -> Result<()> {
    // Handle errors gracefully
    api.get_activities(0, 0)?;
    Ok(())
}
```

### Resource Management

- Clean up resources in `on_stop`
- Unsubscribe from events when stopping
- Close database connections if opened

### Security

- Validate all user input
- Don't expose sensitive data in frontend components
- Use parameterized queries for database operations

## Examples

### Example: Activity Logger Plugin

```rust
pub struct ActivityLogger {
    log_file: PathBuf,
}

impl Plugin for ActivityLogger {
    fn on_activity_recorded(&mut self, activity: &Activity) -> Result<()> {
        let log_entry = format!(
            "{}: {} - {}\n",
            activity.started_at,
            activity.app_name,
            activity.window_title
        );
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?
            .write_all(log_entry.as_bytes())?;
        Ok(())
    }
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
