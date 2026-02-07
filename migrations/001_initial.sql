-- Example Database Migration
-- 
-- This file demonstrates how to create database migrations for your plugin.
-- Migrations are run automatically when the plugin is installed or updated.
--
-- Migration files should be named: {version}_{description}.sql
-- Example: 001_initial.sql, 002_add_user_preferences.sql
--
-- Migrations are executed in order based on the version number.

-- Example: Create a table for plugin-specific data
CREATE TABLE IF NOT EXISTS example_plugin_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Create an index for faster lookups
CREATE INDEX IF NOT EXISTS idx_example_plugin_data_key ON example_plugin_data(key);
