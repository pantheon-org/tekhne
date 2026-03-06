# Nx Plugin Toolkit Tile

Comprehensive toolkit for creating Nx plugins with custom generators and executors.

## Tile Information

- **Name**: `pantheon-ai/nx-plugin-toolkit`
- **Version**: `0.2.0`
- **Registry**: [tessl.io/registry/pantheon-ai/nx-plugin-toolkit](https://tessl.io/registry/pantheon-ai/nx-plugin-toolkit)
- **Visibility**: Public

## Summary

Complete Nx plugin development toolkit: create custom generators, executors, and extend Nx workspaces with reusable automation.

## Skills Included

### nx-plugin-authoring

Create Nx plugins with custom generators and executors for TypeScript monorepos.

**Coverage:**
- Plugin scaffolding and workspace setup
- Generator implementation with Tree API
- Executor implementation with ExecutorContext API
- Schema-driven options and validation
- Template generation with EJS
- Testing and troubleshooting patterns

**References:**
- 9 detailed implementation guides
- 3 conceptual knowledge-base articles
- 5 runtime evaluation scenarios

## Use Cases

1. **Organization-Specific Conventions** - Enforce standards through custom generators
2. **Reusable Scaffolding** - Create project templates with organization patterns
3. **Custom Build Tasks** - Implement specialized executors for unique workflows
4. **Developer Productivity** - Automate repetitive setup and configuration
5. **Workspace Consistency** - Maintain uniformity across monorepo projects

## Key Principles

- **Deterministic Operations**: Use Tree API for predictable filesystem changes
- **Cache Awareness**: Declare outputs for Nx cache optimization
- **Schema Validation**: Type-safe options with upfront validation
- **Progressive Disclosure**: Start simple, reference depth on demand
- **Testing First**: Validate with dry-run before broad rollouts

## Quality Metrics

- **Tessl Review**: 100% (11/11 checks)
- **Quality Audit**: B+ grade (124/140 points, 89%)
- **Eval Coverage**: 85% instruction coverage (40/47 tested)
- **Documentation**: 240-line navigation hub + 12 reference files

## Installation

```bash
tessl install pantheon-ai/nx-plugin-toolkit
```

## Quick Reference

| Topic | Skill |
| --- | --- |
| Plugin creation | nx-plugin-authoring |
| Generator patterns | nx-plugin-authoring (references/generators-guide.md) |
| Executor patterns | nx-plugin-authoring (references/executors-guide.md) |
| Tree API usage | nx-plugin-authoring (references/tree-api-reference.md) |
| Schema design | nx-plugin-authoring (references/schema-design-patterns.md) |
| Testing | nx-plugin-authoring (references/testing-and-troubleshooting.md) |

## Version History

### v0.2.0 (Current)
- Consolidated 3 overlapping skills into unified nx-plugin-authoring
- Eliminated 169 content duplications
- Added 5 comprehensive eval scenarios
- Improved quality grade: C (73%) → B+ (89%)
- Published to Tessl public registry

### v0.1.0
- Initial release with 3 separate skills (generators, executors, extending-plugins)

## Related Tiles

- `pantheon-ai/typescript-advanced` - TypeScript compiler and type system
- `pantheon-ai/biome-complete` - Biome linting and formatting

## License

MIT
