# Scenario 07: Shared Template Refactor for a Multi-Service Pipeline

## User Prompt

An engineering team maintains a GitLab CI pipeline for three microservices — `auth-service`, `order-service`, and `notification-service` — all built with the same Node 20 stack. Their current `.gitlab-ci.yml` has grown to 200+ lines because each service has its own build, lint, and test job with an identical `before_script: [npm ci]` block and the same `image: node:20-alpine` line repeated nine times. A new joiner made a change to the setup command last week and only updated four of the nine jobs, causing inconsistent behaviour.

The team wants the pipeline refactored so that the shared setup is defined in one place and inherited by all service jobs. They also want to make sure the pipeline is fast: each service's tests should start as soon as that service's build finishes, not after all three builds complete.
