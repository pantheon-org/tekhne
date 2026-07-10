# Task: Save a Recurring Bug Workaround

You are an AI coding assistant. During a debugging session, you and the developer discovered a known issue:

> "Whenever the `auth-service` container starts before the `db` container is fully ready, the health check fails silently. The workaround is to add `depends_on` with `condition: service_healthy` in docker-compose.yml."

Write the shell command(s) you would run to record this finding.
