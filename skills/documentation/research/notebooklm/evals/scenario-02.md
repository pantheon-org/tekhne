# Scenario 02: Add a New Notebook Using Smart Add

## User Prompt

The user says: "I have a new NotebookLM notebook at https://notebooklm.google.com/notebook/abc123 — can you add it to my library?"

No name, description, or topics have been provided. Authentication is already set up.

## Expected Behavior

1. Refrain from inventing a name, description, or topics — do not call `notebook_manager.py add` with guessed or placeholder values before querying the notebook
2. Use Smart Add: call `ask_question.py` with a content-discovery question before calling `notebook_manager.py add`
3. Ask a discovery question that requests a concise overview of the notebook's content and covered topics
4. Use the discovered content to populate `--name`, `--description`, and `--topics` in the add command
5. Route all script calls through `python ./scripts/run.py`

## Success Criteria

- **Does NOT invent name or description**: The agent does not call notebook_manager.py add with guessed or placeholder values before querying the notebook.
- **Smart Add: queries notebook before adding**: The agent calls ask_question.py with a content-discovery question before calling notebook_manager.py add.
- **Discovery question asks for content overview and topics**: The discovery query asks for a concise overview of the notebook's content and covered topics.
- **Discovered content used to populate all required fields**: The add command includes --name, --description, and --topics derived from the query result, not generic placeholders.
- **run.py wrapper used for all script invocations**: Every script call goes through python ./scripts/run.py.

## Failure Conditions

- Agent calls `notebook_manager.py add` with invented or placeholder metadata before querying the notebook
- Agent skips the content-discovery query and adds the notebook with no meaningful metadata
- Agent's discovery question is too broad or does not ask about topics
- Agent populates `--name`, `--description`, or `--topics` with generic placeholders not derived from the notebook
- Agent invokes scripts directly rather than through the run.py wrapper
