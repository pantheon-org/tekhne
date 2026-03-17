# P02T07 — Uninstall @astrojs/starlight

## Goal

Remove the `@astrojs/starlight` package from the project entirely.

## File

`package.json` / `bun.lockb`

## Prerequisites

P02T01–P02T06 must all be complete and the build must pass before running this
task.  Uninstalling before all references are removed will break the build.

## Implementation

```sh
bun remove @astrojs/starlight
```

If `@astrojs/mdx` was only a transitive dependency of Starlight and has not yet
been added explicitly (P01T01), verify it is still present after removal:

```sh
bunx astro check 2>&1 | grep "@astrojs/mdx"
```

If missing, add it:

```sh
bunx astro add mdx --yes
```

## Verification

```sh
grep "@astrojs/starlight" package.json && echo "FAIL: still in package.json" && exit 1 \
  || echo "removed from package.json"
grep -r "@astrojs/starlight" src/ && echo "FAIL: source still references starlight" && exit 1 \
  || echo "no source references"
bunx astro build 2>&1 | grep -i "error" | grep -v "^$" | wc -l \
  | xargs -I{} test {} -eq 0 && echo "build ok after uninstall"
```
