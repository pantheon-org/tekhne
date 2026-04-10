# Scenario 02: Configure Module Boundary Rules for a Layered Nx Workspace

## User Prompt

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

## Expected Behavior

1. Configure `@nx/enforce-module-boundaries` as `"error"` in an overrides entry targeting TypeScript and JavaScript files
2. Set `enforceBuildableLibDependency: true` and `allow: []` in the rule options
3. Add a `depConstraints` entry for `scope:web` that allows only `scope:web` and `scope:shared`
4. Add a `depConstraints` entry for `scope:api` that allows only `scope:api` and `scope:shared`
5. Add a `depConstraints` entry for `type:feature` that allows `type:feature`, `type:ui`, `type:data-access`, and `type:util`
6. Add a `depConstraints` entry for `type:ui` that allows only `type:ui` and `type:util`
7. Add a `depConstraints` entry for `type:util` with an empty or explicitly restrictive `onlyDependOnLibsWithTags`

## Success Criteria

- **@nx/enforce-module-boundaries rule configured**: The overrides array contains an entry with `@nx/enforce-module-boundaries` set to `"error"`.
- **enforceBuildableLibDependency: true and allow: []**: The rule options have `enforceBuildableLibDependency` set to `true` and `allow` set to `[]`.
- **scope:web constraint present and correct**: `depConstraints` contains an entry with `sourceTag` `"scope:web"` that `onlyDependOnLibsWithTags` `["scope:web", "scope:shared"]`.
- **scope:api constraint present and correct**: `depConstraints` contains an entry with `sourceTag` `"scope:api"` that `onlyDependOnLibsWithTags` `["scope:api", "scope:shared"]`.
- **type:feature constraint present and correct**: `depConstraints` contains an entry for `type:feature` that allows `type:feature`, `type:ui`, `type:data-access`, and `type:util`.
- **type:ui constraint present and correct**: `depConstraints` contains an entry for `type:ui` that only allows `type:ui` and `type:util`.
- **type:util constraint present as dependency sink**: `depConstraints` contains an entry for `type:util` with an empty or explicitly restrictive `onlyDependOnLibsWithTags` (no upstream allowed).

## Failure Conditions

- `@nx/enforce-module-boundaries` is absent or not set to `"error"`
- `enforceBuildableLibDependency` is not `true` or `allow` is not `[]`
- `scope:web` or `scope:api` boundary constraint is missing or allows dependencies on the wrong scope
- `type:feature` constraint is missing or omits any of the four allowed type tags
- `type:ui` constraint is missing or incorrectly allows types other than `type:ui` and `type:util`
- `type:util` constraint is absent, leaving it without a dependency sink restriction
