# Scenario 02: Resume a Named Stream

## User Prompt

"Load context feature-auth"

## Expected Behavior

1. Agent extracts `feature-auth` from `$ARGUMENTS`
2. Agent runs the detection command (`rtk ls -t`) in parallel with reading `.context/session/CONTEXT-feature-auth-llm.md`
3. Agent parses the named stream file and formats the resume report
4. Resume report title reflects the stream name (`feature-auth`)
5. Agent outputs all required blocks: header fields, NextTasks, Session Context, Hot Files, Next Step

## Success Criteria

- Agent reads `.context/session/CONTEXT-feature-auth-llm.md` (correct filename format for named stream)
- Detection shell command and file read are issued as parallel tool calls in one message
- Report title includes the stream name `feature-auth`
- Agent does NOT prompt the user to choose a stream when stream name is already given
- NextTasks and Hot Files blocks are present in the output
- Next Step sentence references the named stream's focus or top task

## Failure Conditions

- Agent reads `CONTEXT-llm.md` (default file) instead of the named stream file
- Agent runs detection and file read sequentially instead of in parallel
- Agent asks a clarification question before loading when the stream name was supplied
- Report title omits the stream name
- Agent re-orders or re-synthesizes NextTasks content rather than presenting it directly
