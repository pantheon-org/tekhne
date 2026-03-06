# Plugin Setup

## Creating a Plugin

### New Workspace with Plugin

```bash
npx create-nx-plugin my-plugin
```

Scaffolds a complete workspace with: plugin package structure, generator templates, testing setup, and build configuration.

### Add Plugin to Existing Workspace

```bash
# Install plugin capability
npx nx add @nx/plugin

# Generate a new plugin
npx nx g plugin tools/my-plugin
```

Creates a plugin package in `tools/my-plugin` with generator scaffolding and plugin registration files.

## Plugin Components

| Component | Purpose |
|---|---|
| **Generators** | Automate code scaffolding and project creation |
| **Inferred Tasks** | Auto-configure targets (build, test, etc.) from project structure |
| **Migrations** | Update dependencies, configs, and code patterns across versions |
| **Presets** | Starting-point templates for new repositories |
