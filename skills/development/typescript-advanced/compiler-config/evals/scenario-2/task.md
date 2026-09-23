# Scenario: Diagnose Slow Type-Checking Before Changing Anything

A mid-sized project's `npx tsc --noEmit` has gone from a few seconds to over a minute. A developer's first instinct is to start excluding directories from `include` at random to see what helps.

Produce a short document `perf-diagnosis.md` that:

1. Names the correct first diagnostic command to run before making any configuration change, and explains why guessing at `include`/`exclude` changes first is the wrong order of operations.
2. Lists at least three things that command's output would reveal that explain a type-checking slowdown (e.g. instantiation count, files checked, memory usage).
3. Given the diagnosis points to excessive type instantiations from a deeply recursive conditional type used across the codebase, recommends at least two concrete configuration or code-level responses (not just "delete the type") — for example project references, narrowing `include`, or capping recursion.
4. Explicitly states what NOT to do: silently disabling `strict` or a strictness flag to "speed things up".

## Output Specification

Produce a single file `perf-diagnosis.md` containing the diagnostic command, the reasoning for using it first, the interpretation of what it reveals, and the recommended responses.
