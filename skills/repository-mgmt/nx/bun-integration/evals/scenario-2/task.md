# Task: Configure nx.json targetDefaults for Bun build and test targets

You are standardizing cache configuration across an Nx workspace that uses @nx-bun/nx for several projects. Rather than declaring cache inputs in every project.json, you want to centralize the configuration in nx.json targetDefaults.

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
