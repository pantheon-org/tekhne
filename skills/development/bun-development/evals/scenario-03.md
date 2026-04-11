# Scenario 03: Implement a Bun.serve HTTP API Server

## User Prompt

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

## Expected Behavior

1. Use `Bun.serve({ fetch(req) { ... } })` as the server entry point — no `import` from `node:http`, Express, Hono, or other frameworks
2. Read the server port from `process.env.PORT` or `Bun.env.PORT` with a fallback of `3000`; port is not hardcoded
3. Implement all four routes (`GET /tasks`, `POST /tasks`, `GET /tasks/:id`, `DELETE /tasks/:id`) with appropriate HTTP status codes (200, 201, 404)
4. Parse the POST body using `await req.json()` — Bun-native Request API; no body-parser or stream concatenation
5. Return a 404 JSON response for any unmatched route
6. Use TypeScript with typed fields for the task data structure (at minimum `title: string`, `done: boolean`, `id`); no use of `any` as a shortcut for task shape

## Success Criteria

- **Bun.serve() used as the server primitive**: The file uses `Bun.serve({ fetch(req) { ... } })` as the entry point; no import from `node:http`, Express, Hono, or other frameworks
- **PORT from environment variable**: The server port uses `process.env.PORT` or `Bun.env.PORT` with a fallback of 3000; port is not hardcoded
- **All four routes implemented**: `GET /tasks`, `POST /tasks`, `GET /tasks/:id`, and `DELETE /tasks/:id` are all handled with appropriate HTTP status codes (200, 201, 404)
- **POST body parsed correctly**: `POST /tasks` parses the JSON body using `await req.json()` (Bun-native Request API); does not use body-parser or stream concatenation
- **404 fallback handler**: Any unmatched route returns a Response with status 404 and a JSON body `{ error: 'not found' }` or similar
- **TypeScript types used**: The file uses TypeScript; task data structure has typed fields; no use of `any` as a shortcut for task shape

## Failure Conditions

- Agent imports `http` from `node:http` or uses Express, Hono, or another framework instead of `Bun.serve`
- Agent hardcodes port `3000` without reading `process.env.PORT` or `Bun.env.PORT`
- Agent omits one or more of the four required routes or uses incorrect HTTP status codes
- Agent uses a body-parser library or stream concatenation to read the POST body instead of `await req.json()`
- Agent does not add a 404 fallback for unmatched routes
- Agent uses `any` for the task shape instead of a typed interface or type alias
