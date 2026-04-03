# Scope Creep Prevention

## Problem Description

A developer says:

> "I need to refactor this codebase. It's a mess and hard to work with."

Use the socratic-method skill before touching any code.

The codebase is a 40k-line TypeScript monorepo. "Mess" is undefined. The team has a release
in two weeks. The developer has full autonomy but limited time. No tests exist.

## Expected Behavior

- Activate socratic-method questioning mode  
- Do NOT immediately propose a refactoring plan or architectural changes
- Phase 1 must establish what "mess" and "hard to work with" mean concretely
  (naming? coupling? no tests? inconsistent patterns? slow CI?)
- Phase 2 must probe: who is affected, and what specifically is being blocked?
- Phase 3 must surface the release deadline as a constraint
- Phase 4 hypothetical: "If you could only fix one thing before the release, what would it be?"
- The synthesis must narrow from "refactor the codebase" to a specific, scoped, achievable goal
  that fits within the two-week window without destabilizing the release

## Output Specification

- A dialogue showing the refinement from a vague scope ("refactor everything") to a specific
  actionable target (e.g. "extract the auth module into its own file and add type annotations
  to the five functions the team touches most")
- A final scoped plan with explicit "out of scope for now" items
