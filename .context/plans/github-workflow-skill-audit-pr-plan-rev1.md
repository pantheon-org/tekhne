# Revision Plan: GitHub Workflow for Automated Skill Auditing

Based on critical review of `.context/features/github-workflow-skill-audit-pr.md`

## Priority: CRITICAL

### 1. Remove AI Agent Requirement or Clarify Purpose

- **Issue**: Plan repeatedly mentions "invoke AI agent" (lines 9, 12, 39, 99, 153) but `evaluate.sh` is a deterministic shell script
- **Action**: Remove AI agent references unless there's an additional AI-driven aspect not in scope
- **Reference**: `skills/skill-quality-auditor/scripts/evaluate.sh` - runs without AI

### 2. Add Skill Detection Logic

- **Issue**: Plan says "audit only the specific skill" but doesn't detail detection
- **Action**: Add step to parse PR diff and extract skill names from changed files
- **Pattern to use** (from `scripts-ci-integration.md:55-66`):

```bash
for file in $(git diff --name-only origin/main | grep "skills/.*/SKILL.md"); do
  skill_name=$(basename $(dirname $file))
```

### 3. Resolve Threshold Inconsistency

- **Issue**:
  - Plan line 20: "score < 84" (C-pass)
  - `scripts-ci-integration.md:63`: "score -lt 90" (B/A threshold)
  - `evaluate.sh:219`: Passing C-grade is 84
- **Action**: Choose threshold (84 vs 90) and document reasoning in plan

## Priority: HIGH

### 4. Reference Existing CI Integration Patterns

- **Issue**: Plan reinvents workflow that already exists in `scripts-ci-integration.md`
- **Action**:
  - Use existing quality gate workflow as foundation (`scripts-ci-integration.md:23-94`)
  - Add PR-triggered logic and issue creation on top

### 5. Add Schema Validation for Remediation Plans

- **Issue**: Plan references `.context/plans/` but doesn't validate against schema
- **Action**: Add step to run:

```bash
sh skills/skill-quality-auditor/scripts/validate-remediation-plan.sh <plan-file>
```

- **Reference**: `skills/skill-quality-auditor/scripts/validate-remediation-plan.sh`

### 6. Add JSON Parsing for `evaluate.sh --json`

- **Issue**: Plan example doesn't handle JSON output
- **Action**: Use `jq` to parse:

```bash
score=$(sh evaluate.sh "$skill_name" --json | jq '.total')
grade=$(sh evaluate.sh "$skill_name" --json | jq -r '.grade')
```

## Priority: MEDIUM

### 7. Handle Multi-Skill PRs

- **Issue**: Plan audits "the specific skill" but doesn't address multiple skills changed
- **Action**: Loop through all changed skills, create issue for each failing skill

### 8. Choose Artifact vs Commit Storage

- **Issue**: Line 98 asks question without answering
- **Recommendation**: Artifacts (simpler, no commit noise, retention-days control)

### 9. Add Security: Permissions Block

- **Issue**: Workflow should declare minimal permissions
- **Action**:

```yaml
permissions:
  contents: read
  pull-requests: write
  issues: write
```

### 10. Add `workflow_dispatch` for Manual Runs

- **Issue**: No manual trigger for testing
- **Action**: Add:

```yaml
workflow_dispatch:
```

## Priority: LOW

### 11. Add Slack Notification Option

- **Reference**: `scripts-ci-integration.md:278-290`

### 12. Add Error Handling

- **Issue**: Example YAML has no `set -e`, no error handling
- **Action**: Add proper shell error handling in workflow steps

### 13. Document Forked PR Behavior

- **Issue**: Line 100 asks about `pull_request_target` without answering
- **Recommendation**: Use `pull_request` (not `pull_request_target`) for security; forked PRs still trigger but with read-only access

---

## Unanswered Questions from Original Plan

1. ~~Should audit reports be committed or artifacts?~~ - Recommend artifacts
2. ~~Which AI agent?~~ - Remove - no AI needed, shell script sufficient
3. ~~`pull_request_target` for forks?~~ - Recommend standard `pull_request` for security

---

## Recommended Implementation Order

1. Copy `scripts-ci-integration.md:23-94` as base workflow
2. Add PR-triggered filtering (`on: pull_request: paths: skills/**`)
3. Add issue creation logic (score < threshold)
4. Add skill detection loop (git diff parsing)
5. Test with sample PR
6. Add schema validation step
7. Add Slack notifications (optional)

---

## Files to Modify

- **Create**: `.github/workflows/skill-audit-pr.yaml`
- **Reference**:
  - `skills/skill-quality-auditor/scripts/evaluate.sh`
  - `skills/skill-quality-auditor/scripts/audit-skills.sh`
  - `skills/skill-quality-auditor/references/scripts-ci-integration.md`
  - `skills/skill-quality-auditor/scripts/validate-remediation-plan.sh`
