# Scenario 04: Placement / Ownership Question

## User Prompt

"Where should the new invoice-retry validation logic live? Is the billing package the right owner for it, or does it belong closer to the payment gateway client?"

## Expected Behavior

1. The agent recognizes this as a placement/ownership/layering question, not a walkthrough of existing behavior — still handled by this skill's Explain mode, not treated as a request to write code.
2. The agent treats the question as narrow enough to explore directly (existing package boundaries, existing validation conventions) rather than spawning a large parallel-explorer fan-out, unless exploration reveals the boundary genuinely spans several subsystems.
3. The agent explores using a code-graph MCP's context-query tool when one is connected (otherwise Glob/Grep/Read), tracing where similar validation logic already lives and what each candidate package currently owns.
4. The answer states a concrete placement recommendation (not "it depends" alone), grounded in the actual boundaries and conventions found in the codebase, and gives the reasoning tying the recommendation to what currently lives in each candidate location.
5. The response follows the Overview / Key Concepts / How It Works / Where Things Live / Gotchas shape, adapted — a placement question doesn't need every section, but the reasoning should reference concrete files/packages, not abstract principles alone.

## Success Criteria

- A specific placement recommendation is given, not a non-answer.
- The recommendation is grounded in the actual current package boundaries and existing validation conventions found via exploration, with specific file/package references.
- Exploration is attributed to a code-graph tool (when one is connected) or explicit file search, not asserted without evidence.

## Failure Conditions

- The agent answers purely from general software-design principles ("keep validation close to the data") without checking how this specific codebase already organizes similar logic.
- The agent treats the question as a request to implement the validation logic itself rather than answer where it belongs.
- No concrete recommendation is given — the response only lists tradeoffs and leaves the decision entirely to the user.
