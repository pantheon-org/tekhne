# Scenario 4: Proactively Detect and Capture a Bridge Opportunity

## User Prompt

"I just realised — the complexity-management patterns I'm developing for the regulatory compliance project would transfer perfectly to the financial-planning work. Same bureaucratic structure, same negotiation skills."

## Expected Behavior

1. Agent recognises a bridge opportunity from the user's statement without an explicit `/bridge` command.
2. Agent resolves the two referenced projects against config aliases or fuzzy-matches against `name` fields.
3. Agent auto-detects archetype: "patterns", "transfer", "same skill" keywords → `complexity`.
4. Agent detects `direction: one-way` (no bidirectional indicator) and `strength: potential` (no active/theoretical keywords).
5. Agent invokes the bridge capture workflow and writes a YAML file in the configured bridges directory.
6. Agent responds with the one-line capture confirmation: `🔗 Bridge #N: 🔬 <src> → <tgt> (complexity) — <short desc>`.
7. Agent resumes the conversation immediately after the one-line response.

## Failure Conditions

- Agent asks the user to re-issue the request in `/bridge` syntax.
- Agent recognises the bridge but does not write a YAML file.
- Agent assigns the wrong archetype (e.g., `knowledge` instead of `complexity`).
- Agent adds multiple paragraphs of explanation around the capture response.
- Agent fails to resolve project names to aliases and aborts.
- Agent does not resume the conversation after the bridge capture.
