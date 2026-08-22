# Scenario 02: A Dependency Bump Tempts a Caller-List-Only Answer

## User Prompt

I bumped the JSON schema validator dependency to its next major version — what's the blast radius?

## Expected Behavior

1. The agent does not stop at listing the direct callers of the validator; it treats that lookup (via `tokensave_callers`/`tokensave_impact` or `code-review-graph`'s impact tools) as a first step, not
   the deliverable.
2. The agent checks the pinned version and any local patch of the dependency: `tokensave_context`/`tokensave_search` first if the relevant code is inside this repo's own graph (e.g. a vendored copy or
   a local patch file), with raw `Read` used only for source genuinely outside that graph (e.g. actual `node_modules` contents).
3. The agent follows what a caller sweep misses: whether the new major version changes the wire format/JSON shape the validator produces or expects, and what reads that shape several hops downstream.
4. Every caller, API, or behavior change cited is one that was actually found — a search that turns up nothing is reported as such, not silently dropped or replaced with a guess.

## Success Criteria

- The response explicitly goes beyond "here are the callers" into what a caller list can't show (format changes, downstream consumers, version-specific behavior).
- Any lookup of in-repo code (vendored copies, local patches) is attributed to tokensave tools, not raw Read, unless the source is explicitly outside this repo's graph.
- At least one citation of a real `file:line` or a real "searched X, found nothing" statement appears.
- No caller, API, or behavior is asserted without it having actually been found by a described search.

## Failure Conditions

- The response is essentially just a caller list with no discussion of what changed underneath (wire format, error shape, downstream consumers).
- The agent describes using Read/Grep on in-repo files where tokensave_context/tokensave_search would have applied, without explaining why tokensave wasn't sufficient.
- A caller, API signature, or downstream consumer is named that the response gives no indication of having actually located.
- A search that (implicitly or explicitly) found nothing is silently omitted rather than reported as a cleared/checked item.
