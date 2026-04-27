---
category: scripts
priority: MEDIUM
source: CI/CD integration experience
---

# CI/CD Integration for Skill Quality

Setup guides for GitHub Actions, GitLab CI, and Jenkins pipelines to automate skill quality enforcement.

## Integration Overview

**Purpose**: Automate quality checks in CI/CD pipelines  
**Enforcement**: Block PRs with low-quality skills  
**Reporting**: Generate quality reports on schedule

## GitHub Actions

### Quality Gate Workflow

Block PRs with skills below A-grade:

```yaml
# .github/workflows/skill-quality.yml
name: Skill Quality Gate

on:
  pull_request:
    paths:
      - 'skills/**/SKILL.md'
  workflow_dispatch:

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Setup Bun
        uses: oven-sh/setup-bun@v1

      - name: Install dependencies
        run: bun install

      - name: Build skill-auditor
        run: bun run build:skill-auditor

      - name: Evaluate changed skills
        id: evaluate
        run: |
          FAILED=""
          for file in $(git diff --name-only origin/main | grep "skills/.*/SKILL.md"); do
            skill=$(echo "$file" | sed 's|skills/||;s|/SKILL.md||')
            echo "Evaluating: $skill"
            score=$(skill-auditor evaluate "$skill" --json 2>/dev/null | jq '.total')
            
            if [ -n "$score" ] && [ "$score" -lt 90 ]; then
              echo "::warning::$skill scored $score/140 (below threshold 90)"
              FAILED="$FAILED $skill"
            fi
          done
          
          if [ -n "$FAILED" ]; then
            echo "failed=true" >> $GITHUB_OUTPUT
            echo "skills=$FAILED" >> $GITHUB_OUTPUT
          fi

      - name: Check for duplication
        run: |
          ./scripts/detect-duplication.sh
          if grep -q "Critical" .context/analysis/duplication-report-*.md; then
            echo "::warning::Critical duplication detected"
          fi

      - name: Fail if quality issues
        if: steps.evaluate.outputs.failed == 'true'
        run: |
          echo "Quality gate failed for skills: ${{ steps.evaluate.outputs.skills }}"
          exit 1

      - name: Upload reports
        uses: actions/upload-artifact@v4
        with:
          name: quality-reports
          path: .context/analysis/
          retention-days: 30
```

### Scheduled Audit Workflow

Weekly automated audits:

```yaml
# .github/workflows/weekly-audit.yml
name: Weekly Skill Audit

on:
  schedule:
    - cron: '0 0 * * 0'  # Sunday at midnight
  workflow_dispatch:

jobs:
  audit:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Bun
        uses: oven-sh/setup-bun@v1

      - name: Build skill-auditor
        run: bun run build:skill-auditor

      - name: Run full audit
        run: |
          skills=$(find skills -name "SKILL.md" | sed 's|skills/||;s|/SKILL.md||' | tr '\n' ' ')
          skill-auditor batch $skills --store
          ./scripts/detect-duplication.sh

      - name: Create issue if critical issues found
        run: |
          if grep -q "C Grade" .context/analysis/skill-audit-*.md; then
            gh issue create \
              --title "Skill Quality Issues - $(date +%Y-%m-%d)" \
              --body-file .context/analysis/skill-audit-*.md \
              --label "quality,skills"
          fi
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      - name: Upload reports
        uses: actions/upload-artifact@v4
        with:
          name: weekly-audit-reports
          path: .context/analysis/
```

## GitLab CI

### Quality Gate Pipeline

```yaml
# .gitlab-ci.yml
skill-quality:
  stage: test
  image: oven/bun:1
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
      changes:
        - skills/**/SKILL.md
  script:
    - bun install && bun run build:skill-auditor
    - |
      for file in $(git diff --name-only $CI_MERGE_REQUEST_DIFF_BASE_SHA $CI_COMMIT_SHA | grep "skills/.*/SKILL.md"); do
        skill=$(echo "$file" | sed 's|skills/||;s|/SKILL.md||')
        skill-auditor evaluate "$skill" --json --store
      done
    - ./scripts/detect-duplication.sh
  artifacts:
    paths:
      - .context/analysis/
    expire_in: 1 week
```

### Scheduled Pipeline

```yaml
weekly-skill-audit:
  stage: deploy
  image: oven/bun:1
  rules:
    - if: $CI_PIPELINE_SOURCE == "schedule"
  script:
    - bun install && bun run build:skill-auditor
    - skills=$(find skills -name "SKILL.md" | sed 's|skills/||;s|/SKILL.md||' | tr '\n' ' ')
    - skill-auditor batch $skills --store
    - ./scripts/detect-duplication.sh
  artifacts:
    paths:
      - .context/analysis/
    expire_in: 30 days
```

## Jenkins

### Jenkinsfile

```groovy
pipeline {
  agent any
  
  tools {
    nodejs 'NodeJS-18'
  }
  
  stages {
    stage('Install') {
      steps {
        sh 'bun install'
      }
    }
    
    stage('Build') {
      steps {
        sh 'bun run build:skill-auditor'
      }
    }

    stage('Quality Check') {
      when {
        anyOf {
          changeset 'skills/**/SKILL.md'
          branch 'main'
        }
      }
      steps {
        sh '''
          for file in $(git diff --name-only origin/main HEAD | grep "skills/.*/SKILL.md"); do
            skill=$(echo "$file" | sed 's|skills/||;s|/SKILL.md||')
            skill-auditor evaluate "$skill" --json --store
          done
        '''
      }
    }
    
    stage('Duplication Check') {
      steps {
        sh './scripts/detect-duplication.sh'
      }
    }
  }
  
  post {
    always {
      archiveArtifacts artifacts: '.context/analysis/**', allowEmptyArchive: true
    }
  }
}
```

## Quality Gate Configuration

### Thresholds

| Metric | Warning | Error |
|--------|---------|-------|
| Skill score | <100 | <90 |
| Duplication | >20% | >35% |
| File size | >300 lines | >500 lines |
| Missing description | N/A | Yes |

### Bypass Options

For emergency merges:

```yaml
# GitHub Actions
if: "!contains(github.event.head_commit.message, '[skip quality]')"

# GitLab CI
except:
  variables:
    - $CI_COMMIT_MESSAGE =~ /skip quality/
```

## Notifications

### Slack Integration

```yaml
- name: Notify on failure
  if: failure()
  uses: slackapi/slack-github-action@v1
  with:
    channel-id: 'skill-quality'
    slack-message: |
      :warning: Skill quality gate failed
      PR: ${{ github.event.pull_request.html_url }}
      Failed skills: ${{ steps.evaluate.outputs.skills }}
  env:
    SLACK_BOT_TOKEN: ${{ secrets.SLACK_BOT_TOKEN }}
```

### Email Notifications

```yaml
- name: Send report
  if: always()
  uses: dawidd6/action-send-mail@v3
  with:
    server_address: smtp.example.com
    to: team@example.com
    subject: Skill Audit Report - ${{ github.repository }}
    body: file://.context/analysis/skill-audit-*.md
```

## See Also

- `scripts-audit-workflow.md` - Audit process
- `reporting-analysis.md` - Interpreting results
- `advanced-trends-analysis.md` - Historical tracking
