# Anti-Pattern Examples - Extracted from A-Grade Skills

This document contains real anti-patterns from the top 5 A-grade skills (opencode-config, nx-executors, agents-md, biome-complete, nx-generators) to serve as reference examples when creating anti-pattern sections for other skills.

---

## Configuration & Setup Anti-Patterns

### From: opencode-config (111/120 - A)

**NEVER commit API keys directly in config**
- **WHY**: secret leakage through source control history is irreversible.
- **BAD**: `"apiKey": "sk-..."`.
- **GOOD**: `"baseEnv": "OPENAI_API_KEY"`.

**NEVER use broad filesystem or shell permissions by default**
- **WHY**: permissive defaults increase blast radius of mistakes.
- **BAD**: root-level read/write and unrestricted shell.
- **GOOD**: scoped paths and explicit command allowlists.

**NEVER use ambiguous model names**
- **WHY**: providers may resolve generic model aliases differently.
- **BAD**: `"model": "gpt-4"`.
- **GOOD**: provider-qualified or exact model identifiers.

**NEVER skip verification after permission changes**
- **WHY**: permission regressions are often silent until runtime.
- **BAD**: edit-and-commit without test.
- **GOOD**: run `opencode run "test"` and validate behavior.

---

## Framework Integration Anti-Patterns

### From: nx-executors (111/120 - A)

**NEVER put business logic directly inside executor entrypoints**
- **WHY**: bloated executors are hard to test and reuse.
- **BAD**: 200 lines of transformation logic in `executor.ts`.
- **GOOD**: delegate to composable library functions.

**NEVER use relative executor references in targets**
- **WHY**: relative paths are fragile across workspace changes.
- **BAD**: `"executor": "../../tools/executors:task"`.
- **GOOD**: `"executor": "@scope/tools:task"`.

**NEVER skip schema validation details**
- **WHY**: weak schemas produce invalid runtime invocations.
- **BAD**: untyped options with no required fields.
- **GOOD**: explicit `type`, `required`, defaults, and descriptions.

**NEVER omit outputs and dependencies semantics for cacheable work**
- **WHY**: Nx cache quality depends on deterministic inputs and outputs.
- **BAD**: no outputs and implicit file writes.
- **GOOD**: declare outputs and stable execution paths.

**NEVER block executor runs with synchronous file I/O in hot paths**
- **WHY**: sync I/O hurts parallel throughput.
- **BAD**: `fs.readFileSync` in main execution flow.
- **GOOD**: `await fs.promises.readFile` with structured error handling.

---

## Documentation Anti-Patterns

### From: agents-md (110/120 - A)

**NEVER assume a technology stack without discovery**
- **WHY**: incorrect assumptions produce unusable instructions.
- **BAD**: generate React/Jest guidance without evidence.
- **GOOD**: run discovery commands and map docs to detected stack.

**NEVER dump encyclopedic content into root AGENTS.md**
- **WHY**: oversized docs increase token cost and reduce usability.
- **BAD**: embed full framework manuals.
- **GOOD**: keep root concise and link to scoped files/references.

**NEVER duplicate the same instructions across root and sub-files**
- **WHY**: duplication creates drift and maintenance overhead.
- **BAD**: copy/paste identical conventions in every file.
- **GOOD**: keep universal rules at root and package-specific rules locally.

**NEVER provide unverified commands**
- **WHY**: broken commands erode trust and block contributors.
- **BAD**: include hypothetical commands.
- **GOOD**: include only validated copy-paste commands.

---

## Linting & Formatting Anti-Patterns

### From: biome-complete (110/120 - A)

**NEVER run Biome and ESLint on the same files**
- **WHY**: competing rules create contradictory output and noisy reviews.
- **BAD**: ESLint and Biome both lint `src/**/*.ts`.
- **GOOD**: route TS linting and formatting through Biome only.
- **CONSEQUENCE**: duplicate diagnostics and unstable CI outcomes.

**NEVER run Prettier and Biome formatter on the same files**
- **WHY**: different formatting models cause churn in every commit.
- **BAD**: `prettier --write .` and `biome format . --write` in the same pipeline.
- **GOOD**: keep only `biome format . --write` for supported files.
- **CONSEQUENCE**: constant formatting diffs and merge friction.

**NEVER skip `biome.json` customization after init**
- **WHY**: defaults may not match repository conventions.
- **BAD**: commit default config without reviewing formatter/linter settings.
- **GOOD**: define formatter width, linter domains, and VCS ignores explicitly.
- **CONSEQUENCE**: inconsistent style and avoidable lint regressions.

**NEVER blanket-ignore diagnostics to get green CI**
- **WHY**: suppressed warnings compound into critical failures over time.
- **BAD**: disable entire rule categories to pass checks.
- **GOOD**: fix root causes or use targeted ignores with inline comments.
- **CONSEQUENCE**: accumulating technical debt invisible to CI.

---

## Code Generation Anti-Patterns

### From: nx-generators (108/120 - A)

**NEVER modify files outside the Tree API**
- **WHY**: direct filesystem writes bypass dry-run and change tracking.
- **BAD**: `writeFileSync("libs/my-lib/src/index.ts", content)`.
- **GOOD**: `tree.write("libs/my-lib/src/index.ts", content)`.

**NEVER hardcode project paths in generator logic**
- **WHY**: brittle paths fail when workspace layout evolves.
- **BAD**: fixed `libs/my-lib/...` writes.
- **GOOD**: derive paths from schema/context helpers.

**NEVER skip schema validation and typed options**
- **WHY**: invalid inputs fail late with unclear errors.
- **BAD**: `schema: any` and no guardrails.
- **GOOD**: typed schema + required fields + runtime guards.

**NEVER generate across project boundaries without explicit checks**
- **WHY**: hidden boundary violations can introduce circular dependencies.
- **BAD**: generator in one scope writing imports into disallowed scopes.
- **GOOD**: verify tags and dependency direction first.

**NEVER mutate project configuration blindly**
- **WHY**: naive updates remove existing targets/tags.
- **BAD**: overwrite full `project.json`.
- **GOOD**: read, merge, and update only intended keys.

---

## Pattern Analysis

### Common Structural Elements

All A-grade anti-patterns share:
1. **Specific titles**: "NEVER [verb] [specific thing]" not "Avoid bad practices"
2. **Concise WHY**: Single sentence root cause, not essay
3. **Concrete BAD**: Real code/command showing mistake
4. **Concrete GOOD**: Real code/command showing correct alternative
5. **Domain relevance**: Examples specific to skill's technical domain

### Severity Categories

**High-impact patterns** (security, data loss, irreversible):
- Secret leakage (opencode-config)
- Unverified commands (agents-md)
- Blind configuration overwrites (nx-generators)

**Medium-impact patterns** (maintainability, correctness):
- Business logic in wrong layer (nx-executors)
- Competing tools on same files (biome-complete)
- Hardcoded paths (nx-generators)

**Low-impact patterns** (performance, ergonomics):
- Synchronous I/O in hot paths (nx-executors)
- Ambiguous model names (opencode-config)
- Missing schema validation (nx-executors)

### Tone Patterns

**Authoritative** (nx-executors, nx-generators):
- Declarative statements
- Framework-specific terminology
- Assumes technical competence

**Cautionary** (opencode-config, agents-md):
- Risk-focused language
- Emphasizes irreversibility or trust
- Security/reliability concerns

**Pragmatic** (biome-complete):
- Efficiency-focused
- Consequence-driven
- Tool interaction conflicts

---

## Usage Guide

### When Creating New Anti-Patterns

1. **Identify domain**: Find examples above from similar skill type
2. **Match severity**: Use similar WHY/CONSEQUENCE framing
3. **Adapt syntax**: Replace code examples with skill-specific equivalents
4. **Maintain structure**: Keep WHY/BAD/GOOD format consistent
5. **Verify specificity**: Ensure titles are action-focused, not vague

### Template Selection

- **Minimal template** (3 patterns): Use examples from 1-2 categories above
- **Comprehensive template** (5+ patterns): Use examples from 3+ categories above

### Quality Checks

Compare your anti-patterns to these exemplars:
- [ ] WHY is 1 sentence (not paragraph)
- [ ] BAD shows actual code/command (not description)
- [ ] GOOD shows actual code/command (not description)
- [ ] Title uses "NEVER [verb]" format
- [ ] Examples are domain-specific (not generic "best practices")

---

## Extracted: 2026-03-03
## Source Skills: opencode-config, nx-executors, agents-md, biome-complete, nx-generators (all A-grade)
## Purpose: Reference for Phase 5 remediation work
