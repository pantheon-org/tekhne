# Scenario 02: Configure nx.json targetDefaults for Bun Build and Test Targets

## User Prompt

You are standardizing cache configuration across an Nx workspace that uses `@nx-bun/nx` for several projects. Rather than declaring cache inputs in every `project.json`, you want to centralize the configuration in `nx.json` targetDefaults.

Produce an `nx.json` with targetDefaults for `build` and `test` targets.

## Requirements

- **build** target defaults:
  - cache: true
  - inputs: use the `"production"` and `"^production"` named inputs
- **test** target defaults:
  - cache: true
  - inputs: use the `"default"` and `"^production"` named inputs
- The nx.json should also include a `$schema` pointing to the Nx schema URL and a `version: 2` field.

## Output

Produce the file `nx.json`.

## Expected Behavior

1. Include a `$schema` field and a `version` field with value `2`
2. Set `targetDefaults.build.cache` to `true`
3. Set `targetDefaults.build.inputs` to include both `"production"` and `"^production"`
4. Set `targetDefaults.test.cache` to `true`
5. Set `targetDefaults.test.inputs` to include both `"default"` and `"^production"`

## Success Criteria

- **Schema and version present**: `nx.json` has a `$schema` field and a `version` field (value 2).
- **build targetDefault with cache: true**: `targetDefaults.build.cache` is `true`.
- **build inputs use production and ^production**: `targetDefaults.build.inputs` contains both `"production"` and `"^production"`.
- **test targetDefault with cache: true**: `targetDefaults.test.cache` is `true`.
- **test inputs use default and ^production**: `targetDefaults.test.inputs` contains both `"default"` and `"^production"`.

## Failure Conditions

- `$schema` or `version` is missing from `nx.json`
- `targetDefaults.build.cache` is absent or `false`
- `targetDefaults.build.inputs` omits `"production"` or `"^production"`, causing incorrect cache invalidation
- `targetDefaults.test.cache` is absent or `false`
- `targetDefaults.test.inputs` omits `"default"` or `"^production"`, causing incorrect cache invalidation
