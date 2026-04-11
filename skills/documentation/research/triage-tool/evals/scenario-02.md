# Scenario 02: Triage Tool — Borderline Scope Assessment

## User Prompt

Triage the following tool:

- **Repo**: https://github.com/vstorm-co/memv
- **PyPI**: `memvee`
- **Description**: memv is a Python library for agent memory. It provides predict-calibrate episode extraction, bi-temporal memory validity, and hybrid retrieval (sqlite-vec + FTS5 + RRF). It stores memories in SQLite and surfaces relevant past episodes to feed into agent context at query time.

The tool is not currently in the repo. Add it to the research collection with an appropriate scope assessment.

## Expected Behavior

1. Evaluate the tool's scope against in scope / borderline / out of scope criteria before creating `references/<slug>.md`
2. Classify the tool as `borderline` — not simply `in scope` or `out of scope`
3. Document the out-of-scope aspect: memv's primary function is long-term memory storage (episode extraction, bi-temporal validity), which is primarily an agentic-memory concern
4. Document the in-scope aspect: memv surfaces retrieved episodes into agent context at query time (retrieval-augmented context injection), which overlaps with this repo's scope
5. Describe the context injection / retrieval-to-context pathway specifically in the architecture overview or open questions section — not just the storage layer
6. Create `references/<slug>.md` despite the borderline classification — do not silently skip a borderline tool
7. Include `scope-fit: 'borderline — <explanation>'` in the REVIEWED.md detailed section (not a binary in/out classification)
8. Set the REVIEWED.md `disposition` to `pending`, leaving the promotion decision to the user
9. Explicitly inform the user of the borderline classification and explain why

## Success Criteria

- **Scope assessed before writing the summary**: The agent evaluates the tool's scope against the in scope / borderline / out of scope criteria before creating references/<slug>.md.
- **Tool classified as borderline, not in scope or out of scope**: The scope-fit field in the reference file and/or REVIEWED.md shows 'borderline'. It is not classified as simply 'in scope' or 'out of scope'.
- **Borderline reasoning explains the out-of-scope aspect**: The reasoning notes that memv's primary function is long-term memory storage (episode extraction, bi-temporal validity) — which is primarily an agentic-memory concern.
- **Borderline reasoning explains the in-scope aspect**: The reasoning notes that memv surfaces retrieved episodes into agent context at query time (retrieval-augmented context injection), which overlaps with this repo's scope.
- **Context-layer overlap specifically described in the reference file**: The architecture overview or open questions section in references/<slug>.md specifically addresses the context injection / retrieval-to-context pathway, not just the storage layer.
- **references/<slug>.md is created despite borderline classification**: The agent creates a reference summary even though the tool is borderline — a borderline result is triaged, not silently skipped.
- **REVIEWED.md scope-fit field shows borderline with reason**: The detailed REVIEWED.md section includes scope-fit: 'borderline — <explanation>' rather than a binary in/out classification.
- **REVIEWED.md disposition is pending, not out of scope**: The disposition field in REVIEWED.md is 'pending', leaving the promotion decision to the user rather than auto-rejecting.
- **User is informed of the borderline classification**: The agent explicitly tells the user that the tool is borderline and explains why.
- **Tool is not rejected without assessing the context layer**: The agent does not dismiss the tool as out of scope based solely on its memory-storage focus without first examining whether it has a context-injection component.

## Failure Conditions

- Agent writes the reference file before evaluating scope
- Agent classifies the tool as `in scope` or `out of scope` instead of `borderline`
- Agent notes the borderline classification but only explains the out-of-scope aspect
- Agent notes the borderline classification but only explains the in-scope aspect
- Agent describes only the storage layer and does not address the context injection pathway
- Agent skips creating `references/<slug>.md` due to the borderline classification
- Agent sets `scope-fit` to a binary value in the REVIEWED.md detailed section
- Agent sets `disposition` to something other than `pending`
- Agent does not inform the user of the borderline classification
- Agent rejects the tool as out of scope without examining the context-injection component
