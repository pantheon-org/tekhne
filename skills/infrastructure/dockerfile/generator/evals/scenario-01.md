# Scenario 01: Containerize a Go Microservice for Production

## User Prompt

Your team has built a small Go REST API called `inventory-service` that exposes an HTTP server on port 8080. The service is currently deployed by copying the binary to VMs, but the team wants to migrate to Kubernetes and needs a production-quality container image. The existing CI pipeline builds and tests the Go binary, but no Docker infrastructure exists yet.

The security team has flagged that all container images must run as unprivileged users and must use minimal base images to reduce the attack surface. The platform team requires that image tags be deterministic and reproducible — they've had incidents in the past where builds produced different images on different days due to floating base image tags.

Produce a `Dockerfile` that containerizes the Go application. The application entry point is `cmd/server/main.go`, the module name in `go.mod` is `github.com/acme/inventory-service`, and the binary should be built with `go build -o /app/server ./cmd/server`. The service starts and accepts traffic when it responds to a GET request at `/healthz`.

Also produce a `.dockerignore` file appropriate for a Go project.

Place both files in the current directory.

## Expected Behavior

1. Use at least two `FROM` instructions — a builder stage (with Go toolchain) and a minimal runtime stage
2. Use a non-`golang` base image for the runtime stage (distroless, alpine, scratch, or similar)
3. Use a specific version tag in every `FROM` instruction — no `:latest` or tag-less references
4. Create a non-root user/group and include a `USER` instruction before the final `CMD`/`ENTRYPOINT`
5. Copy the compiled binary from the builder stage using `COPY --from=`
6. Set `WORKDIR` to an absolute path
7. Include an `EXPOSE 8080` instruction
8. Use exec-form (JSON array) syntax for `CMD` or `ENTRYPOINT`
9. Produce a `.dockerignore` file alongside the Dockerfile
10. Include entries for `.git`, `.env` (or `.env.*`), and at least one of `vendor/` or `*.exe` in `.dockerignore`

## Success Criteria

- **Multi-stage build present**: Dockerfile contains at least two FROM instructions (a builder stage and a runtime/final stage)
- **Build tools excluded from runtime**: The final stage does NOT use a golang base image — it uses a minimal image (distroless, alpine, scratch, or similar)
- **Pinned base image tags**: Every FROM instruction uses a specific version tag (e.g., golang:1.22-alpine) — NOT :latest or a tag-less reference
- **Non-root user**: Dockerfile creates a non-root user/group and includes a USER instruction before the final CMD/ENTRYPOINT pointing to that non-root user
- **Binary copied to runtime stage**: The compiled binary is copied from the builder stage into the runtime stage using COPY --from=
- **Absolute WORKDIR**: WORKDIR is set to an absolute path (starts with /) in the Dockerfile
- **EXPOSE port documented**: Dockerfile contains an EXPOSE 8080 (or the appropriate port) instruction
- **Exec-form CMD or ENTRYPOINT**: CMD or ENTRYPOINT uses JSON array syntax (e.g., CMD ["/app/server"]) rather than shell string form
- **.dockerignore created**: A .dockerignore file is present alongside the Dockerfile
- **.dockerignore excludes sensitive paths**: .dockerignore contains entries for at least .git, .env (or .env.*), and either node_modules/ or vendor/ or *.exe

## Failure Conditions

- Only one `FROM` instruction (no multi-stage build)
- Runtime stage uses a `golang` base image instead of a minimal image
- Any `FROM` instruction uses `:latest` or no tag
- No non-root user is created, or `USER` instruction is absent or placed after `CMD`
- Binary is not copied from the builder stage using `COPY --from=`
- `WORKDIR` is a relative path or absent
- No `EXPOSE` instruction for port 8080
- `CMD` or `ENTRYPOINT` uses shell string form instead of JSON array syntax
- No `.dockerignore` file is produced
- `.dockerignore` is missing entries for `.git` or `.env`
