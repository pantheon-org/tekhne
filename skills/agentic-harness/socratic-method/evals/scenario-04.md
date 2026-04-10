# Scenario 04: Scope Creep Prevention

## User Prompt

A developer says:

> "I need to refactor this codebase. It's a mess and hard to work with."

Use the socratic-method skill before touching any code.

The codebase is a 40k-line TypeScript monorepo. "Mess" is undefined. The team has a release
in two weeks. The developer has full autonomy but limited time. No tests exist.

## Expected Behavior

- Activate socratic-method questioning mode without proposing a refactoring plan or architectural changes
- Phase 1 must establish what "mess" and "hard to work with" mean concretely (naming? coupling? no tests? inconsistent patterns? slow CI?)
- Phase 2 must probe who is affected and what specifically is being blocked
- Phase 3 must surface the release deadline as a binding constraint
- Phase 4 hypothetical: "If you could only fix one thing before the release, what would it be?" — to narrow scope
- The synthesis must narrow from "refactor the codebase" to a specific, scoped, achievable goal within the two-week window, with explicit out-of-scope items and no moralising about technical debt

## Output Specification

- A dialogue showing the refinement from a vague scope ("refactor everything") to a specific actionable target (e.g., "extract the auth module into its own file and add type annotations to the five functions the team touches most")
- A final scoped plan with explicit "out of scope for now" items

## Success Criteria

- **No premature refactoring plan**: Does not propose a refactoring plan or architectural changes before completing questioning
- **Phase 1 defines mess concretely**: Phase 1 establishes what 'mess' and 'hard to work with' mean concretely
- **Phase 2 identifies who is blocked**: Phase 2 identifies who is blocked and what specifically is being impeded
- **Phase 3 surfaces deadline constraint**: Phase 3 surfaces the upcoming release deadline as a binding constraint
- **Phase 4 narrows scope**: Phase 4 hypothetical narrows the scope to the highest-value change within the deadline
- **Bounded synthesis with out-of-scope items**: Synthesis produces a specific, bounded goal with explicit out-of-scope items; no moralising about the technical debt

## Failure Conditions

- Proposes a broad refactoring plan or architectural restructure before completing the questioning phases
- Phase 1 accepts "it's a mess" as sufficient and moves on without clarifying what that means concretely
- Never probes who is affected or what specific workflows are blocked
- Does not surface the two-week release deadline as a constraint
- Skips the Phase 4 scoping hypothetical
- Delivers a synthesis that is still vague ("clean up the codebase") rather than a specific, bounded target
