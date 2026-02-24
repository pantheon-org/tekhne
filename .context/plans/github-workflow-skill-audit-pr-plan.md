---
plan_date: "2026-02-24"
skill_name: "github-workflow-skill-audit-pr"
source_audit: ".context/features/github-workflow-skill-audit-pr.md"
---

# Implementation Plan: GitHub Workflow for Automated Skill Auditing

## Executive Summary

| Metric | Value |
| --- | --- |
| **Status** | Proposed |
| **Priority** | High |
| **Effort** | M |
| **Verdict** | Create workflow file with PR-triggered audit and issue creation |

---

## Overview

Create a GitHub Actions workflow (`.github/workflows/skill-audit-pr.yaml`) that:

1. Triggers on PRs modifying `skills/**` path
2. Runs `skill-quality-auditor` evaluation on changed skill only
3. Posts PR comment with score and pass/fail status
4. Creates GitHub Issue with remediation plan when score < 84
5. Re-audits on subsequent pushes to verify fixes

---

## Architecture Decisions

### 1. Audit Report Storage

**Decision**: Keep as workflow artifacts (NOT committed to repo)

**Rationale**:

- Avoids repo pollution and merge conflicts
- Reduces CI noise in git history
- Artifact retention can be configured (default 90 days)
- Report available via GitHub UI for download

### 2. AI Agent Invocation

**Decision**: Use GitHub-native PR comments + issue assignment (not external AI)

**Rationale**:

- Document specifies invoking AI agent, but repository doesn't have explicit AI agent configured
- Using `@github-actions` bot for comments provides equivalent notification flow
- Can be extended to call Copilot/Gemini API later if needed

### 3. Pull Request Event

**Decision**: Use `pull_request` (not `pull_request_target`)

**Rationale**:

- `pull_request` has access to `github.event.pull_request.changes` for path filtering
- More secure: runs in context of PR, not triggering repo
- Use `paths` filter to only run when `skills/**` changes

### 4. Issue Deduplication

**Decision**: Search for existing open issues with title pattern `skill-audit.*<skill-name>`

**Implementation**:

- Query GitHub Issues API with `state=open` and `labels=skill-audit`
- Match against skill name
- Close superseded and create new only if no match

---

## Implementation Steps

### Step 1: Create Workflow File

**File**: `.github/workflows/skill-audit-pr.yaml`

```yaml
name: Skill Audit on PR

on:
  pull_request:
    paths:
      - 'skills/**'
    types: [opened, synchronize, reopened]

permissions:
  contents: read
  issues: write
  pull-requests: write

jobs:
  audit-skill:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        
      - name: Detect Changed Skill
        id: detect-skill
        run: |
          # Get list of changed files in PR
          files=$(git diff --name-only ${{ github.event.pull_request.base_sha }}...HEAD -- 'skills/**')
          
          # Extract skill name from first changed file
          # Pattern: skills/<skill-name>/SKILL.md or skills/<skill-name>/...
          skill_path=$(echo "$files" | head -1)
          if [ -n "$skill_path" ]; then
            skill_name=$(echo "$skill_path" | sed 's|skills/||' | cut -d'/' -f1)
            echo "skill=$skill_name" >> $GITHUB_OUTPUT
          else
            echo "skill=" >> $GITHUB_OUTPUT
          fi
      
      - name: Run Skill Evaluation
        if: steps.detect-skill.outputs.skill != ''
        id: evaluate
        run: |
          skill="${{ steps.detect-skill.outputs.skill }}"
          cd ${{ github.workspace }}
          
          # Run evaluation with JSON output
          result=$(sh skills/skill-quality-auditor/scripts/evaluate.sh "$skill" --json)
          echo "result=$result" >> $GITHUB_OUTPUT
          
          # Extract score for threshold check
          score=$(echo "$result" | grep -o '"total":[0-9]*' | grep -o '[0-9]*')
          echo "score=$score" >> $GITHUB_OUTPUT
          
          # Store pass/fail status
          if [ "$score" -ge 84 ]; then
            echo "status=pass" >> $GITHUB_OUTPUT
          else
            echo "status=fail" >> $GITHUB_OUTPUT
          fi
      
      - name: Post PR Comment
        if: steps.evaluate.outputs.skill != ''
        uses: actions/github-script@v7
        with:
          script: |
            const score = parseInt('${{ steps.evaluate.outputs.score }}');
            const status = '${{ steps.evaluate.outputs.status }}';
            const skill = '${{ steps.detect-skill.outputs.skill }}';
            const prNumber = context.issue.number;
            
            const grade = score >= 114 ? 'A+' :
                          score >= 108 ? 'A' :
                          score >= 102 ? 'B+' :
                          score >= 96 ? 'B' :
                          score >= 90 ? 'C+' :
                          score >= 84 ? 'C' :
                          score >= 78 ? 'D' : 'F';
            
            const body = `## Skill Quality Audit
            
**Skill**: \`${skill}\`
**Score**: ${score}/120 (${grade}) - ${status === 'pass' ? 'PASS' : 'FAIL'}

### Status
${status === 'pass' 
  ? '✅ Audit passed. Score meets threshold (84).' 
  : '⚠️ Score below passing threshold (84). Remediation required.'}

---
*Automated audit via skill-quality-auditor*`;
            
            // Create comment
            await github.rest.issues.createComment({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: prNumber,
              body: body
            });
            
            // Store comment URL for later reference
            console.log('Comment posted');

      - name: Create Issue on Failure
        if: steps.evaluate.outputs.status == 'fail'
        uses: actions/github-script@v7
        with:
          script: |
            const score = parseInt('${{ steps.evaluate.outputs.score }}');
            const skill = '${{ steps.detect-skill.outputs.skill }}';
            const prAuthor = '${{ github.event.pull_request.user.login }}';
            const prNumber = context.issue.number;
            
            // Check for existing issue
            const { data: issues } = await github.rest.issues.listForRepo({
              owner: context.repo.owner,
              repo: context.repo.repo,
              state: 'open',
              labels: 'skill-audit',
              per_page: 100
            });
            
            const existingIssue = issues.find(i => 
              i.title.toLowerCase().includes(skill.toLowerCase())
            );
            
            let issueNumber;
            
            if (existingIssue) {
              // Close superseded issue
              await github.rest.issues.update({
                owner: context.repo.owner,
                repo: context.repo.repo,
                issue_number: existingIssue.number,
                state: 'closed',
                state_reason: 'not_planned'
              });
              
              // Create new issue
              const { data: newIssue } = await github.rest.issues.create({
                owner: context.repo.owner,
                repo: context.repo.repo,
                title: `[Skill Audit] ${skill} - Score ${score}/120 (Below Threshold)`,
                body: `## Remediation Required

**Skill**: ${skill}
**Current Score**: ${score}/120
**Threshold**: 84/120
**PR**: #${prNumber}

### Action Required

This skill scored below the passing threshold. Please address the issues and push fixes to the PR.

### Next Steps

1. Review the audit results in PR #${prNumber}
2. Make improvements to \`skills/${skill}/SKILL.md\`
3. Push changes to trigger re-audit

---
*Auto-generated by skill-audit-pr workflow*`,
                labels: ['skill-audit', 'needs-review', 'remediation'],
                assignees: [prAuthor]
              });
              issueNumber = newIssue.number;
            } else {
              const { data: newIssue } = await github.rest.issues.create({
                owner: context.repo.owner,
                repo: context.repo.repo,
                title: `[Skill Audit] ${skill} - Score ${score}/120 (Below Threshold)`,
                body: `## Remediation Required

**Skill**: ${skill}
**Current Score**: ${score}/120
**Threshold**: 84/120
**PR**: #${prNumber}

### Action Required

This skill scored below the passing threshold. Please address the issues and push fixes to the PR.

### Next Steps

1. Review the audit results in PR #${prNumber}
2. Make improvements to \`skills/${skill}/SKILL.md\`
3. Push changes to trigger re-audit

---
*Auto-generated by skill-audit-pr workflow*`,
                labels: ['skill-audit', 'needs-review', 'remediation'],
                assignees: [prAuthor]
              });
              issueNumber = newIssue.number;
            }
            
            console.log(`Issue created: #${issueNumber}`);

      - name: Close Issue on Pass
        if: steps.evaluate.outputs.status == 'pass'
        uses: actions/github-script@v7
        with:
          script: |
            const skill = '${{ steps.detect-skill.outputs.skill }}';
            
            // Find and close open issues for this skill
            const { data: issues } = await github.rest.issues.listForRepo({
              owner: context.repo.owner,
              repo: context.repo.repo,
              state: 'open',
              labels: 'skill-audit',
              per_page: 100
            });
            
            const skillIssue = issues.find(i => 
              i.title.toLowerCase().includes(skill.toLowerCase())
            );
            
            if (skillIssue) {
              await github.rest.issues.update({
                owner: context.repo.owner,
                repo: context.repo.repo,
                issue_number: skillIssue.number,
                state: 'closed',
                state_reason: 'completed'
              });
              console.log(`Closed issue #${skillIssue.number}`);
            }

      - name: Upload Audit Report
        if: always() && steps.evaluate.outputs.skill != ''
        uses: actions/upload-artifact@v4
        with:
          name: audit-report-${{ steps.detect-skill.outputs.skill }}
          path: .context/audits/
          retention-days: 30
```

---

## Key Workflow Components

### 1. Path Filter

```yaml
paths:
  - 'skills/**'
```

Ensures workflow only runs when skill files change.

### 2. Skill Detection

Extracts skill name from changed files using:

```bash
git diff --name-only ${{ github.event.pull_request.base_sha }}...HEAD -- 'skills/**'
```

### 3. Threshold Check

```bash
if [ "$score" -ge 84 ]; then
  echo "status=pass"
else
  echo "status=fail"
fi
```

### 4. Issue Management

- **Creation**: Only when score < 84
- **Deduplication**: Close existing before creating new
- **Labels**: `skill-audit`, `needs-review`, `remediation`
- **Assignment**: Assign to PR author

### 5. Verification Flow

- On push to PR: workflow re-runs automatically
- Pass → closes issue automatically
- Fail → updates/creates issue with new score

---

## Validation Commands

After implementation, verify with:

```bash
# Check workflow syntax
bunx actlint .github/workflows/skill-audit-pr.yaml

# Test locally with act (optional)
act pull_request -W .github/workflows/skill-audit-pr.yaml --workflow-dispatch
```

---

## Success Criteria

- [ ] Workflow file created at `.github/workflows/skill-audit-pr.yaml`
- [ ] Triggers only on PRs modifying `skills/**`
- [ ] Extracts and audits only changed skill
- [ ] Posts PR comment with score and grade
- [ ] Creates issue only when score < 84
- [ ] Uses labels: `skill-audit`, `needs-review`, `remediation`
- [ ] Assigns issue to PR author
- [ ] Closes/updates issues on re-audit
- [ ] Uploads audit report as artifact
- [ ] Graceful failure with clear error messages

---

## Future Enhancements (Out of Scope)

1. Call external AI agent (Copilot/Gemini) for remediation suggestions
2. Commit audit reports to `.context/audits/` directory
3. Generate remediation plans automatically using AI
4. Support forked PRs with `pull_request_target`
5. Add dashboard showing audit trends over time

---

## Dependencies

- `skills/skill-quality-auditor/scripts/evaluate.sh` - Core evaluation script
- GitHub Actions (built-in)
- `actions/github-script@v7` - For API interactions
- `actions/upload-artifact@v4` - For report storage

---

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Workflow file creation | M | 2 hours |
| Testing (dry runs) | S | 30 min |
| Documentation | S | 30 min |
| **Total** | **M** | **~3 hours** |

---

## Rollback Plan

```bash
git checkout HEAD~1 -- .github/workflows/skill-audit-pr.yaml
```

---

## Notes

- Workflow uses `pull_request` event which provides `github.event.pull_request.changes` for accurate path filtering
- Artifact retention set to 30 days to balance storage and historical debugging needs
- Issue deduplication uses case-insensitive matching for flexibility
