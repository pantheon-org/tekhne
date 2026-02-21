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

      - name: Make scripts executable
        run: chmod +x skills/skill-quality-auditor/scripts/*.sh

      - name: Evaluate changed skills
        id: evaluate
        run: |
          FAILED=""
          for file in $(git diff --name-only origin/main | grep "skills/.*/SKILL.md"); do
            skill_name=$(basename $(dirname $file))
            echo "Evaluating: $skill_name"
            
            cd skills/skill-quality-auditor
            score=$(sh scripts/evaluate.sh "$skill_name" --json 2>/dev/null | jq '.total')
            cd ../..
            
            if [ -n "$score" ] && [ "$score" -lt 90 ]; then
              echo "::warning::$skill_name scored $score/120 (below A-grade threshold 90)"
              FAILED="$FAILED $skill_name"
            fi
          done
          
          if [ -n "$FAILED" ]; then
            echo "failed=true" >> $GITHUB_OUTPUT
            echo "skills=$FAILED" >> $GITHUB_OUTPUT
          fi

      - name: Check for duplication
        run: |
          cd skills/skill-quality-auditor
          ./scripts/detect-duplication.sh ../..
          if grep -q "Critical" ../../.context/analysis/duplication-report-*.md; then
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

      - name: Run full audit
        run: |
          chmod +x skills/skill-quality-auditor/scripts/*.sh
          skills/skill-quality-auditor/scripts/audit-skills.sh
          skills/skill-quality-auditor/scripts/detect-duplication.sh

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
    - chmod +x skills/skill-quality-auditor/scripts/*.sh
    - |
      for file in $(git diff --name-only $CI_MERGE_REQUEST_DIFF_BASE_SHA $CI_COMMIT_SHA | grep "skills/.*/SKILL.md"); do
        skill_name=$(basename $(dirname $file))
        cd skills/skill-quality-auditor
        sh scripts/evaluate.sh "$skill_name" --json
        cd ../..
      done
    - skills/skill-quality-auditor/scripts/detect-duplication.sh
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
    - chmod +x skills/skill-quality-auditor/scripts/*.sh
    - skills/skill-quality-auditor/scripts/audit-skills.sh
    - skills/skill-quality-auditor/scripts/detect-duplication.sh
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
    
    stage('Quality Check') {
      when {
        anyOf {
          changeset 'skills/**/SKILL.md'
          branch 'main'
        }
      }
      steps {
        sh '''
          chmod +x skills/skill-quality-auditor/scripts/*.sh
          
          for file in $(git diff --name-only origin/main HEAD | grep "skills/.*/SKILL.md"); do
            skill_name=$(basename $(dirname $file))
            cd skills/skill-quality-auditor
            sh scripts/evaluate.sh "$skill_name" --json
            cd ../..
          done
        '''
      }
    }
    
    stage('Duplication Check') {
      steps {
        sh 'skills/skill-quality-auditor/scripts/detect-duplication.sh'
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
