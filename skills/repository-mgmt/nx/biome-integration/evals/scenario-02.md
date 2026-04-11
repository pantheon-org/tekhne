# Scenario 02: Configure nx.json targetDefaults for Cache-Stable Biome Targets

## User Prompt

You are working in an Nx workspace that already has a root `biome.json` and `@biomejs/biome` installed. The workspace has no `targetDefaults` entries for Biome yet.

Produce a file `nx.json` that contains targetDefaults entries for both `biome-lint` and `biome-format` so that Nx caching is reliable.

## Requirements

- Both `biome-lint` and `biome-format` must have `cache: true`.
- Both targets must declare an `inputs` array that includes:
  - `"default"` named input
  - `"^default"` named input
  - The workspace-root biome config file: `"{workspaceRoot}/biome.json"`
  - An `externalDependencies` entry for `"@biomejs/biome"`
- The nx.json may include other typical top-level fields (e.g. `$schema`, `version`) but the `targetDefaults` section is what is being evaluated.

## Output

Produce the file `nx.json`.

## Expected Behavior

1. Set `targetDefaults.biome-lint.cache` to `true`
2. Set `targetDefaults.biome-format.cache` to `true`
3. Include both `"default"` and `"^default"` in `biome-lint.inputs`
4. Include both `"default"` and `"^default"` in `biome-format.inputs`
5. Include `"{workspaceRoot}/biome.json"` in `biome-lint.inputs`
6. Include `"{workspaceRoot}/biome.json"` in `biome-format.inputs`
7. Include an `externalDependencies` object containing `"@biomejs/biome"` in the inputs of both targets

## Success Criteria

- **cache: true on biome-lint**: `targetDefaults.biome-lint.cache` is set to `true`.
- **cache: true on biome-format**: `targetDefaults.biome-format.cache` is set to `true`.
- **biome-lint inputs include default and ^default**: `targetDefaults.biome-lint.inputs` contains both `"default"` and `"^default"` strings.
- **biome-format inputs include default and ^default**: `targetDefaults.biome-format.inputs` contains both `"default"` and `"^default"` strings.
- **biome-lint inputs include workspaceRoot biome.json**: `targetDefaults.biome-lint.inputs` contains the string `"{workspaceRoot}/biome.json"`.
- **biome-format inputs include workspaceRoot biome.json**: `targetDefaults.biome-format.inputs` contains the string `"{workspaceRoot}/biome.json"`.
- **externalDependencies entry for @biomejs/biome on both targets**: Both `biome-lint` and `biome-format` inputs arrays contain an object with `externalDependencies` including `"@biomejs/biome"`.

## Failure Conditions

- Either `biome-lint` or `biome-format` has `cache` absent or set to `false`
- Either target's `inputs` omits `"default"` or `"^default"`
- Either target's `inputs` omits `"{workspaceRoot}/biome.json"`, meaning config changes do not invalidate the cache
- Either target's `inputs` omits the `externalDependencies` entry for `"@biomejs/biome"`, meaning version upgrades do not invalidate the cache
