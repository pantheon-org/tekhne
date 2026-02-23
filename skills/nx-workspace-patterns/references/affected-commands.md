# Affected Commands

Use this reference for CI and local workflows that execute only changed scope.

## Core Commands

```bash
nx affected -t lint --base=origin/main
nx affected -t test --base=origin/main
nx affected -t build --base=origin/main
```

## CI Integration

- Ensure full git history or sufficient fetch depth for SHA resolution.
- Use CI-provided base/head SHAs when available.
- Keep affected target set aligned with required quality gates.

## Base/Head Strategy

- PR pipelines: base = target branch, head = PR commit.
- Main branch pipelines: compare against previous successful commit.

## Troubleshooting

- If affected scope is unexpectedly large, inspect changed files and named inputs.
- If unexpectedly small, verify base/head derivation and fetch depth.
