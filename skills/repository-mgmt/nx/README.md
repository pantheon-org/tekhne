# Nx Plugin Toolkit

Complete toolkit for creating custom Nx plugins with generators and executors.

## Included Skills

### nx-plugin-authoring

Create Nx plugins with custom generators and executors for TypeScript monorepos. Covers plugin scaffolding, Tree API usage, schema-driven options, ExecutorContext API, template generation, project-graph-safe updates, cache-aware outputs, and testable workflows.

**Use when:**
- Creating Nx plugins from scratch
- Building custom generators for scaffolding
- Implementing custom executors for task automation
- Enforcing organization-specific conventions
- Extending Nx workspace automation

## Installation

```bash
tessl install pantheon-ai/nx-plugin-toolkit
```

## Quick Start

### Create a New Plugin

```bash
npx create-nx-plugin my-plugin
cd my-plugin
```

### Add Plugin to Existing Workspace

```bash
npx nx add @nx/plugin
npx nx g @nx/plugin:plugin tools/my-plugin
```

### Generate a Generator

```bash
npx nx g generator my-plugin/src/generators/library-with-readme
```

### Generate an Executor

```bash
mkdir -p tools/executors/my-executor
```

## Key Features

- **Plugin Scaffolding**: Complete setup with `create-nx-plugin` or `@nx/plugin`
- **Tree API**: Safe, deterministic filesystem operations for generators
- **Schema-Driven Options**: Type-safe generator and executor options
- **ExecutorContext API**: Access workspace configuration and project metadata
- **Template Generation**: EJS-based file generation with variable substitution
- **Cache-Aware Execution**: Declare outputs for Nx cache integration
- **Testing Support**: Unit and E2E testing patterns for plugins

## Core Concepts

### Generators

Generators automate file and project scaffolding:
- Use Tree API exclusively (`tree.write`, `generateFiles`, `updateJson`)
- Define typed schemas for options validation
- Generate files from EJS templates
- Update project configurations safely

### Executors

Executors define reusable build, test, and development tasks:
- Return `{ success: boolean }` from executor function
- Use ExecutorContext for workspace metadata
- Declare outputs for cache awareness
- Use async file I/O for performance

## Anti-Patterns

- **Never modify files outside Tree API** - breaks dry-run and change tracking
- **Never hardcode project paths** - derive from `readProjectConfiguration().root`
- **Never skip schema validation** - causes late failures with unclear errors
- **Never mutate project config blindly** - read before update to preserve existing state
- **Never use relative executor references** - use package notation (`@scope/pkg:executor`)
- **Never omit outputs for cacheable tasks** - breaks Nx cache optimization

## Quality Metrics

- **Tessl Score**: 100% (11/11 checks passed)
- **Quality Grade**: B+ (124/140, 89%)
- **Eval Coverage**: 85% (40/47 instructions tested)
- **Documentation**: 9 reference guides, 3 knowledge-base articles

## Evaluation Scenarios

This skill includes 5 comprehensive runtime validation scenarios:

1. **Safe Generator Implementation** - Tree API, schema validation, boundary enforcement
2. **Cache-Aware Executor** - ExecutorContext API, async I/O, output declarations
3. **Template-Driven Generation** - EJS templates, file name substitution, formatFiles
4. **Safe Config Updates** - Read-before-mutate, target preservation, package references
5. **Plugin Registration** - Entry points, registry files, workspace integration

## Resources

- [Nx Plugin Development](https://nx.dev/extending-nx/intro)
- [Nx Devkit API](https://nx.dev/reference/devkit)
- [Plugin Registry](https://nx.dev/plugin-registry)

## Version

**v0.2.0** - Consolidated from 3 overlapping skills (nx-generators, nx-executors, extending-plugins)

## License

MIT
