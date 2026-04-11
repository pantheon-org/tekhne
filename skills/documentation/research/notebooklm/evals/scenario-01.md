# Scenario 01: Query a NotebookLM Notebook with Follow-Up

## User Prompt

The user asks: "Can you check my NotebookLM about the methodology section of the clinical trial I uploaded? I want to understand the inclusion and exclusion criteria."

The user has previously authenticated. Notebook ID `nb-001` labelled "Clinical Trial Protocol" is already in the library and active.

## Expected Behavior

1. Check authentication status by running `auth_manager.py status` before issuing any query
2. Route all script calls through `python ./scripts/run.py` — no direct script execution
3. Ask a specific, scoped question about inclusion and exclusion criteria rather than a generic methodology query
4. After receiving the first answer, check whether both inclusion AND exclusion criteria are covered and issue a follow-up query if either is missing
5. Combine all query results into a coherent, synthesised response before replying to the user

## Success Criteria

- **Authentication checked before querying**: The agent runs auth_manager.py status before issuing any query.
- **run.py wrapper used for all script invocations**: Every script call goes through python ./scripts/run.py — no direct script execution.
- **Question is specific and scoped**: The query explicitly asks about inclusion and exclusion criteria, not a generic 'tell me about the methodology'.
- **Follow-up mechanism applied**: After receiving the first answer, the agent checks whether both inclusion AND exclusion criteria are covered, and issues a follow-up query if either is missing.
- **All answers synthesised before responding**: The agent combines results from all queries into a coherent response rather than dumping raw NotebookLM output.

## Failure Conditions

- Agent issues queries without first checking authentication status
- Agent invokes scripts directly rather than through the run.py wrapper
- Agent asks a generic methodology question instead of scoping to inclusion/exclusion criteria
- Agent fails to issue a follow-up when one of inclusion or exclusion criteria is not addressed
- Agent dumps raw NotebookLM output without synthesising a coherent response
