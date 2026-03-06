---
name: extending-nx-plugins
description: Creates Nx plugins, builds custom generators, configures inferred tasks, and writes version migrations for Nx workspaces. Use when the user wants to create a custom Nx plugin, scaffold a generator with nx generate, add inferred tasks, write workspace migrations, build a custom executor, develop a plugin preset, or extend an Nx workspace with custom tooling. Covers nx plugin development, generator templates, Nx Devkit API usage, schema definitions, testing generators, and publishing plugins.
---

# Extending Nx with Plugins

Navigation hub for creating and developing Nx plugins and generators.

## When to Use

- Creating a new Nx plugin (standalone or within an existing workspace)
- Building a custom generator to automate code scaffolding
- Integrating a new build tool via a custom executor
- Writing workspace migrations for version upgrades
- Developing a preset for bootstrapping new repositories

## When Not to Use

- Configuring existing Nx targets or pipelines → see `nx-workspace-patterns`
- Running or optimizing tasks in CI → see `nx-workspace-patterns`
- Installing third-party Nx plugins (no custom code needed)

## Workflow

1. Scaffold the plugin: new workspace (`create-nx-plugin`) or add to existing (`nx g plugin`)
2. Generate a generator: `nx g generator <plugin>/src/generators/<name>`
3. Implement generator logic using Nx Devkit Tree API — see [generators.md](references/generators.md)
4. Define schema.json with typed, required fields
5. Add EJS templates under `files/` for scaffolded output
6. Test with `--dry-run` before executing; write unit tests with `createTreeWithEmptyWorkspace`
7. Publish the plugin following semver; mark breaking changes as major bumps

## Quick Reference

| Topic | Reference |
|---|---|
| Plugin scaffolding commands and plugin components | [plugin-setup.md](references/plugin-setup.md) |
| Generator implementation, templates, validation, best practices | [generators.md](references/generators.md) |
| Nx Devkit API: file ops, project management, dependencies, utilities | [devkit-api.md](references/devkit-api.md) |
| Composing generators, conditional generation, updating projects | [common-patterns.md](references/common-patterns.md) |

## Anti-Patterns

### NEVER modify files outside the Tree API

- **WHY**: direct filesystem operations bypass dry-run mode and change tracking, breaking `--dry-run` preview.
- **BAD**: `writeFileSync("libs/my-lib/README.md", content)` in generator.
- **GOOD**: `tree.write("libs/my-lib/README.md", content)` via Nx Devkit Tree API.

### NEVER hardcode workspace structure assumptions

- **WHY**: monorepo layouts evolve; hardcoded paths like `"apps/"` or `"packages/"` break when directories change.
- **BAD**: `const projectRoot = "libs/" + options.name;` assumes `libs/` convention.
- **GOOD**: derive paths from workspace layout: `readProjectConfiguration(tree, projectName).root`.

### NEVER skip schema validation or use `schema: any`

- **WHY**: untyped options cause runtime errors when invalid inputs are passed; users get cryptic failures.
- **BAD**: generator options interface with `schema: any` and no required fields.
- **GOOD**: typed schema.json with `"required": ["name"]`, explicit types, descriptions, and defaults.

### NEVER mutate project configuration blindly

- **WHY**: overwriting full `project.json` deletes existing targets and tags that other generators added.
- **BAD**: `updateProjectConfiguration(tree, name, { root, targets: { build: {...} } })` (replaces entire config).
- **GOOD**: `const config = readProjectConfiguration(tree, name); config.targets.build = {...}; updateProjectConfiguration(tree, name, config);`.

### NEVER generate across project boundaries without dependency checks

- **WHY**: adding imports from restricted scopes introduces circular dependencies or violates module boundary rules.
- **BAD**: generator in `@frontend` scope writes imports to `@backend` scope without checking tags.
- **GOOD**: read target project tags, verify allowed dependencies via `ensureProjectBoundaries` or tag rules before generating imports.

## References

- [Nx Devkit API Reference](https://nx.dev/docs/reference/devkit)
- [Enforce Best Practices Tutorial](https://nx.dev/docs/extending-nx/organization-specific-plugin)
- [Tool Integration Tutorial](https://nx.dev/docs/extending-nx/tooling-plugin)
- [Creating Files with Generators](https://nx.dev/docs/extending-nx/creating-files)
- [Plugin Registry](https://nx.dev/docs/plugin-registry)
- [Publishing Plugins](https://nx.dev/docs/extending-nx/publish-plugin)
- [Nx Community Discord](https://go.nx.dev/community)
