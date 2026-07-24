# Scenario: Flags ordering and missing HEALTHCHECK, handles missing tool

hadolint is not installed and the Dockerfile sets USER after CMD with no HEALTHCHECK. Validate and decide how to proceed.
