# Scenario 05: Ambiguous Scope, No Clarifying Question First

## User Prompt

"How does the caching layer work?"

(The codebase has three distinct caches: an HTTP response cache, an in-memory query cache, and a CDN edge cache, and the user gave no further detail about which one they mean.)

## Expected Behavior

1. The agent recognizes the question is ambiguous — three plausible referents for "the caching layer" — but does not stop to ask a clarifying question first.
2. The agent states its best-guess interpretation explicitly and briefly (e.g. "I'll assume you mean the in-memory query cache, since that's what's usually meant by 'the caching layer' in this codebase's own terminology — let me know if you meant one of the others") before proceeding.
3. The agent proceeds to explore and explain the chosen interpretation using the normal Explain-mode flow, rather than blocking on user confirmation.
4. If exploration surfaces that the question spans more than one cache in a way that matters (e.g. they share an invalidation path), the agent says so and broadens scope rather than forcing a single-cache answer that misses the shared mechanism.

## Success Criteria

- The agent states which interpretation it picked and why, in one or two sentences, before diving into the explanation.
- The agent does not ask the user "which cache do you mean?" and wait for a reply before doing any exploration.
- The explanation that follows is coherent and grounded in the actual code for the chosen (or broadened) interpretation.

## Failure Conditions

- The agent's first response is a clarifying question with no exploration or best-guess interpretation offered.
- The agent silently picks one cache without flagging that the question was ambiguous, leaving the user unable to tell whether the agent noticed the ambiguity.
- The agent tries to explain all three caches in equal, undifferentiated depth without picking a primary interpretation, producing an unfocused answer.
