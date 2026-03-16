# Task: Assign tags to projects in an Nx workspace

You are working on an Nx monorepo with the following projects that currently have no tags assigned. Using the workspace tag vocabulary `scope:web`, `scope:api`, `scope:shared`, `type:app`, `type:feature`, `type:ui`, `type:data-access`, and `type:util`, assign the correct tags to each project.

## Projects to tag

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
