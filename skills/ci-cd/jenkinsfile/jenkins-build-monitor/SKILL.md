---
name: jenkins-build-monitor
description: Monitor Jenkins build status across multiple dashboards and identify failed builds requiring attention. Use when checking Jenkins CI status, reviewing failing builds, or querying build monitor dashboards.
---

# Jenkins Build Monitor

Monitor Jenkins build status across multiple dashboards and identify failed builds requiring attention.

## Trigger Conditions

Use this skill when the user requests:
- "Check Jenkins build status"
- "What's failing on Jenkins?"
- "Jenkins dashboard status"
- "Check build monitors"
- "Show me Jenkins failures"

## Prerequisites

- VPN connection to your organization's VPN (required for Jenkins access)
- Jenkins credentials configured in environment or netrc
- `jq` and `curl` available

## Monitored Dashboards

Configure the dashboards you want to monitor. Each dashboard entry requires:

- A display name
- The Jenkins view URL in the format: `https://<jenkins-host>/job/<folder>/view/<view-name>/`

Example structure:

```
1. Build Dashboard A
   - URL: https://<jenkins-host>/job/<folder>/view/<view-name>/

2. Build Dashboard B
   - URL: https://<jenkins-host>/job/<folder>/view/<view-name>/
```

## Workflow

### 1. VPN Connection Check

First verify VPN connectivity:

```bash
# Check if Jenkins is reachable
if ! curl -s --connect-timeout 5 https://<jenkins-host> > /dev/null 2>&1; then
  echo "ERROR: VPN connection required. Please connect to your organization VPN."
  exit 1
fi
```

### 2. Fetch Build Status

Use the bundled script to check all dashboards:

```bash
~/.config/opencode/skills/jenkins-build-monitor/scripts/check-builds.sh
```

### 3. Parse and Report

The script will:
- Check each dashboard via Jenkins API
- Identify failed/unstable builds
- Show build names, status, and last failure time
- Provide direct links to failed builds

## Output Format

```
Jenkins Build Status Report
===========================

[DASHBOARD: My Builds]
✓ build-job-1: SUCCESS
✗ build-job-2: FAILURE (last failed: 2 hours ago)
  → https://<jenkins-host>/job/build-job-2/

[DASHBOARD: Integration Tests]
⚠ test-job-1: UNSTABLE (last failed: 30 minutes ago)
  → https://<jenkins-host>/job/test-job-1/

Summary:
- Total jobs monitored: 45
- Failed: 2
- Unstable: 1
- Requires attention: 3
```

## Scripts

### check-builds.sh

Located at: `~/.config/opencode/skills/jenkins-build-monitor/scripts/check-builds.sh`

Checks all configured dashboards and reports status.

### get-view-jobs.sh

Located at: `~/.config/opencode/skills/jenkins-build-monitor/scripts/get-view-jobs.sh`

Helper script to fetch jobs from a specific Jenkins view.

## Configuration

### Environment Variables

```bash
# Required: Jenkins host
export JENKINS_HOST="https://jenkins.example.com"

# Optional: Jenkins credentials (if not in .netrc)
export JENKINS_USER="your-username"
export JENKINS_TOKEN="your-api-token"

# Optional: Custom timeout for API calls
export JENKINS_TIMEOUT=10
```

### .netrc Configuration

Alternatively, configure credentials in `~/.netrc`:

```
machine jenkins.example.com
login your-username
password your-api-token
```

## Jenkins API Endpoints

The skill uses Jenkins JSON API:

```bash
# View API
https://<jenkins-host>/job/{folder}/view/{view}/api/json?tree=jobs[name,url,color,lastBuild[number,result,timestamp,duration]]

# Job API
https://<jenkins-host>/job/{job}/api/json?tree=lastBuild[number,result,timestamp,duration,url]
```

## Error Handling

- **VPN not connected**: Clear error message with instructions
- **Authentication failed**: Check credentials configuration
- **Timeout**: Increase JENKINS_TIMEOUT value
- **API changes**: Update tree parameter in API calls

## Usage Examples

### Check all dashboards

```bash
~/.config/opencode/skills/jenkins-build-monitor/scripts/check-builds.sh
```

### Check specific dashboard

```bash
~/.config/opencode/skills/jenkins-build-monitor/scripts/check-builds.sh --dashboard "Integration Tests"
```

### JSON output for automation

```bash
~/.config/opencode/skills/jenkins-build-monitor/scripts/check-builds.sh --json
```

### Only show failures

```bash
~/.config/opencode/skills/jenkins-build-monitor/scripts/check-builds.sh --failures-only
```

## Troubleshooting

### Cannot connect to Jenkins

1. Verify VPN connection: `ping <vpn-host>`
2. Test Jenkins connectivity: `curl -I https://<jenkins-host>`
3. Check firewall rules

### Authentication errors

1. Verify credentials in .netrc or environment variables
2. Test with: `curl -u $JENKINS_USER:$JENKINS_TOKEN https://<jenkins-host>/api/json`
3. Regenerate API token if needed

### Slow response

1. Increase timeout: `export JENKINS_TIMEOUT=30`
2. Check network latency to Jenkins server
3. Consider caching results for frequent checks

## Integration

### With CI/CD

```yaml
# .gitlab-ci.yml example
check-jenkins-status:
  script:
    - ~/.config/opencode/skills/jenkins-build-monitor/scripts/check-builds.sh --json > status.json
    - if jq -e '.summary.failed > 0' status.json; then exit 1; fi
```

### With Slack/notifications

```bash
# Send failures to Slack
FAILURES=$(~/.config/opencode/skills/jenkins-build-monitor/scripts/check-builds.sh --failures-only)
if [ -n "$FAILURES" ]; then
  curl -X POST -H 'Content-type: application/json' \
    --data "{\"text\":\"Jenkins Failures:\n$FAILURES\"}" \
    $SLACK_WEBHOOK_URL
fi
```

## References

- [Jenkins Remote Access API](https://www.jenkins.io/doc/book/using/remote-access-api/)
- [Jenkins REST API](https://www.jenkins.io/doc/book/using/remote-access-api/)
