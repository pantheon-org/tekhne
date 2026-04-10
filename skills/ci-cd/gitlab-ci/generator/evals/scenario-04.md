# Scenario 04: Shared Template Refactor for a Multi-Service Pipeline

## User Prompt

An engineering team maintains a GitLab CI pipeline for three microservices — `auth-service`, `order-service`, and `notification-service` — all built with the same Node 20 stack. Their current `.gitlab-ci.yml` has grown to 200+ lines because each service has its own build, lint, and test job with an identical `before_script: [npm ci]` block and the same `image: node:20-alpine` line repeated nine times. A new joiner made a change to the setup command last week and only updated four of the nine jobs, causing inconsistent behaviour.

The team wants the pipeline refactored so that the shared setup is defined in one place and inherited by all service jobs. They also want to make sure the pipeline is fast: each service's tests should start as soon as that service's build finishes, not after all three builds complete.

## Output Specification

Produce a single `.gitlab-ci.yml` that covers all three services. Use a reusable template mechanism to avoid repeating the common setup. The file should be noticeably shorter than the 200-line original while preserving the build → test flow per service.

## Expected Behavior

1. Use `extends:` referencing a hidden job template (dot-prefixed) OR YAML anchors to share `before_script` and `image` across all service jobs
2. Define the `npm ci` setup command in ONE place (template, anchor, or `default:` block) so it is not repeated in each service job
3. Declare the Node image in ONE location rather than duplicating it across every job
4. Configure a `cache:` block for node_modules using a GitLab CI predefined variable (e.g., `$CI_COMMIT_REF_SLUG`) in the key
5. Use `needs:` on each service's test job to reference only its own service's build job for per-service DAG chains
6. Pin the shared image to a specific version, avoid `only:`/`except:`, and add `expire_in:` to any artifact paths

## Success Criteria

- **extends or YAML anchors used**: The YAML uses `extends:` referencing a hidden job template (dot-prefixed) OR YAML anchors (`&`/`<<: *`) to share common configuration
- **No duplicate before_script**: The common setup command (npm ci or equivalent) appears in ONE place (template/anchor/default), NOT repeated in each service job
- **Image declared once**: The Node image is declared in ONE location (default block, shared template, or anchor), not duplicated across every job
- **Cache configured**: A `cache:` block is present, configured for node_modules or equivalent dependency directory
- **Cache key uses CI variable**: The cache `key:` references a GitLab CI predefined variable (e.g. $CI_COMMIT_REF_SLUG) rather than a static string
- **needs for per-service DAG**: Each service's test job uses `needs:` to reference only its own service's build job
- **Image pinned**: The shared image tag does NOT use `:latest`
- **No only/except**: The YAML does NOT use `only:` or `except:`
- **expire_in on artifacts**: Any artifact block with `paths:` includes `expire_in:`
- **Kebab-case job names**: All job names use kebab-case

## Failure Conditions

- No `extends:` or YAML anchors are used — the `before_script` remains duplicated across each service job
- The `npm ci` setup command is repeated in each service job instead of defined once
- The Node image is declared in multiple jobs instead of in a single location
- No `cache:` block is present for node_modules
- Cache `key:` uses a static string rather than a GitLab CI predefined variable
- Any service's test job does not use `needs:` to reference its own service's build job
- The image uses `:latest` or is untagged
- The YAML contains `only:` or `except:` keywords
