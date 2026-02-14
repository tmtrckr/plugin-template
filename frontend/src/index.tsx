/**
 * TimeTracker Plugin Frontend Entry Point
 *
 * This module exports an object with initialize(api) and optional cleanup().
 * Register routes, sidebar items, dashboard widgets, and settings tabs in initialize.
 */

import React from 'react';

/**
 * API provided by TimeTracker to the plugin at runtime.
 * Types are minimal; the host app supplies the implementation.
 */
interface PluginFrontendAPI {
  registerSettingsTab?(name: string, component: React.ComponentType): void;
  registerComponent?(name: string, component: React.ComponentType): void;
  [key: string]: unknown;
}

/**
 * Example Settings Component
 *
 * This component can be registered in TimeTracker's plugin settings via the API.
 */
const ExampleSettings: React.FC = () => {
  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-4">Example Plugin Settings</h2>
      <p className="text-gray-600">
        This is an example settings component for your plugin.
        Customize this component to add your plugin&apos;s configuration UI.
      </p>
    </div>
  );
};

export default {
  initialize(api: PluginFrontendAPI) {
    if (typeof api.registerSettingsTab === 'function') {
      api.registerSettingsTab('ExampleSettings', ExampleSettings);
    }
    if (typeof api.registerComponent === 'function') {
      api.registerComponent('ExampleSettings', ExampleSettings);
    }
  },
  cleanup() {
    // Clean up when the plugin is unloaded
  },
};
