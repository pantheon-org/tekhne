# Scenario 05: Log Forwarding to Authenticated HTTP Endpoint

## User Prompt

A financial services company collects application logs from multiple servers and needs to forward them to a centralized log aggregation service over HTTPS. The destination accepts logs at `https://logs.acme-corp.internal/api/ingest` and requires HTTP Basic Authentication (username: `fluentbit-writer`, password provided at runtime). The security team has flagged a previous configuration where the password was written directly into the config file that ended up committed to the company's internal Git repository, triggering a secrets rotation exercise. They now require that no credentials appear in any configuration file.

The agent is given a broken starting configuration below. The team wants the config corrected and ready for production use, forwarding structured JSON logs from `/var/app/logs/*.log`.
