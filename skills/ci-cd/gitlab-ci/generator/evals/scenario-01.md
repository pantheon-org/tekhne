# Scenario 01: Node.js API CI Pipeline

## User Prompt

A small product team is launching a new Node.js REST API and needs a GitLab CI pipeline wired up before they merge their first feature. They previously worked at a company that used `only: master` in their pipelines and they have copy-pasted that pattern into a draft `.gitlab-ci.yml` they shared. A DevOps consultant has flagged that this approach is outdated and will cause subtle problems as the team adopts GitLab's newer features, but the team does not know the modern equivalent.

The pipeline should run on every push, but the deploy step should only trigger on the `main` branch. The team uses Node 20 and produces a `dist/` build folder. Tests generate a `coverage/` directory that should be available for download for 3 days, while the build artifact only needs to exist for the next stage and can be discarded sooner.

## Output Specification

Produce a `.gitlab-ci.yml` file for this pipeline. The pipeline should have at minimum: an install/build stage, a test stage, and a deploy stage. Do not include any hardcoded credentials or tokens in the YAML.

## Expected Behavior

1. Use `rules:` with `if:` conditions instead of the deprecated `only:` and `except:` keywords
2. Pin all Docker image references to a specific version tag (not `:latest`)
3. Set `expire_in:` on every `artifacts:` block that contains `paths:`
4. Restrict the deploy job to the `main` branch via a `rules:` condition referencing `$CI_COMMIT_BRANCH`
5. Use kebab-case for job names and UPPER_SNAKE_CASE for pipeline variables
6. Set a `timeout:` on at least one job or in the `default:` block

## Success Criteria

- **No only/except used**: The generated YAML does NOT contain `only:` or `except:` keys anywhere
- **rules used for branch conditions**: The deploy (or equivalent) job uses a `rules:` block with an `if:` condition referencing `$CI_COMMIT_BRANCH`
- **Image pinned to version**: All `image:` declarations (default or per-job) specify a version tag that is NOT `:latest` (e.g. node:20-alpine, node:20.11-alpine3.19)
- **expire_in on all artifacts**: Every `artifacts:` block that contains `paths:` also contains an `expire_in:` field
- **No hardcoded secrets**: The YAML does NOT contain any hardcoded token, password, or API key strings — credentials referenced only via CI variables (e.g. $DEPLOY_TOKEN)
- **Kebab-case job names**: All job names use kebab-case (e.g. build-app, run-tests, deploy-staging) not snake_case or PascalCase
- **UPPER_SNAKE_CASE variables**: Any pipeline-level `variables:` blocks use UPPER_SNAKE_CASE keys
- **timeout set on jobs**: At least one job or the `default:` block includes a `timeout:` field
- **Specific artifact paths**: Artifact `paths:` list specific directories (e.g. dist/, coverage/) rather than a wildcard like ./**
- **Stages declared**: The pipeline declares a `stages:` list with at least build, test, and deploy (or equivalents)

## Failure Conditions

- The generated YAML contains `only:` or `except:` keywords
- Deploy job has no `rules:` condition restricting it to the main branch
- Any `image:` uses `:latest` or is untagged
- Any `artifacts:` block with `paths:` lacks `expire_in:`
- Any hardcoded token, password, or API key appears as a literal string value
- Job names use snake_case or PascalCase instead of kebab-case
- Pipeline variable keys are not UPPER_SNAKE_CASE
- No `timeout:` is set on any job or in the default block
