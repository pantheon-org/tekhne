# Scenario 08: Docker Image Build and Push Pipeline

## User Prompt

A platform team is containerising a Go application and needs a GitLab CI pipeline to build, tag, and push Docker images to the GitLab Container Registry on every merge to `main`. The current draft pipeline a junior engineer wrote hard-codes the registry URL and uses an `image: docker:latest` with credentials typed directly into the YAML. The security team has blocked the merge.

The team also discovered that two simultaneous pushes to `main` (from back-to-back merges in a busy sprint) caused a race condition where both pipeline runs pushed an image with the same tag, and it was impossible to trace which pipeline produced the final image in production.
