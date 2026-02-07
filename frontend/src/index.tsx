/**
 * TimeTracker Plugin Frontend Entry Point
 * 
 * This file exports React components that will be loaded by TimeTracker.
 * Components listed in plugin.toml [frontend].components will be available
 * in the TimeTracker UI.
 */

import React from 'react';

/**
 * Example Settings Component
 * 
 * This component will be available in TimeTracker's plugin settings.
 * Export your components here and list them in plugin.toml.
 */
export const ExampleSettings: React.FC = () => {
  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-4">Example Plugin Settings</h2>
      <p className="text-gray-600">
        This is an example settings component for your plugin.
        Customize this component to add your plugin's configuration UI.
      </p>
    </div>
  );
};

// Export all components that should be available to TimeTracker
export default {
  ExampleSettings,
};
