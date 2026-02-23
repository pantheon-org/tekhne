# Project Graph Configuration

Use this reference for `nx.json` and graph-level configuration patterns.

## Workspace Shape

```text
workspace/
  apps/
  libs/
  tools/
  nx.json
```

## `nx.json` Core Ideas

- Set `affected.defaultBase` to your primary integration branch.
- Define `namedInputs` so build/test/lint can share deterministic input sets.
- Use `targetDefaults` to keep task behavior consistent across projects.

## Implicit Dependencies

Use implicit/global inputs for files that impact many projects, such as:

- root TypeScript config
- lint config
- shared build or test presets

## Graph Visualization Tips

- Run `nx graph` during architecture changes.
- Validate that feature and shared layers remain acyclic.
- Use graph views in PR reviews for dependency-heavy changes.
