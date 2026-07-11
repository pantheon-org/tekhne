---
name: codebase-alignment
description: >
  Given a user-provided set of coding standards (either as direct text input or as
  a link to a file/URL), systematically review the codebase to identify violations,
  report findings, and optionally remediate them. The model MUST ask the user for
  the standards — never guess or assume them.
---

# Codebase Alignment — Audit a Codebase Against User-Provided Standards

Systematically checks every file in a codebase against human-readable standards
the user supplies, reports violations grouped by file and convention, and offers
to fix them.

## Mindset

**The model MUST NOT guess or assume the standards. This is the core constraint.**
If the user says "align this with our conventions" without spelling them out, stop
and ask. Standards that are assumed are standards that are wrong.

Conventions that cannot be checked at all (subjective design preferences, naming
that depends on context) should be reported as "manual review needed" — do not
pretend an objective check was run when one wasn't.

Fix suggestions are best-effort. When a violation is ambiguous (e.g. a naming
convention that could be fixed multiple ways), flag it for the user rather than
guessing.

## When to Use

- Onboarding a new team member who needs the codebase brought in line with docs
- After adopting a new style guide or coding standard
- Before a major release or code review sprint
- When migrating a codebase and need to verify alignment with target conventions
- When the user says "audit this codebase", "align with our standards", "check our conventions", "does this follow our style guide"

If the user already has automated linting catching everything, prefer `standards-to-tooling` to codify the rules instead — this skill is for ad-hoc or partially-automated conventions.

## When NOT to Use

- The user already has automated linting/formatting catching everything (point
  them to `standards-to-tooling` instead to codify the rules)
- The user has not provided standards and refuses to (do not proceed)
- The task is a simple mechanical refactor with no judgment calls
- Standards are being changed mid-review — ask to finalise them first
- The codebase is entirely generated or compiled output (vendored deps, build artifacts)
- Standards reference a language or framework not present in this project — align tooling first

## Workflow

### Phase 1 — MUST: Collect the Standards

Before any code review begins, the model MUST explicitly ask the user for the
standards. Do not skip this step. Do not assume a default set of standards.

Present the user with two options using the `Question` tool:

1. **Direct input** — User types or pastes their standards into the chat
2. **File or URL** — User provides a local file path or a URL to a document
   containing the standards

Collect ALL standards before proceeding. If the user adds standards incrementally
during the review, pause and re-scan affected files.

*Anti-pattern*: "I'll check against standard JS conventions" without asking first.
*Anti-pattern*: Accepting "just use the Airbnb style guide" without confirming.
*Always confirm*, even for well-known guides — the user may have local overrides.

### Phase 2 — Parse the Standards

Convert the user's human-readable standards into a structured checklist:

| Standard | Category | Check method |
|----------|----------|-------------|
| "Use `const` not `let` for invariants" | Style | Grep for `let` in function scope |
| "Files must be kebab-case" | Naming | Glob check: no PascalCase/camelCase filenames |
| "No `console.log` in production code" | Style | Grep for `console.log` |
| "Vue SFCs must use `<script setup>`" | Framework | Grep for `export default {` in `.vue` files |
| "API error responses follow RFC 9457" | Contract | Manual review (cannot auto-check fully) |

For each standard, classify:
- **Auto-checkable** — can be verified with grep, glob, or lint rules
- **Semi-auto** — can be partially checked but needs human judgment
- **Manual only** — subjective, context-dependent

### Phase 3 — Scan the Codebase

For each auto-checkable standard, run the appropriate scan:

```bash
# Example: check for `var` usage
rg --include '*.{js,ts,vue}' '\bvar\b'

# Example: check for console.log
rg --include '*.{js,ts,vue}' 'console\.(log|dir|table)\('

# Example: check filename casing (replace <source-dir> with actual path)
find <source-dir> -name '*[A-Z]*' -not -path '*/node_modules/*'

# Example: check for Options API in Vue SFCs
rg 'export default \{' --include '*.vue'
```

For semi-auto standards, produce a shortlist for manual review.

For manual-only standards, list them separately as "requires human review".

### Phase 4 — Report Findings

Present the results grouped by standard, showing:

```
## Alignment Report

### [AUTO] Use const over let
  ✅ 142/150 files use const for invariants
  ⚠️  8 files use let where const would work
  → src/utils/format.js:12, src/composables/useAuth.js:45
  → src/components/StarCard.vue:88

### [AUTO] Kebab-case filenames
  ✅ All 98 source files comply

### [SEMI] No console.log
  ⚠️  3 occurrences found (may be intentional)
  → src/utils/debug.js:22 (debug utility — possibly intentional)
  → src/store/index.js:104 (error logging — verify)

### [MANUAL] Error responses follow RFC 9457
  🔍 4 API handler files need human review:
  → src/api/handlers/*.js

Summary: 12 violations across 8 files, 2 standards fully compliant,
1 standard needs manual review.
```

### Phase 5 — Offer Remediation (Optional)

After presenting the report, ask the user:

> "Would you like me to fix the auto-fixable violations? [Yes] [No] [Pick files]"

For each violation, apply the minimal fix. When multiple interpretations exist
(e.g. renaming a variable), flag it for confirmation rather than guessing.

## Rules of Engagement

### MUST ask for the standards before doing anything else

WHY: Guessing the standards produces wrong results and erodes trust. The user's
mental model of "good code" is the only source of truth.

BAD: "Let me check for common issues" without asking what standards to use.
GOOD: "What standards should I check against? You can type them now or share a link."

### MUST present findings before offering to fix

WHY: The user needs to see what changed before deciding whether to apply fixes.
Some violations are intentional, and auto-fixing without review introduces bugs.

BAD: "Found 12 let violations — fixed them all."
GOOD: Report first, ask to fix second.

### NEVER report a violation without a file path and line number

WHY: Untraceable warnings are noise — the user cannot act on them.

BAD: "Some files use var instead of const."
GOOD: "web/src/utils/old-script.js:15 uses `var count = 0`."

### NEVER run destructive edits without confirmation

WHY: Bulk auto-fix can introduce bugs. Always confirm before writing changes.

BAD: Applying regex replacements across 50 files without asking.
GOOD: "Found 12 fixable violations across 8 files. Apply fixes?"

### NEVER claim a standard is fully checked when only partially checked

WHY: False confidence in coverage is worse than no coverage — the user acts on
assurance that may be wrong.

BAD: "Naming conventions checked: ✅" when only filenames were checked but not
variable names.
GOOD: Clearly label each standard's check coverage.

## Troubleshooting

| Situation | Response |
|-----------|----------|
| User provides vague standards ("make it cleaner") | Ask for specifics: "Can you give me 2-3 concrete rules? For example, naming conventions, preferred patterns, or things to avoid." |
| User provides a link that's inaccessible | Report the error, ask for direct input instead |
| Standards conflict with each other | Flag the conflict: "Standard A says X but Standard B implies Y — which takes precedence?" |
| Codebase is very large (>1000 files) | Ask the user to scope: "Should I check the whole codebase or a specific directory?" |
| User adds standards mid-review | Pause, acknowledge, re-scan affected files, update the report incrementally |
| A standard has zero violations | Show it as ✅ in the report — compliance is useful signal too |

## Verification

After completing the review:

- Does every standard from Phase 2 appear in the Phase 4 report (either ✅ or ⚠️)?
- Does every violation include a file path and line number?
- Were fixes applied only after explicit user confirmation?
- Are manual-only standards clearly distinguished from auto-checked ones?
- If the user provided standards incrementally, was the report updated to reflect them?

## References

| Tool | When to use |
|------|-------------|
| `rg` (ripgrep) | Fast pattern search across the codebase |
| `glob` (tool) | Filename pattern matching |
| `Read` | Inspect specific files for context-dependent standards |
| `standards-to-tooling` | After alignment, to codify rules as lint config |
| `pr-author` | When alignment results should become a PR |
| `vault-capture` | Persist alignment decisions (e.g. "we allow console.warn") |
