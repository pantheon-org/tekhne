# Task: Implement a Bun.serve HTTP API Server

Implement a simple REST API server for a task management application using `Bun.serve`. The server must:

- Listen on port 3000 (configurable via `PORT` environment variable, defaulting to 3000)
- Handle the following routes:
  - `GET /tasks` — return all tasks as JSON
  - `POST /tasks` — create a new task from JSON body (fields: `title: string`, `done: boolean`)
  - `GET /tasks/:id` — return a single task by ID
  - `DELETE /tasks/:id` — delete a task by ID
  - Any other route returns 404 with `{ "error": "not found" }`
- Store tasks in-memory (a simple Map or array is fine)
- Use `Bun.serve` (not Express or any other framework)
- Use TypeScript
- No Node.js http module or Express; Bun-native server only

Produce a single `server.ts` file that implements this API.
