# Caching Strategies

Use this reference to optimize local and CI task execution.

## Local Cache Basics

- Enable caching for stable targets (`build`, `test`, `lint`, `e2e`).
- Ensure target `outputs` are explicit when artifacts are produced.
- Keep named inputs focused; avoid over-broad invalidation.

## Remote Cache Options

- Nx Cloud for managed remote caching.
- Self-hosted runners/plugins for regulated environments.

## Optimization Patterns

- Use `dependsOn` with `^build` where dependency builds are required.
- Keep CI parallelism tuned to compute resources.
- Reset cache after significant executor/plugin refactors.

## Common Pitfalls

- Cache enabled but no outputs declared.
- Shared config files missing from named inputs.
- Mixing inconsistent environment variables across CI jobs.
