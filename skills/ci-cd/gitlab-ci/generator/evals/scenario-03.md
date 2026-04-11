# Scenario 03: Multi-Environment Deployment Pipeline

## User Prompt

A DevOps team at a fintech company manages three environments: `development`, `staging`, and `production`. They have experienced two incidents in the past year where simultaneous deployments to staging from different branches caused a partially deployed state that took hours to diagnose. Production deployments have no formal approval gate — a developer once accidentally triggered one at 2 AM and it resulted in a 45-minute outage.

The team wants a pipeline that deploys to development automatically on every push to `develop`, requires a manual click to promote to staging, and requires a separate manual approval for production that can only be triggered from `main`. The pipeline must also track which version is live in each environment so the team can see deployment history in the GitLab UI.

## Output Specification

Produce a `.gitlab-ci.yml` for this deployment pipeline. You may use placeholder `script:` blocks (e.g. `echo "deploying"`) for the actual deployment commands. The pipeline structure, conditions, and safety mechanisms are what matter.

## Expected Behavior

1. Declare an `environment:` block on each deployment job with at least a `name:` field so GitLab tracks deployment history
2. Use lowercase values for environment names (development, staging, production)
3. Add `resource_group:` to staging and production deployment jobs to prevent concurrent deployments
4. Set `when: manual` on the production deployment job (or use a `rules:` condition with `when: manual`)
5. Restrict the production deployment job to the `main` branch via a `rules:` condition
6. Avoid `only:` and `except:` keywords, pin all images, avoid hardcoded secrets, and use kebab-case job names

## Success Criteria

- **environment block on deploy jobs**: Each deployment job (development, staging, production) includes an `environment:` key with at least a `name:` field
- **environment name lowercase**: Environment `name:` values are lowercase (development, staging, production — not Development, PRODUCTION)
- **resource_group on deploy jobs**: At least the staging and production deployment jobs include a `resource_group:` key to prevent concurrent runs
- **when: manual for production**: The production deployment job has `when: manual` (or equivalent in a rules condition)
- **Production restricted to main branch**: The production deployment job has a `rules:` condition that limits it to runs on the `main` branch
- **No only/except**: The YAML does NOT use `only:` or `except:` keywords
- **Image pinned**: All `image:` values specify a version other than `:latest`
- **No hardcoded secrets**: The YAML does NOT contain hardcoded tokens, passwords, or API keys
- **Kebab-case job names**: Deployment job names use kebab-case (e.g. deploy-production, deploy-staging)
- **timeout on deploy jobs**: Deployment jobs include a `timeout:` field

## Failure Conditions

- Any deployment job lacks an `environment:` block
- Environment names are not lowercase (e.g., using `Development` or `PRODUCTION`)
- Staging or production deployment jobs lack `resource_group:` to prevent concurrent runs
- The production deployment job does not have `when: manual`
- The production deployment job has no `rules:` condition restricting it to the main branch
- The YAML contains `only:` or `except:` keywords
- Any `image:` uses `:latest` or is untagged
- Any hardcoded token, password, or API key appears as a literal string value
