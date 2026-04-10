# Scenario 05: LogQL Query for a New Team Member

## User Prompt

A junior platform engineer just joined a team that uses Grafana Loki for log management. They have been asked to build a query to investigate a recurring issue: sporadic database connection failures in the `auth-service`. The service runs in Kubernetes and logs in JSON format. Relevant log fields include `level`, `component` (value: `"db"` for database logs), `error_type`, and `message`. Stream labels available are `app="auth-service"`, `namespace="backend"`, and `env="staging"`.

The engineer is not yet confident with LogQL and wants to understand how the query is constructed, not just receive a final answer. They have asked for help understanding what each part of the query does before seeing the full version, and want to be able to take away a reference they can consult later.

The final goal is a log-filter query that returns only error-level lines from the database component of the auth-service, formatted to show the `error_type` and `message` fields prominently.

Produce a file named `learning_session.md` that:

1. Shows the query being built one stage at a time — each step should be on its own, runnable, and there should be at least 3 intermediate steps before the final query.
2. Explains what each step adds and why it is done in that order.
3. Shows the final complete query.
4. Includes a section explaining how to run the final query (at least two different methods, e.g. Grafana UI and a CLI tool).
5. Includes a note identifying which labels in the stream selector or filters the engineer would need to change when moving to production.

## Expected Behavior

1. Present at least 3 distinct intermediate queries that build incrementally (stream selector → line filter → parser → label filter)
2. Ensure the line filter step (`|= "error"` or similar) appears before the json parser step
3. Accompany each intermediate step with a plain-English explanation of what it adds and why
4. Use `line_format` or `label_format` in the final query to surface `error_type` and `message` prominently
5. In the final query, maintain correct order: line filter before json parser, label filters after parser
6. Provide at least two ways to run the query (e.g. Grafana Explore UI and `logcli`, or HTTP API)
7. Identify at least one label or value to change when moving from staging to production
8. Include at least two label matchers in the final query's stream selector

## Success Criteria

- **Incremental steps present**: The document shows at least 3 distinct intermediate queries, each building on the previous step (stream selector → line filter → parser → label filter)
- **Line filter before parser in steps**: The step that adds a line filter (`|= "error"` or similar) appears BEFORE the step that adds the json parser
- **Each step explained**: Each intermediate step is accompanied by a plain-English explanation of what it adds and why
- **line_format used for output**: The final query uses `line_format` or `label_format` to surface the `error_type` and `message` fields prominently
- **Filter ordering correct in final query**: In the final query, line filter comes before json parser, and label filters (`level`, `component`) come after the parser
- **Two usage methods provided**: The document mentions at least two ways to run the query (e.g. Grafana Explore UI and `logcli`, or HTTP API)
- **Customization notes present**: The document identifies at least one label or value to change when moving from staging to production
- **Specific stream selector**: The final query stream selector includes at least two label matchers

## Failure Conditions

- Provides fewer than 3 intermediate steps, jumping straight to the final query
- Introduces the json parser before the line filter in the step sequence
- Omits explanations for one or more intermediate steps
- Final query does not use `line_format` or `label_format` to format output fields
- Final query places the line filter after the json parser
- Only describes one way to run the query
- Provides no customization notes for moving to production
- Final stream selector uses only a single label matcher
