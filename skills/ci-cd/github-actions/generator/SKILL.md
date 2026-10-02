---
name: github-actions-generator
description: Generates production-ready GitHub Actions workflows, custom actions, and CI/CD configurations following security and performance standards. Creates CI/CD pipelines, test workflows, deployment configurations, matrix builds, caching strategies, composite actions, Docker actions, JavaScript actions, and reusable workflows. Use when creating or scaffolding GHA resources, writing .github/workflows YAML files, setting up build automation, implementing deployment pipelines, adding security scanning, or building reusable actions — including triggers like 'create a workflow', 'build a pipeline', 'add CI', 'set up GHA', or 'generate a YAML workflow'.
---

# GitHub Actions Generator

Generate production-ready GitHub Actions workflows and custom actions following current best practices, security standards, and naming conventions. All generated resources are automatically validated using the devops-skills:github-actions-validator skill.

## Philosophy

- Treat the supply chain as hostile: pin actions to a commit SHA and grant the least permissions that work.
- Prefer fast feedback: use caching, concurrency cancellation and sensible timeouts.
- Validate every complete resource before presenting it, and fix before delivery rather than after.

## When to Use

- Creating or scaffolding workflows under `.github/workflows` for CI, testing or deployment.
- Building composite, Docker or JavaScript actions, or reusable workflows.
- Adding security scanning such as dependency review, CodeQL or SBOM attestation.
- Setting up matrix builds, caching, environments or job summaries.

## When Not to Use

- Validating or debugging an existing workflow without generating anything: use the validator skill alone.
- Targeting other CI systems such as GitLab CI, Azure Pipelines or Jenkins.
- Administering repository settings, runners or organisation-level secrets.
- Writing application code that the workflow merely builds or tests.

## Core Capabilities

### 1. Generate Workflows

**Triggers:** "Create a workflow for...", "Build a CI/CD pipeline..."

**Process:**
1. Understand requirements (triggers, runners, dependencies)
2. Reference `references/best-practices.md` for patterns
3. Reference `references/common-actions.md` for action versions
4. Generate workflow with:
   - Semantic names, pinned actions (SHA), proper permissions
   - Concurrency controls, caching, matrix strategies
5. **Validate** with devops-skills:github-actions-validator skill
6. Fix issues and re-validate if needed

**Minimal Example:**
```yaml
name: CI Pipeline

on:
  push:
    branches: [main]
  pull_request:

permissions:
  contents: read

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@08c6903cd8c0fde910a37f88322edcfb5dd907a8 # v5.0.0
      - uses: actions/setup-node@2028fbc5c25fe9cf00d9f06a71cc4710d4507903 # v6.0.0
        with:
          node-version: '20'
          cache: 'npm'
      - run: npm ci
      - run: npm test
```

### 2. Generate Custom Actions

**Triggers:** "Create a composite action...", "Build a Docker action...", "Create a JavaScript action..."

**Types:**
- **Composite:** Combine multiple steps → Fast startup
- **Docker:** Custom environment/tools → Isolated
- **JavaScript:** API access, complex logic → Fastest

**Process:**
1. Use templates from `assets/templates/action/`
2. Follow structure in `references/custom-actions.md`
3. Include branding, inputs/outputs, documentation
4. **Validate** with devops-skills:github-actions-validator skill

See `references/custom-actions.md` for:
- Action metadata and branding
- Directory structure patterns
- Versioning and release workflows

### 3. Generate Reusable Workflows

**Triggers:** "Create a reusable workflow...", "Make this workflow callable..."

**Key Elements:**
- `workflow_call` trigger with typed inputs
- Explicit secrets (avoid `secrets: inherit`)
- Outputs mapped from job outputs
- Minimal permissions

```yaml
on:
  workflow_call:
    inputs:
      environment:
        required: true
        type: string
    secrets:
      deploy-token:
        required: true
    outputs:
      result:
        value: ${{ jobs.build.outputs.result }}
```

See `references/advanced-triggers.md` for complete patterns.

### 4. Generate Security Workflows

**Triggers:** "Add security scanning...", "Add dependency review...", "Generate SBOM..."

**Components:**
- **Dependency Review:** `actions/dependency-review-action@v4`
- **SBOM Attestations:** `actions/attest-sbom@v2`
- **CodeQL Analysis:** `github/codeql-action`

**Required Permissions:**
```yaml
permissions:
  contents: read
  security-events: write  # For CodeQL
  id-token: write         # For attestations
  attestations: write     # For attestations
```

See `references/best-practices.md` section on security.

### 5. Modern Features

**Triggers:** "Add job summaries...", "Use environments...", "Run in container..."

See `references/modern-features.md` for:
- Job summaries (`$GITHUB_STEP_SUMMARY`)
- Deployment environments with approvals
- Container jobs with services
- Workflow annotations

### 6. Public Action Documentation

When using public actions:

1. **Search for documentation:**
   ```
   "[owner/repo] [version] github action documentation"
   ```

2. **Or use Context7 MCP:**
   - `mcp__context7__resolve-library-id` to find action
   - `mcp__context7__get-library-docs` for documentation

3. **Pin to SHA with version comment:**
   ```yaml
   - uses: actions/checkout@08c6903cd8c0fde910a37f88322edcfb5dd907a8 # v5.0.0
   ```

See `references/common-actions.md` for pre-verified action versions.

---

## Validation Workflow

**CRITICAL:** Every generated resource MUST be validated.

1. Generate workflow/action file
2. Invoke `devops-skills:github-actions-validator` skill
3. If errors: fix and re-validate
4. If success: present with usage instructions

**Skip validation only for:**
- Partial code snippets
- Documentation examples
- User explicitly requests skip

---

## Mandatory Standards

All generated resources must follow:

| Standard | Implementation |
|----------|---------------|
| **Security** | Pin to SHA, minimal permissions, mask secrets |
| **Performance** | Caching, concurrency, shallow checkout |
| **Naming** | Descriptive names, lowercase-hyphen files |
| **Error Handling** | Timeouts, cleanup with `if: always()` |

See `references/best-practices.md` for complete guidelines.

---

## Common Patterns

### Matrix Testing

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest]
    node: [18, 20, 22]
  fail-fast: false
```

### Conditional Deployment

```yaml
deploy:
  if: github.event_name == 'push' && github.ref == 'refs/heads/main'
```

### Artifact Sharing

```yaml
# Upload
- uses: actions/upload-artifact@v4
  with:
    name: build-${{ github.sha }}
    path: dist/

# Download (in dependent job)
- uses: actions/download-artifact@v4
  with:
    name: build-${{ github.sha }}
```

---

## Workflow Summary

1. **Understand** requirements
2. **Reference** appropriate docs
3. **Generate** with standards
4. **Search** for public action docs (if needed)
5. **Validate** with devops-skills:github-actions-validator
6. **Fix** any errors
7. **Present** validated result

## Anti-Patterns

### NEVER use `@latest` or branch-based action references

**WHY:** Mutable references allow the action to change silently between runs, enabling supply chain attacks where a compromised upstream injects malicious code into your workflow.

**BAD:** `uses: actions/checkout@main`

**GOOD:** `uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2`

### NEVER use `secrets: inherit` in reusable workflows without justification

**WHY:** `secrets: inherit` exposes every secret from the caller to the callee even when only one is needed, violating the principle of least privilege and widening the blast radius of a compromised workflow.

**BAD:** `secrets: inherit`

**GOOD:** Declare only required secrets explicitly, for example `secrets: deploy-token: required: true`.

### NEVER omit `permissions:` at the job or workflow level

**WHY:** `GITHUB_TOKEN` defaults to write permissions in older repositories. Omitting `permissions:` means every job can push commits, create releases, or modify issues unintentionally.

**BAD:** A workflow with no `permissions:` block at all.

**GOOD:** Set `permissions: contents: read` as the workflow default and override per-job only where write access is genuinely required.

### NEVER set `fail-fast: false` by default in matrix builds

**WHY:** `fail-fast: false` causes the entire matrix to keep running after the first failure, wasting runner minutes and delaying feedback. It should be an intentional choice, not a default.

**BAD:** `strategy: fail-fast: false` added to every matrix without explanation.

**GOOD:** Omit `fail-fast` to use the default `true`, or add an explicit comment explaining why all combinations must complete.

### NEVER use `pull_request_target` with `actions/checkout` checking out PR code

**WHY:** `pull_request_target` runs with write permissions and access to secrets. Combining it with a checkout of untrusted PR code enables attackers to exfiltrate secrets or tamper with your repository.

**BAD:** `on: pull_request_target` combined with `uses: actions/checkout@... with: ref: ${{ github.event.pull_request.head.sha }}`

**GOOD:** Use `pull_request` for untrusted code, or carefully scope and audit any `pull_request_target` workflow before adding a checkout step.

### NEVER pin an action to a SHA without a version comment

**WHY:** a bare SHA tells reviewers nothing about which release it is, so upgrades and audits become guesswork.

**BAD:** `uses: actions/setup-node@2028fbc5c25fe9cf00d9f06a71cc4710d4507903`

**GOOD:** `uses: actions/setup-node@2028fbc5c25fe9cf00d9f06a71cc4710d4507903 # v6.0.0`

### NEVER guess the version of a public action

**WHY:** invented versions or SHAs fail at run time or pin to the wrong release.

**BAD:** typing a SHA from memory.

**GOOD:** take the version from `references/common-actions.md`, or look up the action's documentation first.

### NEVER ship a workflow without concurrency control and timeouts

**WHY:** superseded runs keep consuming runner minutes and a hung job can run until the platform limit.

**BAD:** a workflow with no `concurrency:` block and no `timeout-minutes:`.

**GOOD:** add `concurrency` with `cancel-in-progress: true` and a `timeout-minutes` on each job.

### NEVER deliver a generated workflow or action without validating it

**WHY:** unvalidated YAML ships syntax and security errors to the user.

**BAD:** presenting a new workflow straight after writing it.

**GOOD:** invoke the github-actions-validator skill, fix errors, and re-validate before presenting.

### NEVER publish a custom action without inputs, outputs and documentation

**WHY:** consumers cannot use an action whose interface is undocumented, and branding and metadata are part of the contract.

**BAD:** an `action.yml` with no inputs, outputs or description.

**GOOD:** include branding, typed inputs and outputs, and documentation, following `references/custom-actions.md`.

## References

| Topic | Reference | When to Use |
| --- | --- | --- |
| Best practices | [references/best-practices.md](references/best-practices.md) | Every workflow: security, performance, patterns |
| Public actions | [references/common-actions.md](references/common-actions.md) | Action versions, inputs and outputs |
| Expressions | [references/expressions-and-contexts.md](references/expressions-and-contexts.md) | Complex conditionals and `${{ }}` syntax |
| Advanced triggers | [references/advanced-triggers.md](references/advanced-triggers.md) | workflow_run, dispatch, ChatOps and reusable workflows |
| Custom actions | [references/custom-actions.md](references/custom-actions.md) | Metadata, structure and versioning |
| Modern features | [references/modern-features.md](references/modern-features.md) | Summaries, environments and container jobs |
| Basic workflow template | [assets/templates/workflow/basic_workflow.yml](assets/templates/workflow/basic_workflow.yml) | Starting a new workflow |
| Reusable workflow template | [assets/templates/workflow/reusable_workflow.yml](assets/templates/workflow/reusable_workflow.yml) | Starting a callable workflow |
| Composite action template | [assets/templates/action/composite/action.yml](assets/templates/action/composite/action.yml) | Combining steps into one action |
| Docker action template | [assets/templates/action/docker/action.yml](assets/templates/action/docker/action.yml) | Custom environment or tools |
| JavaScript action template | [assets/templates/action/javascript/action.yml](assets/templates/action/javascript/action.yml) | API access and complex logic |
