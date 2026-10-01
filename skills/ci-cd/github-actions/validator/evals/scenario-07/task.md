# Scenario 07: Validate a Reusable Workflow Pair

## User Prompt

A platform team has introduced a reusable workflow pattern to standardize how microservices run their tests. They have a shared `test-template.yml` workflow (the callee) and a `service-ci.yml` (the caller) that invokes it. Before enabling this pattern across 20 services, they want the pair validated to catch any issues that static analysis might surface, including expression type errors and input/output mismatches that only appear when both files are considered together.

Validate both workflow files, document any issues found in either file, and note any limitations where runtime-only behavior cannot be verified by static analysis alone.
