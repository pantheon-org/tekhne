# Task: Configure nx.json targetDefaults for cache-stable Biome targets

You are working in an Nx workspace that already has a root `biome.json` and `@biomejs/biome` installed. The workspace has no `targetDefaults` entries for Biome yet.

Produce a file `nx.json` that contains targetDefaults entries for both `biome-lint` and `biome-format` so that Nx caching is reliable.

## Requirements

- Both `biome-lint` and `biome-format` must have `cache: true`.
- Both targets must declare an `inputs` array that includes:
  - `"default"` named input
  - `"^default"` named input
  - The workspace-root biome config file: `"{workspaceRoot}/biome.json"`
  - An `externalDependencies` entry for `"@biomejs/biome"`
- The nx.json may include other typical top-level fields (e.g. `$schema`, `version`) but the targetDefaults section is what is being evaluated.

## Output

Produce the file `nx.json`.
