# Scenario 03: Audit and Rewrite a Bloated Python Dockerfile

## User Prompt

A data team inherited a Dockerfile for a FastAPI application (`analytics-api`) from a contractor. After a recent Docker image size audit, the operations team found the image is 2.1 GB — nearly ten times what they expected. The image is also flagged by the security scanner for including build tools, stale apt lists, and cached pip packages in the final image.

The team needs the Dockerfile rewritten from scratch so it produces a small, clean production image. The application uses Python 3.11, the package list is in `requirements.txt`, and the server starts with `uvicorn app.main:app --host 0.0.0.0 --port 8000`.

Produce a new `Dockerfile` for the `analytics-api` Python/FastAPI application.

Also produce an appropriate `.dockerignore` for a Python project.

Place both files in the current directory.

The original (problematic) Dockerfile is:

```dockerfile
FROM python:3.11

RUN apt-get update
RUN apt-get install -y curl gcc build-essential
RUN pip install --upgrade pip

ADD . /app
WORKDIR /app

RUN pip install -r requirements.txt

CMD uvicorn app.main:app --host 0.0.0.0 --port 8000
```

## Expected Behavior

1. Chain `apt-get update`, `apt-get install`, and `rm -rf /var/lib/apt/lists/*` in a SINGLE `RUN` instruction using `&&`
2. Include the apt cache cleanup (`rm -rf /var/lib/apt/lists/*`) inside the SAME `RUN` as `apt-get install` — not a separate `RUN`
3. Use `COPY` for all file copy operations — no `ADD`
4. Use a specific Python version tag (e.g., `python:3.11-slim`) — not `:latest`
5. Use a minimal base image (`python:X.X-slim`, `python:X.X-alpine`, or distroless) rather than the full `python:3.11`
6. Create a non-root user and include a `USER` instruction before `CMD`/`ENTRYPOINT`
7. Copy `requirements.txt` BEFORE the pip install `RUN` instruction (layer caching)
8. Use exec-form (JSON array) syntax for `CMD`
9. Include a `.dockerignore` with at least one Python-specific entry (`__pycache__/`, `*.pyc`, or `.venv/`)

## Success Criteria

- **RUN commands combined**: apt-get update, apt-get install, and cache cleanup (rm -rf /var/lib/apt/lists/*) are chained in a single RUN instruction using &&, rather than separate RUN instructions
- **Cache cleaned in same layer**: The apt cache cleanup (rm -rf /var/lib/apt/lists/* or equivalent) is inside the SAME RUN instruction as apt-get install, not a separate RUN
- **COPY used not ADD**: Dockerfile uses COPY for all file copy operations; ADD is NOT present for plain file/directory copying
- **Pinned base image tag**: FROM uses a specific Python version tag (e.g., python:3.11-slim) and NOT :latest
- **Minimal base image**: Final image uses python:X.X-slim, python:X.X-alpine, or distroless rather than the full python:X.X image
- **Non-root user**: A non-root user is created and USER instruction is set before CMD/ENTRYPOINT
- **Dependency files copied first**: COPY of requirements.txt appears BEFORE the pip install RUN instruction
- **Exec-form CMD**: CMD uses JSON array syntax rather than bare shell string form
- **.dockerignore with Python entries**: .dockerignore is present and contains at least one Python-specific entry such as __pycache__/, *.pyc, or .venv/

## Failure Conditions

- `apt-get update`, `apt-get install`, and cache cleanup are split across multiple `RUN` instructions
- `rm -rf /var/lib/apt/lists/*` is a separate `RUN` instruction instead of being in the same layer as `apt-get install`
- `ADD` is used instead of `COPY`
- `FROM` uses `:latest` or the full non-slim `python:3.11` image
- Final image is not a minimal/slim variant
- No non-root user, or `USER` instruction is absent or placed after `CMD`
- `requirements.txt` is not copied before the pip install step
- `CMD` uses shell string form instead of JSON array syntax
- `.dockerignore` is missing Python-specific entries
