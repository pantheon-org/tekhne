# Docker Image Build and Push Pipeline

## Problem/Feature Description

A platform team is containerising a Go application and needs a GitLab CI pipeline to build, tag, and push Docker images to the GitLab Container Registry on every merge to `main`. The current draft pipeline a junior engineer wrote hard-codes the registry URL and uses an `image: docker:latest` with credentials typed directly into the YAML. The security team has blocked the merge.

The team also discovered that two simultaneous pushes to `main` (from back-to-back merges in a busy sprint) caused a race condition where both pipeline runs pushed an image with the same tag, and it was impossible to trace which pipeline produced the final image in production.

## Output Specification

Produce a `.gitlab-ci.yml` for this Docker build pipeline. The pipeline should build the image, tag it uniquely per pipeline run so the exact commit can always be identified, and push it to the registry. Use Docker-in-Docker or an equivalent approach available in GitLab CI. Do not include any literal credential values in the YAML.
