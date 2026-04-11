# Scenario 02: Monorepo Pipeline with Frontend and Backend Components

## User Prompt

A mid-sized SaaS company has a monorepo containing a React frontend and a Python backend service. Currently their `.gitlab-ci.yml` runs all jobs strictly in stage order, meaning the backend tests cannot start until the frontend build has finished — even though the two components are completely independent. As the codebase has grown, pipelines now take 18 minutes end-to-end on a fast runner. The engineering manager wants to bring this down to under 10 minutes without splitting the repository.

A separate concern: the pipeline has occasionally hung when a flaky integration test stalls an external database fixture, blocking the runner for over an hour. There is no mechanism to cancel the stalled pipeline when a developer pushes a follow-up fix to the same branch.

## Output Specification

Produce a `.gitlab-ci.yml` that builds and tests the frontend and backend components. The pipeline structure should allow independent component tests to start as soon as their own build step finishes, rather than waiting for all build jobs in the stage to complete. Jobs that do not need to run to completion before a developer pushes again should be cancelable by a new pipeline. Long-running jobs should have a safeguard so they cannot run indefinitely.

## Expected Behavior

1. Use `needs:` on test jobs to reference only their own component's build job, enabling DAG-based parallel execution
2. Decouple frontend and backend chains so frontend tests depend only on the frontend build and backend tests depend only on the backend build
3. Set `timeout:` on at least two jobs to prevent infinite hangs
4. Configure `retry:` on at least one job to handle transient failures
5. Mark build and test jobs as `interruptible: true` so they can be cancelled by a newer pipeline on the same branch
6. Pin all image references to specific versions and avoid `only:`/`except:`

## Success Criteria

- **needs used for DAG**: At least one test or downstream job uses a `needs:` list that references only its own component's build job (not all build jobs)
- **Frontend and backend chains decoupled**: Frontend test/deploy jobs reference frontend build in `needs:`, and backend test/deploy jobs reference backend build in `needs:` — not cross-component
- **timeout on jobs**: At least two jobs (or the default block) include a `timeout:` field with a value in minutes
- **retry configured**: At least one job (or the default block) includes a `retry:` configuration
- **interruptible: true on build/test jobs**: At least one build or test job has `interruptible: true` (or it is set in the `default:` block)
- **Image pinned**: All `image:` values specify a version other than `:latest`
- **No only/except**: The YAML does NOT use `only:` or `except:` keywords
- **expire_in on artifacts**: Any `artifacts:` block with `paths:` includes `expire_in:`
- **Kebab-case job names**: Job names use kebab-case
- **Stages declared**: A `stages:` key is present listing the pipeline stages

## Failure Conditions

- No `needs:` is used, leaving all jobs running in strict stage order
- Frontend and backend job chains are not decoupled (cross-component needs references)
- No `timeout:` is set on at least two jobs or in the default block
- No `retry:` configuration is present on any job
- No build or test job has `interruptible: true`
- Any `image:` uses `:latest` or is untagged
- The YAML contains `only:` or `except:` keywords
- Any `artifacts:` block with `paths:` lacks `expire_in:`
