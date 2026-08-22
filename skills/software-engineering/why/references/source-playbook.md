# Source playbooks

The why skill spawns one investigator per available evidence category, each reading a single source-specific playbook below. The playbooks are concrete examples for common MCPs; adapt them for a different MCP in the same category.

| Category | Playbook | Example MCP it documents | This session's likely equivalent |
|---|---|---|---|
| Source control history | [`code-archaeology.md`](./sources/code-archaeology.md) | git, `gh` | git + `mcp-gitlab` (no `gh` remote for this Journal repo) |
| Issue / ticket tracker | [`linear.md`](./sources/linear.md) | Linear (adapt for Jira, GitHub Issues, Plane, Shortcut) | Jira via `mcp-atlassian`, or GitLab Issues via `mcp-gitlab` |
| Long-form documents | [`notion.md`](./sources/notion.md) | Notion (adapt for Confluence, Google Docs, Coda) | Confluence via `mcp-atlassian` |
| Real-time team chat | [`slack.md`](./sources/slack.md) | Slack (adapt for Discord, Microsoft Teams, Mattermost) | Slack MCP |
| Infrastructure observability | [`datadog.md`](./sources/datadog.md) | Datadog (adapt for New Relic, Honeycomb, Grafana, Splunk) | none connected by default — check before assuming |
| Error / exception tracking | [`sentry.md`](./sources/sentry.md) | Sentry (adapt for Rollbar, Bugsnag, Airbrake) | none connected by default — check before assuming |
| Product analytics warehouse | [`databricks.md`](./sources/databricks.md) | Databricks SQL (adapt for Snowflake, BigQuery, ClickHouse, dbt) | none connected by default — check before assuming |

Cross-cutting:

- [`incident-postmortem.md`](./sources/incident-postmortem.md). Add this if the target code looks defensive (null checks, retry, timeout, rate limit, feature flag, egress guard, OOM handler).

The "likely equivalent" column is a starting guess for this repo's usual toolset, not a guarantee — always confirm which MCPs are actually connected in the current session (see the Discovery step in `why`'s `SKILL.md`) before assigning investigators.
