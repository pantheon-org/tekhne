# Scenario 06: Full Publication Workflow Execution

## User Prompt

"Execute the complete Tessl public publication workflow for the bdd-testing skill."

## Expected Behavior

1. Agent runs quality audit: `sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh development/bdd-testing --json --store`
2. Agent checks audit results for A-grade threshold (≥108/120)
3. If below threshold, agent reviews remediation plan and blocks publication
4. Agent verifies evaluation-scenarios/ directory exists with 5+ scenarios
5. Agent checks tile.json for `private: false`
6. Agent runs `tessl skill review skills/development/bdd-testing`
7. If Tessl score < 90%, agent runs `tessl skill review --optimize`
8. Agent performs agent-agnostic compliance check
9. Agent runs publication readiness script (if available)
10. Only after all gates pass, agent executes `tessl skill publish skills/development/bdd-testing --public`
11. Agent verifies publication with `tessl search bdd-testing`

## Success Criteria

- Agent executes all workflow steps in correct order
- Agent blocks at any failed gate (quality, evals, tile config, compliance)
- Agent runs optimization if Tessl score < 90%
- Agent provides clear status updates at each step
- Agent only publishes after ALL requirements met
- Agent verifies publication success
- Agent handles errors gracefully at each step
- Total workflow completes end-to-end for publication-ready skill

## Failure Conditions

- Agent skips any workflow step
- Agent proceeds past a failed gate
- Agent publishes before completing all validations
- Agent doesn't optimize Tessl score when < 90%
- Agent ignores agent-agnostic compliance requirement
- Agent doesn't verify publication success
- Agent fails to provide status updates
- Agent executes steps out of order
