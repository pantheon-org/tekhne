# Scenario 05: Generate Stage 10 Final Validation Report

## User Prompt

You have completed stages 1-9 of the helm-validator workflow on a chart called `monitoring-stack`. The results from each stage are summarised below:

- **Stage 1 (Tool Check):** helm v3.14.0, yamllint 1.35.1, kubeconform 0.6.4 all present. kubectl not available.
- **Stage 2 (Structure):** Chart.yaml present (apiVersion: v2). values.yaml present. templates/ present. Missing: .helmignore, NOTES.txt.
- **Stage 3 (Helm Lint):** 0 errors. 1 warning: `icon` field missing from Chart.yaml.
- **Stage 4 (Template Render):** 6 templates rendered successfully to ./rendered/.
- **Stage 5 (YAML Syntax):** 1 issue: trailing whitespace on line 14 of rendered/deployment.yaml.
- **Stage 6 (CRD Detection):** 1 CRD detected: `monitoring.coreos.com/v1 ServiceMonitor`. Documentation lookup performed — required fields: `selector`, `endpoints`.
- **Stage 7 (Schema Validation):** Deployment, Service, ConfigMap: valid. ServiceMonitor: "No schema found" (expected for CRD). Manual check using Stage 6 docs: `selector` present, `endpoints` present — passes.
- **Stage 8 (Dry-Run):** Skipped — no cluster access. Documented as limitation.
- **Stage 9 (Security):** Deployment has no pod securityContext. Container has no `allowPrivilegeEscalation: false`. Image tag is `v2.1.0` (pinned — good). No livenessProbe or readinessProbe. Resource limits missing for memory.

## Your Task

Produce the complete Stage 10 Final Report exactly as specified by the helm-validator skill, including:

1. A validation summary table covering all 9 stages with status icons and issue counts.
2. Categorised issue list (Errors, Warnings, Info) with file references.
3. Proposed changes section with at least two before/after code blocks.
4. An automation opportunities section listing any applicable helper commands.
5. A final summary block with chart name, overall status, issue counts, and next steps.

## Expected Behavior

1. Include a summary table with a row for each of the 9 stages using status icons (pass/warning/skipped) and issue counts matching the provided stage results
2. Split issues into Errors (must fix), Warnings (should fix), and Info categories — security findings (missing securityContext, missing probes, missing memory limits) as Warnings; missing .helmignore and NOTES.txt as Warnings or Info
3. Provide at least two proposed changes with file reference, before code block, and after code block
4. List at least one automation helper such as scripts/generate_helpers.sh for _helpers.tpl or copying assets/.helmignore
5. End with a summary block containing the chart name (monitoring-stack), overall status, total error/warning/info counts, and at least one next step
6. Explicitly state that no files were modified and the user should apply fixes or use helm-generator

## Success Criteria

- **Validation summary table**: Report includes a table with a row for each of the 9 stages, using status icons (pass/warning/skipped) and issue counts that match the provided stage results
- **Categorised issue list**: Issues are correctly split into Errors (must fix), Warnings (should fix), and Info categories. The missing securityContext, missing probes, and missing memory limits appear as Warnings. Missing .helmignore and NOTES.txt appear as Warnings or Info
- **Proposed changes with before/after blocks**: At least two proposed changes include a file reference, a before code block, and an after code block
- **Automation opportunities listed**: Report mentions at least one automation helper such as scripts/generate_helpers.sh for _helpers.tpl or copying assets/.helmignore for the missing .helmignore
- **Final summary block present**: Report ends with a summary block containing the chart name (monitoring-stack), overall status, total error/warning/info counts, and at least one next step
- **Read-only posture maintained throughout**: Report proposes all changes but explicitly states that no files were modified and the user should apply or use helm-generator to apply fixes

## Failure Conditions

- The validation summary table is absent or does not cover all 9 stages
- Issues are not categorised into Errors, Warnings, and Info
- No proposed changes include both a before and after code block
- No automation opportunities or helper commands are mentioned
- The final summary block is absent or lacks chart name, overall status, or issue counts
- The report implies files were modified rather than maintaining a read-only posture
