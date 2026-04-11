# Scenario 03: Best Practices: Deprecated Keywords, Caching, Artifact Expiration, and Image Pinning

## User Prompt

A platform team is reviewing a `.gitlab-ci.yml` that was written before GitLab 14.x. The pipeline runs correctly today but the team has been warned it will break when GitLab completes the removal of deprecated keywords. Additionally, build times have increased 40% over the past quarter and the team suspects missing caches and inefficient artifact configuration.

Review the pipeline below for best practices violations. For each finding, explain the problem and its impact, and produce a fully improved version of the pipeline.

## Output Specification

Produce:
- A corrected `.gitlab-ci.yml` with all best practice improvements applied
- A `best-practices-report.md` listing each finding with: category (deprecation/performance/reliability), job name, problem description, and fix applied

## Input Files

The following file is provided as input.

=============== FILE: inputs/.gitlab-ci.yml ===============
stages:
  - install
  - build
  - test
  - deploy

install_deps:
  stage: install
  image: node:latest
  script:
    - npm ci
  artifacts:
    paths:
      - node_modules/

build_app:
  stage: build
  image: node:latest
  script:
    - npm run build
  artifacts:
    paths:
      - dist/
  only:
    - main
    - merge_requests

test_unit:
  stage: test
  image: node:latest
  script:
    - npm test
  except:
    - tags

test_e2e:
  stage: test
  image: cypress/base:latest
  script:
    - npx cypress run
  artifacts:
    paths:
      - cypress/screenshots/
      - cypress/videos/
  only:
    - main
    - merge_requests

deploy_production:
  stage: deploy
  image: alpine:latest
  script:
    - ./deploy.sh production
  only:
    - main

## Expected Behavior

1. Detect and flag `only:` usage in multiple jobs as deprecated in favor of `rules:`
2. Detect and flag `except:` usage in test_unit as deprecated
3. Flag the absence of a `cache:` configuration for node_modules as a performance problem
4. Flag missing `expire_in:` on artifact blocks as a reliability issue
5. Flag `node:latest`, `cypress/base:latest`, and `alpine:latest` as unpinned image tags
6. Migrate `only:`/`except:` to `rules:` in the corrected pipeline
7. Add a `cache:` block for node_modules keyed on package-lock.json
8. Add `expire_in:` to artifact blocks that were missing it
9. Assign a category (deprecation, performance, or reliability) to each finding

## Success Criteria

- **Deprecated only: keyword detected**: best-practices-report.md (or equivalent) flags the use of `only:` in multiple jobs as deprecated in favour of `rules:`
- **Deprecated except: keyword detected**: best-practices-report.md flags the use of `except:` in test_unit as deprecated
- **Missing cache for node_modules detected**: best-practices-report.md flags the absence of a cache configuration for node_modules in the install_deps or downstream jobs
- **Missing artifact expiration detected**: best-practices-report.md flags that one or more artifact blocks are missing an `expire_in` setting
- **Unpinned :latest image tags detected**: best-practices-report.md flags the `node:latest`, `cypress/base:latest`, and `alpine:latest` image references as unpinned
- **only:/except: migrated to rules: in output**: In the corrected .gitlab-ci.yml, at least the jobs that previously used `only:` or `except:` now use `rules:` syntax instead
- **cache: added for node_modules in output**: In the corrected .gitlab-ci.yml, at least one job (install_deps or a shared default) includes a `cache:` configuration for node_modules keyed on package-lock.json
- **expire_in added to artifact blocks in output**: In the corrected .gitlab-ci.yml, artifact blocks that were missing expire_in now have an expiration value set
- **Category assigned to each finding**: best-practices-report.md assigns a category (deprecation, performance, or reliability) to each finding

## Failure Conditions

- `only:` usage in jobs is not flagged as deprecated
- `except:` usage in test_unit is not flagged as deprecated
- Missing `cache:` for node_modules is not identified as a performance concern
- Missing `expire_in` on artifact blocks is not flagged
- `:latest` image tags are not identified as unpinned
- Corrected pipeline retains `only:` or `except:` keywords instead of migrating to `rules:`
- Corrected pipeline has no `cache:` block for node_modules
- Corrected pipeline has artifact blocks without `expire_in`
- Report does not assign a category (deprecation/performance/reliability) to each finding
