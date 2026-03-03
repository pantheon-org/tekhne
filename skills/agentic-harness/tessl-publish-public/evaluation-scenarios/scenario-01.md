# Scenario 01: Basic Publication Readiness Check

## User Prompt

"Is the nx-vite-integration skill ready to publish publicly on Tessl?"

## Expected Behavior

1. Agent locates the skill directory at `skills/repository-mgmt/nx-vite-integration/`
2. Checks for `evaluation-scenarios/` directory existence
3. Verifies `tile.json` exists and checks `private` field value
4. Looks for recent skill-quality-auditor results in `.context/audits/repository-mgmt/nx-vite-integration/`
5. Reports publication readiness status with specific missing requirements
6. Provides remediation steps if requirements are unmet

## Success Criteria

- Agent identifies evaluation scenarios directory (exists or missing)
- Agent checks tile.json for `private: false` setting
- Agent checks for quality audit results (≥108/120 A-grade required)
- Agent provides specific checklist of what's missing
- Agent does NOT attempt to publish if requirements are unmet
- Agent reports clear pass/fail status for each requirement

## Failure Conditions

- Agent skips evaluation scenario check entirely
- Agent publishes without verifying `private: false` in tile.json
- Agent ignores quality audit requirement
- Agent provides generic "looks good" without validation
- Agent proceeds to publish command without completing checks
- Agent misses any of the three critical requirements (evals, quality, tile config)
