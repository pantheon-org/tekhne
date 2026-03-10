#!/usr/bin/env sh
# Setup script for scenario-2: pre-create the existing plan structure
# so the agent finds real files on disk and the grader can verify they are untouched.

set -e

PLAN_DIR=".context/plans/plan-blog-platform"

mkdir -p "$PLAN_DIR/phases/phase-01-workspace-bootstrap/tasks"
mkdir -p "$PLAN_DIR/phases/phase-02-data-model/tasks"

cat > "$PLAN_DIR/README.md" << 'EOF'
# Plan: Blog Platform

## Phases

- [Phase 01 — Workspace Bootstrap](phases/phase-01-workspace-bootstrap/README.md)
- [Phase 02 — Data Model](phases/phase-02-data-model/README.md)
EOF

cat > "$PLAN_DIR/phases/phase-01-workspace-bootstrap/README.md" << 'EOF'
# Phase 01 — Workspace Bootstrap

## Goal

Initialise the Node.js/TypeScript project with toolchain, linting, and CI skeleton.

## Gate

```sh
npm ci && npm run build && echo "ok"
```

## Dependencies

None.
EOF

cat > "$PLAN_DIR/phases/phase-01-workspace-bootstrap/tasks/task-P01T01-init-package-json.md" << 'EOF'
# P01T01 — Initialise package.json

## Description

Run `npm init -y` and configure `name`, `version`, `scripts.build`, and `scripts.test`.

## File

`package.json`

## Verification

```sh
node -e "require('./package.json')" && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-01-workspace-bootstrap/tasks/task-P01T02-tsconfig.md" << 'EOF'
# P01T02 — Add tsconfig.json

## Description

Create `tsconfig.json` with `strict: true`, `outDir: dist`, `rootDir: src`.

## File

`tsconfig.json`

## Verification

```sh
npx tsc --noEmit && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-01-workspace-bootstrap/tasks/task-P01T03-ci-skeleton.md" << 'EOF'
# P01T03 — Add CI skeleton

## Description

Create `.github/workflows/ci.yml` with a single job that runs `npm ci && npm test`.

## File

`.github/workflows/ci.yml`

## Verification

```sh
test -f .github/workflows/ci.yml && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/README.md" << 'EOF'
# Phase 02 — Data Model

## Goal

Define the SQLite schema and migration tooling for posts, authors, and tags.

## Gate

```sh
npm run migrate && sqlite3 blog.db ".tables" | grep posts && echo "ok"
```

## Dependencies

Phase 01 complete.
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/tasks/task-P02T01-schema-sql.md" << 'EOF'
# P02T01 — Write initial schema.sql

## Description

Create `db/schema.sql` with CREATE TABLE statements for `posts`, `authors`, `tags`,
and `post_tags`. Include `created_at`, `updated_at`, and `deleted_at` columns.

## File

`db/schema.sql`

## Verification

```sh
sqlite3 /tmp/test.db < db/schema.sql && sqlite3 /tmp/test.db ".tables" | grep posts && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/tasks/task-P02T02-migrate-script.md" << 'EOF'
# P02T02 — Add migrate npm script

## Description

Add `scripts/migrate.ts` that reads `db/schema.sql` and runs it against `blog.db`
using `better-sqlite3`. Wire it to `npm run migrate`.

## File

`scripts/migrate.ts`

## Verification

```sh
npm run migrate && test -f blog.db && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/tasks/task-P02T03-post-repository.md" << 'EOF'
# P02T03 — Implement PostRepository

## Description

Create `src/db/PostRepository.ts` with `findAll(page)`, `findById(id)`,
`create(data)`, `update(id, data)`, and `softDelete(id)` methods.

## File

`src/db/PostRepository.ts`

## Verification

```sh
npx ts-node -e "const r = require('./src/db/PostRepository'); console.log(typeof r.PostRepository)" | grep function && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/tasks/task-P02T04-repository-tests.md" << 'EOF'
# P02T04 — Unit tests for PostRepository

## Description

Create `src/db/PostRepository.test.ts` with tests for all five repository methods
using an in-memory SQLite database.

## File

`src/db/PostRepository.test.ts`

## Verification

```sh
npm test -- --testPathPattern=PostRepository && echo "ok"
```
EOF

echo "Scenario-2 setup complete: plan-blog-platform created with 2 phases and 7 task files."
