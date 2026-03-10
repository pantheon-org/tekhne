# Task: Add a new phase to an existing implementation plan

An implementation plan already exists at `.context/plans/plan-blog-platform/`. The plan currently has two phases:

- `phases/phase-01-workspace-bootstrap/` (3 tasks — all complete)
- `phases/phase-02-data-model/` (4 tasks — all complete)

You need to extend this plan with a new phase covering the **REST API layer**. The API layer must implement:

1. `GET /posts` — list all posts (paginated, 10 per page)
2. `POST /posts` — create a new post (title, body, author required)
3. `GET /posts/:id` — get a single post by ID
4. `PUT /posts/:id` — update title/body of a post
5. `DELETE /posts/:id` — delete a post (soft delete — set `deleted_at`)
6. `GET /health` — health check returning `{"status":"ok"}`

Use Express.js. All handlers must be in separate files under `src/api/`.

Add this as phase 3 to the existing plan. Do not modify or delete the existing phase files.
