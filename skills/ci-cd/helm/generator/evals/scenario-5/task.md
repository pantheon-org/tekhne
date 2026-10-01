# Scenario 5: Migrate Raw Kubernetes Manifests to Helm

## User Prompt

A startup has been running their `notification-service` on Kubernetes using raw YAML manifests managed by kubectl. They have decided to adopt Helm so they can version the chart, parameterise environment differences, and share it with other teams. The raw manifests are provided below.

The existing manifests have several issues that the migration should fix: image tags are hardcoded, all resource names are hardcoded strings, there are no labels following Kubernetes recommended conventions, and no resource limits are set. The migration should produce a proper Helm chart that addresses these issues.

Additionally, the team wants a simple shell script `validate.sh` that documents their intended validation workflow for the chart — covering both static linting and rendered manifest checking.
