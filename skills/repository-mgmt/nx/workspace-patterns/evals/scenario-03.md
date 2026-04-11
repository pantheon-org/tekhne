# Scenario 03: Assign Tags to Projects in an Nx Workspace

## User Prompt

You are working on an Nx monorepo with the following projects that currently have no tags assigned. Using the workspace tag vocabulary `scope:web`, `scope:api`, `scope:shared`, `type:app`, `type:feature`, `type:ui`, `type:data-access`, and `type:util`, assign the correct tags to each project.

## Projects to Tag

| Project | Location | Description |
|---------|----------|-------------|
| `web-app` | `apps/web-app` | React application served to end users |
| `admin-app` | `apps/admin-app` | React admin dashboard |
| `api-server` | `apps/api-server` | NestJS backend API server |
| `web-auth-feature` | `libs/web/auth-feature` | Auth flow feature library for the web app |
| `shared-ui-components` | `libs/shared/ui-components` | Reusable UI components shared between web and admin |
| `api-user-data-access` | `libs/api/user-data-access` | Data access layer for user management in the API |
| `shared-utils` | `libs/shared/utils` | Utility functions with no framework dependencies |

## Requirements

For each project, produce a `project.json` file at the correct path with:
- The correct `name`, `root`, and `projectType` fields.
- A `tags` array with the appropriate `scope:*` and `type:*` tags.
- An empty `targets: {}` (targets are out of scope for this task).

Ensure tags are consistent with the vocabulary above.

## Output

Produce one `project.json` file per project (7 files total), each at their respective project root path.

## Expected Behavior

1. Tag `web-app`, `admin-app`, and `api-server` all with `type:app`; `web-app` and `admin-app` with `scope:web`; `api-server` with `scope:api`
2. Tag `web-auth-feature` with `scope:web` and `type:feature`
3. Tag `shared-ui-components` with `scope:shared` and `type:ui`
4. Tag `api-user-data-access` with `scope:api` and `type:data-access`
5. Tag `shared-utils` with `scope:shared` and `type:util`
6. All 7 files must be structurally valid, with `name`, `root`, `projectType`, `tags`, and `targets` fields using only vocabulary tags from the defined taxonomy

## Success Criteria

- **Application projects tagged with type:app**: `web-app`, `admin-app`, and `api-server` `project.json` files each have `"type:app"` in their `tags` array.
- **App scope tags correct**: `web-app` and `admin-app` have `"scope:web"`; `api-server` has `"scope:api"`.
- **web-auth-feature tagged correctly**: `web-auth-feature` `project.json` has tags `["scope:web", "type:feature"]` (or equivalent correct combination).
- **shared-ui-components tagged correctly**: `shared-ui-components` `project.json` has tags `["scope:shared", "type:ui"]`.
- **api-user-data-access tagged correctly**: `api-user-data-access` `project.json` has tags `["scope:api", "type:data-access"]`.
- **shared-utils tagged correctly**: `shared-utils` `project.json` has tags `["scope:shared", "type:util"]`.
- **All project.json files are structurally valid**: All 7 files have `name`, `root`, `projectType`, `tags`, and `targets` fields. Tags use only vocabulary from the defined taxonomy.

## Failure Conditions

- Any application project (`web-app`, `admin-app`, `api-server`) is missing the `type:app` tag
- `web-app` or `admin-app` is tagged with `scope:api` instead of `scope:web`, or vice versa
- `web-auth-feature` is missing `scope:web` or `type:feature`
- `shared-ui-components` is missing `scope:shared` or uses `type:feature` instead of `type:ui`
- `api-user-data-access` is missing `scope:api` or `type:data-access`
- `shared-utils` is missing `scope:shared` or uses a type other than `type:util`
- Any project.json uses tags outside the defined vocabulary
