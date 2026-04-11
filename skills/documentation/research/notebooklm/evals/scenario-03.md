# Scenario 03: First-Time Setup and Notebook Query

## User Prompt

The user asks: "I've never used NotebookLM through Claude before — can you set it up and then search my machine learning notebook for anything about attention mechanisms?"

No authentication exists. No notebooks are in the library yet. The user has a notebook at https://notebooklm.google.com/notebook/ml-research.

## Expected Behavior

1. Run `auth_manager.py status` as the very first action before any notebook or query operations
2. Run `auth_manager.py setup` and explicitly warn the user that a browser window will open for Google login
3. Add or activate the ML research notebook before issuing any question
4. Route all script calls through `python ./scripts/run.py` — no direct script execution
5. After receiving the first answer about attention mechanisms, check completeness and issue follow-ups if needed
6. Combine all query results into a coherent response before replying to the user

## Success Criteria

- **Authentication status checked first**: The agent runs auth_manager.py status as the very first action before any notebook or query operations.
- **Auth setup run with browser-visible warning**: The agent runs auth_manager.py setup and explicitly tells the user that a browser window will open for Google login.
- **Notebook added or activated before querying**: The agent adds or activates the ML research notebook before issuing any question.
- **run.py wrapper used for every script invocation**: Every script call goes through python ./scripts/run.py — no direct script execution.
- **Follow-up mechanism applied**: After receiving the first answer about attention mechanisms, the agent checks completeness and issues follow-ups if needed.
- **Synthesised response delivered to user**: The agent combines all query results into a coherent answer before replying to the user.

## Failure Conditions

- Agent skips the authentication status check or does not run it first
- Agent runs auth setup without warning the user that a browser window will open
- Agent issues queries without first adding or activating the notebook
- Agent invokes scripts directly rather than through the run.py wrapper
- Agent fails to apply the follow-up mechanism when the first answer is incomplete
- Agent dumps raw query output without synthesising a coherent final response
