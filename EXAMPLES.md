# Plugin Examples

This document provides practical examples for common plugin use cases.

## Example 1: Activity Counter Plugin

Count activities and display statistics.

### Backend (`src/plugin.rs`)

```rust
use super::*;
use std::sync::{Arc, Mutex};

pub struct ActivityCounter {
    name: String,
    version: String,
    count: Arc<Mutex<u64>>,
}

impl ActivityCounter {
    pub fn new() -> Self {
        Self {
            name: "activity-counter".to_string(),
            version: "1.0.0".to_string(),
            count: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn get_count(&self) -> u64 {
        *self.count.lock().unwrap()
    }
}

impl Plugin for ActivityCounter {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn on_init(&mut self, _api: &PluginAPI) -> Result<()> {
        println!("ActivityCounter: Initialized");
        Ok(())
    }
    
    fn on_start(&mut self) -> Result<()> {
        *self.count.lock().unwrap() = 0;
        println!("ActivityCounter: Started, counter reset");
        Ok(())
    }
    
    fn on_stop(&mut self) -> Result<()> {
        let count = *self.count.lock().unwrap();
        println!("ActivityCounter: Stopped, total activities: {}", count);
        Ok(())
    }
    
    fn on_activity_recorded(&mut self, _activity: &Activity) -> Result<()> {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        println!("ActivityCounter: Activity count: {}", *count);
        Ok(())
    }
    
    fn on_category_created(&mut self, _category: &Category) -> Result<()> {
        Ok(())
    }
    
    fn migrations(&self) -> Vec<Migration> {
        vec![]
    }
    
    fn commands(&self) -> Vec<String> {
        vec!["get_activity_count".to_string()]
    }
}
```

### Frontend (`frontend/src/index.tsx`)

```tsx
import React, { useState, useEffect } from 'react';

export const ActivityCounterWidget: React.FC = () => {
  const [count, setCount] = useState<number>(0);
  
  useEffect(() => {
    // This would call a Tauri command exposed by the plugin
    // const fetchCount = async () => {
    //   const result = await invoke('get_activity_count');
    //   setCount(result);
    // };
    // fetchCount();
    // const interval = setInterval(fetchCount, 5000);
    // return () => clearInterval(interval);
  }, []);
  
  return (
    <div className="p-4 bg-blue-50 rounded-lg">
      <h3 className="text-lg font-semibold mb-2">Activity Counter</h3>
      <p className="text-3xl font-bold">{count}</p>
      <p className="text-sm text-gray-600 mt-1">activities tracked</p>
    </div>
  );
};

export default {
  ActivityCounterWidget,
};
```

## Example 2: Category Statistics Plugin

Track time spent in each category.

### Backend (`src/plugin.rs`)

```rust
use super::*;
use std::collections::HashMap;

pub struct CategoryStats {
    name: String,
    version: String,
    stats: Arc<Mutex<HashMap<i64, u64>>>, // category_id -> seconds
}

impl CategoryStats {
    pub fn new() -> Self {
        Self {
            name: "category-stats".to_string(),
            version: "1.0.0".to_string(),
            stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Plugin for CategoryStats {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn on_init(&mut self, _api: &PluginAPI) -> Result<()> {
        Ok(())
    }
    
    fn on_start(&mut self) -> Result<()> {
        self.stats.lock().unwrap().clear();
        Ok(())
    }
    
    fn on_stop(&mut self) -> Result<()> {
        Ok(())
    }
    
    fn on_activity_recorded(&mut self, activity: &Activity) -> Result<()> {
        if let Some(category_id) = activity.category_id {
            if let Some(ended_at) = activity.ended_at {
                let duration = (ended_at - activity.started_at) as u64;
                let mut stats = self.stats.lock().unwrap();
                *stats.entry(category_id).or_insert(0) += duration;
            }
        }
        Ok(())
    }
    
    fn on_category_created(&mut self, _category: &Category) -> Result<()> {
        Ok(())
    }
    
    fn migrations(&self) -> Vec<Migration> {
        vec![]
    }
    
    fn commands(&self) -> Vec<String> {
        vec!["get_category_stats".to_string()]
    }
}
```

## Example 3: Plugin with Database Storage

Store plugin data in the database.

### Migration (`migrations/001_initial.sql`)

```sql
CREATE TABLE IF NOT EXISTS my_plugin_storage (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_my_plugin_storage_key ON my_plugin_storage(key);
```

### Backend (`src/plugin.rs`)

```rust
impl Plugin for MyPlugin {
    fn migrations(&self) -> Vec<Migration> {
        vec![
            Migration {
                version: 1,
                sql: include_str!("../migrations/001_initial.sql").to_string(),
            },
        ]
    }
    
    // Use PluginAPI to interact with database
    fn on_init(&mut self, api: &PluginAPI) -> Result<()> {
        // Example: Store plugin configuration
        // api.db.execute(
        //     "INSERT OR REPLACE INTO my_plugin_storage (key, value) VALUES (?1, ?2)",
        //     &["config", "{\"enabled\": true}"]
        // )?;
        Ok(())
    }
}
```

## Example 4: Event Subscription Plugin

Subscribe to TimeTracker events.

### Backend (`src/plugin.rs`)

```rust
pub struct EventSubscriber {
    name: String,
    version: String,
    subscription_id: Option<SubscriptionId>,
}

impl Plugin for EventSubscriber {
    fn on_init(&mut self, api: &PluginAPI) -> Result<()> {
        // Subscribe to activity recorded events
        let callback = Box::new(|event: Event| {
            println!("Event received: {:?}", event);
        });
        
        self.subscription_id = Some(
            api.subscribe(EventType::ActivityRecorded, callback)
        );
        
        Ok(())
    }
    
    fn on_stop(&mut self) -> Result<()> {
        // Unsubscribe from events
        // api.unsubscribe(self.subscription_id)?;
        self.subscription_id = None;
        Ok(())
    }
}
```

## Example 5: Settings Component with State

React component that manages plugin settings.

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
    // Load settings from plugin storage
    // const loadSettings = async () => {
    //   const saved = await invoke('get_plugin_settings');
    //   if (saved) setSettings(saved);
    //   setLoading(false);
    // };
    // loadSettings();
    setLoading(false);
  }, []);
  
  const saveSettings = async () => {
    // Save settings via Tauri command
    // await invoke('save_plugin_settings', { settings });
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

1. **State Management**: Use `Arc<Mutex<T>>` for shared state in Rust plugins
2. **Error Handling**: Always return proper `Result` types from trait methods
3. **Resource Cleanup**: Clean up subscriptions and resources in `on_stop`
4. **Frontend Communication**: Use Tauri commands to communicate between frontend and backend
5. **Database**: Use migrations for schema changes, never modify tables directly
6. **Testing**: Test your plugin on all target platforms before releasing
