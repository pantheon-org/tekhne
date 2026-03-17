# P02T07 — uninstall-starlight

## Phase

Phase 02 — Route Migration and Starlight Cutover

## Goal

Remove `@astrojs/starlight` from `docs/package.json` and `docs/bun.lock` by running `bun remove`, so no Starlight code remains in the dependency tree.

## File to create / modify

```
docs/package.json
docs/bun.lock
```

## Implementation

```sh
cd docs
bun remove @astrojs/starlight
```

This is a single shell command; no file editing is required. The command updates both `package.json` and `bun.lock` atomically.

## Notes

- This task must run **after** P02T01–P02T06 are complete; removing the package before the layouts and routes are in place will break the build.
- After removal, verify that no `src/` file still imports from `@astrojs/starlight` or `virtual:starlight/*`.
- If any remaining import is found, that file must be fixed before this task can be considered complete.

## Verification

```sh
cd docs
grep -r "@astrojs/starlight" src/ && echo "FAIL: starlight imports remain" || echo "ok"
grep -r "virtual:starlight" src/ && echo "FAIL: virtual:starlight imports remain" || echo "ok"
cat package.json | node -e "const d=JSON.parse(require('fs').readFileSync('/dev/stdin','utf8')); process.exit(('@astrojs/starlight' in (d.dependencies??{})) ? 1 : 0)" && echo "ok" || echo "FAIL: still in package.json"
```
