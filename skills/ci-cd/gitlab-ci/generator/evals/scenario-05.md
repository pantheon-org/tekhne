# Scenario 05: Docker Image Build and Push Pipeline

## User Prompt

A platform team is containerising a Go application and needs a GitLab CI pipeline to build, tag, and push Docker images to the GitLab Container Registry on every merge to `main`. The current draft pipeline a junior engineer wrote hard-codes the registry URL and uses an `image: docker:latest` with credentials typed directly into the YAML. The security team has blocked the merge.

The team also discovered that two simultaneous pushes to `main` (from back-to-back merges in a busy sprint) caused a race condition where both pipeline runs pushed an image with the same tag, and it was impossible to trace which pipeline produced the final image in production.

## Output Specification

Produce a `.gitlab-ci.yml` for this Docker build pipeline. The pipeline should build the image, tag it uniquely per pipeline run so the exact commit can always be identified, and push it to the registry. Use Docker-in-Docker or an equivalent approach available in GitLab CI. Do not include any literal credential values in the YAML.

## Expected Behavior

1. Tag the Docker image with `$CI_COMMIT_SHORT_SHA` (or `$CI_COMMIT_SHA`) so every image is traceable to a specific commit
2. Use GitLab's predefined registry variables (`$CI_REGISTRY`, `$CI_REGISTRY_USER`, `$CI_REGISTRY_PASSWORD`) for Docker login — no hardcoded credentials
3. Use `$CI_REGISTRY_IMAGE` in the image name rather than a hardcoded registry path
4. Pin the Docker `image:` to a specific version (e.g., `docker:24-dind`) — not `docker:latest`
5. Use `rules:` to restrict the build/push job to the `main` branch; avoid `only:`/`except:`
6. Add a `timeout:` and `retry:` on the build job, and add `expire_in:` to any artifact blocks with `paths:`

## Success Criteria

- **CI_COMMIT_SHORT_SHA for tag**: The image tag uses `$CI_COMMIT_SHORT_SHA` (or `$CI_COMMIT_SHA`) to produce a unique, commit-traceable tag
- **No hardcoded credentials**: The YAML does NOT contain literal registry URLs with embedded usernames/passwords or hardcoded token strings
- **Registry variables used for auth**: Docker login uses GitLab's predefined registry variables: $CI_REGISTRY, $CI_REGISTRY_USER, $CI_REGISTRY_PASSWORD
- **Docker image pinned**: The `image:` for the Docker build job is pinned to a specific version (e.g. docker:24-dind or docker:24.0) — NOT docker:latest
- **No only/except**: The YAML does NOT use `only:` or `except:`
- **rules restricts to main**: The build/push job has a `rules:` condition limiting it to runs on the `main` branch (or equivalent)
- **CI_REGISTRY_IMAGE used**: The image name references `$CI_REGISTRY_IMAGE` (or a variable derived from it) rather than a hardcoded registry path
- **timeout set**: The Docker build job includes a `timeout:` field
- **retry configured**: The build or push job has a `retry:` block to handle transient registry failures
- **expire_in on artifacts if present**: If any `artifacts:` block with `paths:` is present, it includes `expire_in:`

## Failure Conditions

- Image tag does not use `$CI_COMMIT_SHORT_SHA` or `$CI_COMMIT_SHA` for traceability
- Any hardcoded registry URL with credentials or literal token string appears in the YAML
- Docker login does not use GitLab's predefined registry variables ($CI_REGISTRY, $CI_REGISTRY_USER, $CI_REGISTRY_PASSWORD)
- The Docker `image:` uses `:latest` or is otherwise unspecified
- The YAML contains `only:` or `except:` keywords
- The build/push job has no `rules:` condition restricting it to the main branch
- A hardcoded registry path is used instead of `$CI_REGISTRY_IMAGE`
