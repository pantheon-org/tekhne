# Scenario 02: Multi-Environment Deployment Pipeline

## User Prompt

A fintech startup runs a `payments-api` service across three environments: development, staging, and production. Each environment uses a different replica count, database connection string, and ingress hostname. The team is setting up a GitHub Actions workflow to deploy to each environment on merge.

Currently, the chart's `values.yaml` contains production database URLs and replica counts — meaning a developer deploying to dev accidentally inherits prod-scale settings. The team wants a clean separation between the chart's built-in defaults and what changes per environment. They also experienced a painful incident last month where a `helm upgrade` left a release in a broken state after a failed deployment, and they want CI to automatically roll back on failure.

## Output Specification

Produce:
1. A minimal Helm chart directory named `payments-api/` with `Chart.yaml`, a clean `values.yaml` (defaults only), and at least one template file.
2. An environment override file `values-production.yaml` at the repo root demonstrating at least 2 production-specific overrides (replica count and an ingress hostname are good examples).
3. A shell script `deploy.sh` that performs the Helm deployment in a way that handles failures gracefully and can be called as: `bash deploy.sh <environment> <release-name> <chart-path>`.

The `values.yaml` inside the chart must not contain any environment-specific values.

## Expected Behavior

1. Keep `values.yaml` free of environment-specific strings (no production hostnames, prod database URLs, or environment names like `prod` or `staging`)
2. Create a separate `values-production.yaml` (or equivalent) outside the chart directory with at least 2 override entries
3. In `deploy.sh`, pass the environment override file to helm using the `-f` flag rather than editing `values.yaml`
4. Include `--atomic` on the `helm upgrade` command in `deploy.sh` so failed deployments are automatically rolled back
5. Add `--timeout` alongside `--atomic` (e.g., `--timeout 5m`) to cap the wait time
6. Use `helm upgrade --install` (idempotent form) and accept the environment as a script argument

## Success Criteria

- **No env values in chart**: values.yaml inside the chart directory does NOT contain any environment-specific strings (e.g., production hostnames, prod database URLs, or environment names like 'prod', 'staging')
- **Separate override file**: A separate values-production.yaml (or equivalent env-specific file) exists outside the chart directory with at least 2 override entries
- **Layered -f flag usage**: deploy.sh passes the environment override file to helm using the -f flag (e.g., -f values-production.yaml), not by editing values.yaml
- **atomic flag present**: The helm upgrade command in deploy.sh includes the --atomic flag
- **timeout with atomic**: The helm upgrade command in deploy.sh includes a --timeout flag alongside --atomic (e.g., --timeout 5m)
- **upgrade --install used**: deploy.sh uses 'helm upgrade --install' (idempotent form) rather than separate helm install and helm upgrade commands
- **Image tag not latest**: values.yaml does NOT set the image tag to 'latest'; it is either empty or a placeholder variable
- **Resources defined**: The chart template or values.yaml includes a resources block with CPU and memory settings
- **values.yaml comments**: values.yaml contains at least 3 comment lines (starting with #) documenting individual settings
- **Environment param in script**: deploy.sh accepts the target environment as a script argument or variable and selects the appropriate values file based on it

## Failure Conditions

- `values.yaml` inside the chart contains production hostnames, database URLs, or environment-specific strings
- No separate environment override file exists outside the chart directory
- `deploy.sh` modifies `values.yaml` instead of passing override files with `-f`
- `deploy.sh` does not include `--atomic` on the helm upgrade command
- `deploy.sh` does not include `--timeout` alongside `--atomic`
- `deploy.sh` uses separate `helm install` and `helm upgrade` commands instead of `helm upgrade --install`
- `values.yaml` sets the image tag to `latest`
- No `resources:` block is present in the chart
