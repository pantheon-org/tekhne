# Scenario 01: Save with a Named Stream

## User Prompt

"Save context as stream 'api-refactor' — we finished extracting the auth middleware."

## Expected Behavior

1. Agent parses `api-refactor` as the stream name from arguments
2. Agent parses "we finished extracting the auth middleware" as the description
3. Agent gathers data in parallel: lists `.context/session/` directory and any existing CONTEXT files
4. Agent synthesizes conversation into Next tasks (3 inferred), Session progression, Hot Files (≤10), and a Focus/Goal statement
5. Agent writes `.context/session/CONTEXT-api-refactor-llm.md` using the canonical template
6. Agent runs `scripts/upsert-index.sh` with area, project, context=`api-refactor`, status, focus, and saved date
7. Agent reports: file path written and INDEX.md update result

## Success Criteria

- **Stream naming**: Output file is `CONTEXT-api-refactor-llm.md`, not `CONTEXT-llm.md`
- **Template compliance**: File contains all required sections (saved timestamp, stream, status, focus, goal, Next, Session, Hot Files, Refs)
- **Token budget**: Total file is 1200-1500 tokens max; Session section is ≤780 tokens
- **INDEX upsert**: `scripts/upsert-index.sh` is called with all 6 required positional args
- **Next tasks inferred**: Three actionable next tasks derived from the conversation, not generic placeholders

## Failure Conditions

- Agent writes to `CONTEXT-llm.md` instead of `CONTEXT-api-refactor-llm.md`
- Agent omits the `upsert-index.sh` call
- Output file lacks any of the required template sections
- Session section exceeds 780 tokens
- Next tasks are empty or contain generic filler ("continue working", "do more things")
