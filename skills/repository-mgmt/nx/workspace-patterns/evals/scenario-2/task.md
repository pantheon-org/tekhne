# Task: Configure module boundary rules for a layered Nx workspace

You are enforcing dependency constraints in an Nx workspace that uses the following tag taxonomy:

**Scope tags:** `scope:web`, `scope:api`, `scope:shared`
**Type tags:** `type:app`, `type:feature`, `type:ui`, `type:data-access`, `type:util`

The dependency rules are:

1. Projects tagged `scope:web` may only depend on projects tagged `scope:web` or `scope:shared`.
2. Projects tagged `scope:api` may only depend on projects tagged `scope:api` or `scope:shared`.
3. Projects tagged `type:feature` may depend on `type:feature`, `type:ui`, `type:data-access`, and `type:util`.
4. Projects tagged `type:ui` may only depend on `type:ui` and `type:util`.
5. Projects tagged `type:util` have no allowed upstream dependencies (they are dependency sinks).

Produce a `.eslintrc.json` at the workspace root that enforces these boundaries using `@nx/enforce-module-boundaries`.

## Requirements

- Use the `overrides` array with `files` targeting `*.ts`, `*.tsx`, `*.js`, `*.jsx`.
- Set `enforceBuildableLibDependency: true`.
- Set `allow: []`.
- The `depConstraints` array must encode all five rules above.

## Output

Produce the file `.eslintrc.json`.
