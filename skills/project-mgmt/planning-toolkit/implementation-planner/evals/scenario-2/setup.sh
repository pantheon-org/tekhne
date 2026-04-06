#!/usr/bin/env sh
# Setup script for scenario-2: pre-create the existing plan structure
# so the agent finds real files on disk and the grader can verify they are untouched.

set -e

PLAN_DIR=".context/plans/plan-blog-platform"

mkdir -p "$PLAN_DIR/phases/phase-01-workspace-bootstrap/tasks"
mkdir -p "$PLAN_DIR/phases/phase-02-data-model/tasks"

cat > "$PLAN_DIR/README.md" << 'EOF'
# Implementation Plan — Blog Platform

## Goal

Build a TypeScript blog platform with SQLite persistence, REST API, and CI pipeline.

## Phases

| # | Phase | Status | Tasks |
|---|-------|--------|-------|
| 01 | Workspace Bootstrap | pending | 3 |
| 02 | Data Model | pending | 4 |
EOF

cat > "$PLAN_DIR/phases/phase-01-workspace-bootstrap/README.md" << 'EOF'
# Phase 01 — Workspace Bootstrap

## Goal

Initialise the Node.js/TypeScript project with toolchain, linting, and CI skeleton.

## Gate

- [ ] `npm ci && npm run build` exits 0
- [ ] `npm test` exits 0

## Dependencies

None.

## Tasks

### P01T01 — Initialise package.json

[task-P01T01-init-package-json.md](tasks/task-P01T01-init-package-json.md)

### P01T02 — Add tsconfig.json

[task-P01T02-tsconfig.md](tasks/task-P01T02-tsconfig.md)

### P01T03 — Add CI skeleton

[task-P01T03-ci-skeleton.md](tasks/task-P01T03-ci-skeleton.md)
EOF

cat > "$PLAN_DIR/phases/phase-01-workspace-bootstrap/tasks/task-P01T01-init-package-json.md" << 'EOF'
# P01T01 — Initialise package.json

## Phase

01 — Workspace Bootstrap

## Goal

Run `npm init -y` and configure `name`, `version`, `scripts.build`, and `scripts.test`.

## File to create / modify

```
package.json
```

## Implementation

```json
{
  "name": "blog-platform",
  "version": "0.1.0",
  "scripts": {
    "build": "tsc",
    "test": "jest"
  }
}
```

## Verification

```sh
node -e "require('./package.json')" && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-01-workspace-bootstrap/tasks/task-P01T02-tsconfig.md" << 'EOF'
# P01T02 — Add tsconfig.json

## Phase

01 — Workspace Bootstrap

## Goal

Create `tsconfig.json` with `strict: true`, `outDir: dist`, `rootDir: src`.

## File to create / modify

```
tsconfig.json
```

## Implementation

```json
{
  "compilerOptions": {
    "strict": true,
    "outDir": "dist",
    "rootDir": "src"
  }
}
```

## Verification

```sh
npx tsc --noEmit && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-01-workspace-bootstrap/tasks/task-P01T03-ci-skeleton.md" << 'EOF'
# P01T03 — Add CI skeleton

## Phase

01 — Workspace Bootstrap

## Goal

Create `.github/workflows/ci.yml` with a single job that runs `npm ci && npm test`.

## File to create / modify

```
.github/workflows/ci.yml
```

## Implementation

```yaml
name: CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npm ci && npm test
```

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

- [ ] `npm run migrate` exits 0
- [ ] `sqlite3 blog.db ".tables"` lists `posts`

## Dependencies

Phase 01 complete.

## Tasks

### P02T01 — Write initial schema.sql

[task-P02T01-schema-sql.md](tasks/task-P02T01-schema-sql.md)

### P02T02 — Add migrate npm script

[task-P02T02-migrate-script.md](tasks/task-P02T02-migrate-script.md)

### P02T03 — Implement PostRepository

[task-P02T03-post-repository.md](tasks/task-P02T03-post-repository.md)

### P02T04 — Unit tests for PostRepository

[task-P02T04-repository-tests.md](tasks/task-P02T04-repository-tests.md)
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/tasks/task-P02T01-schema-sql.md" << 'EOF'
# P02T01 — Write initial schema.sql

## Phase

02 — Data Model

## Goal

Create `db/schema.sql` with CREATE TABLE statements for `posts`, `authors`, `tags`,
and `post_tags`. Include `created_at`, `updated_at`, and `deleted_at` columns.

## File to create / modify

```
db/schema.sql
```

## Implementation

```sql
CREATE TABLE IF NOT EXISTS authors (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS posts (id INTEGER PRIMARY KEY, title TEXT NOT NULL, created_at TEXT, updated_at TEXT, deleted_at TEXT);
CREATE TABLE IF NOT EXISTS tags (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS post_tags (post_id INTEGER, tag_id INTEGER);
```

## Verification

```sh
sqlite3 /tmp/test.db < db/schema.sql && sqlite3 /tmp/test.db ".tables" | grep posts && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/tasks/task-P02T02-migrate-script.md" << 'EOF'
# P02T02 — Add migrate npm script

## Phase

02 — Data Model

## Goal

Add `scripts/migrate.ts` that reads `db/schema.sql` and runs it against `blog.db`
using `better-sqlite3`. Wire it to `npm run migrate`.

## File to create / modify

```
scripts/migrate.ts
```

## Implementation

```typescript
import Database from "better-sqlite3";
import { readFileSync } from "fs";
const db = new Database("blog.db");
db.exec(readFileSync("db/schema.sql", "utf8"));
```

## Verification

```sh
npm run migrate && test -f blog.db && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/tasks/task-P02T03-post-repository.md" << 'EOF'
# P02T03 — Implement PostRepository

## Phase

02 — Data Model

## Goal

Create `src/db/PostRepository.ts` with `findAll(page)`, `findById(id)`,
`create(data)`, `update(id, data)`, and `softDelete(id)` methods.

## File to create / modify

```
src/db/PostRepository.ts
```

## Implementation

```typescript
export class PostRepository {
  findAll(page: number) { return []; }
  findById(id: number) { return null; }
  create(data: unknown) { return data; }
  update(id: number, data: unknown) { return data; }
  softDelete(id: number) { return id; }
}
```

## Verification

```sh
npx ts-node -e "const r = require('./src/db/PostRepository'); console.log(typeof r.PostRepository)" | grep function && echo "ok"
```
EOF

cat > "$PLAN_DIR/phases/phase-02-data-model/tasks/task-P02T04-repository-tests.md" << 'EOF'
# P02T04 — Unit tests for PostRepository

## Phase

02 — Data Model

## Goal

Create `src/db/PostRepository.test.ts` with tests for all five repository methods
using an in-memory SQLite database.

## File to create / modify

```
src/db/PostRepository.test.ts
```

## Implementation

```typescript
import { PostRepository } from "./PostRepository";
describe("PostRepository", () => {
  it("findAll returns array", () => expect(new PostRepository().findAll(1)).toEqual([]));
});
```

## Verification

```sh
npm test -- --testPathPattern=PostRepository && echo "ok"
```
EOF

echo "Scenario-2 setup complete: plan-blog-platform created with 2 phases and 7 task files."
