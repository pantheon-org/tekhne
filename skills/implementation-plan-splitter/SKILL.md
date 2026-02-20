---
name: implementation-plan-splitter
description: Split large implementation plan documents into digestible, hierarchical structures with descriptive names. Use when refactoring monolithic planning docs, organizing phase documentation, or creating contributor-friendly task breakdowns. Triggers: "split this plan", "organize phases", "break down implementation docs", "create task hierarchy".
---

# Implementation Plan Splitter

## Decision Points

```
┌─────────────────────────────────────────────────────────────┐
│                    WHAT IS YOUR SOURCE?                      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
              ┌───────────────────────────────┐
              │  Flat markdown phase files?   │
              │  (phase-1.md, phase-2.md)     │
              └───────────────────────────────┘
                    │ YES              │ NO
                    ▼                  ▼
        ┌─────────────────┐   ┌───────────────────────┐
        │  Manual Split   │   │  JSON plan definition? │
        │  (Steps 1-8)    │   │  (plan.json file)     │
        └─────────────────┘   └───────────────────────┘
                                      │ YES
                                      ▼
                            ┌─────────────────────┐
                            │  Use generate-      │
                            │  structure.ts       │
                            └─────────────────────┘
                              │
                              ▼
                    ┌─────────────────────────┐
                    │  Structure exists and   │
                    │  needs validation?      │
                    └─────────────────────────┘
                              │ YES
                              ▼
                    ┌─────────────────────────┐
                    │  Run validate-          │
                    │  structure.ts           │
                    └─────────────────────────┘
```

**Scenario Guide:**

| Source Type | Approach | Tool Usage |
|-------------|----------|------------|
| Flat .md phase files | Manual workflow (Steps 1-8) | Use validate-structure.ts after split |
| JSON plan definition | Automation | Run generate-structure.ts, then validate |
| Existing structure | Validation only | Run validate-structure.ts |

**Edge Cases:**

| Scenario | Action |
|----------|--------|
| Source has mixed phase/step naming | Normalize to one convention first (all phases OR all steps) |
| Existing structure conflicts with plan | Create new structure, migrate content, delete old |
| Circular dependencies detected | Flatten to sequential order, document the constraint |
| Phase file contains both activities and steps | Split into separate directories per type |
| Items reference non-existent parents | Create placeholder parent or renumber children |

## Resource Usage

**Resources are ONLY loaded when needed. Do NOT load all resources by default.**

### MANDATORY Loading Triggers

**ALWAYS load `scripts/validate-structure.ts` when:**
- After completing manual split (Step 8 complete)
- Before marking task as complete
- When user asks "validate this structure"

**ALWAYS load `scripts/generate-structure.ts` when:**
- User provides a JSON plan file
- User says "generate from JSON" or "use the automation"

### CONDITIONAL Loading

**Load `templates/*.yaml` ONLY when:**
- Using generate-structure.ts automation
- Need to customize generated output

**Load `schemas/*.json` ONLY when:**
- Doing programmatic validation outside the scripts
- Debugging validation errors

### DO NOT Load

- Scripts, schemas, or templates when doing manual splitting
- Templates when only validating existing structure
- Schemas when not debugging validation issues

## Expert Heuristics

### When to Flatten vs Subdivide

**Flatten when:**
- Item has <3 children (merge with sibling or parent)
- Children are trivially related (no cognitive load benefit from grouping)
- Navigation depth would exceed 4 levels
- All children fit comfortably on one README screen (<20 lines)

**Subdivide when:**
- Item has >10 children (cognitive overload)
- Children cluster into distinct semantic groups (setup vs execution vs cleanup)
- Parallel work streams exist (frontend/backend can proceed independently)
- Different team ownership (devops vs developers vs qa)

**Rule of thumb:** 3-7 items per group is the sweet spot. Below 3 feels sparse; above 10 forces scanning.

### Naming Conventions That Scale

**Descriptive names age well:**
```
step-1-extract-movement-logic/     # GOOD: survives refactors
step-1-refactor-game-code/         # BAD: vague, becomes meaningless
```

**Verb-first for actions, noun-first for outcomes:**
```
activity-1-analyze-codebase/       # Action-oriented
activity-1-analysis-complete/      # Outcome-oriented (preferred for tracking)
```

**Avoid temporal names:**
```
step-1-initial-setup/              # BAD: "initial" becomes misleading later
step-1-project-bootstrap/          # GOOD: timeless
```

### Handling Legacy Conventions

When source uses inconsistent naming:
1. Extract the semantic meaning, not the literal text
2. Normalize to outcome-based names
3. Document the mapping for traceability
4. Keep original numbering for cross-referencing

## Pre-Split Verification

Before splitting, verify:

1. **Source type detected** (use decision tree above)
2. **Natural groupings identified** (phases → activities/steps → groups)
3. **Numbering convention detected** (step-1.1, step-1.2 belong together)

**Ask yourself:**
- Does each phase/step have 3-10 child items? (Fewer = flatten; more = subdivide)
- Are there natural breaks in the content (headings, numbered sections)?
- Will a reader understand the hierarchy without opening every file?

## Workflow

**Use the heuristics above for naming/grouping decisions. Apply these steps in order:**

### 1. Structure

```
docs/refactoring/phases/
├── phase-{number}-{name}/
│   ├── README.md
│   └── activities/              # OR steps/
│       ├── README.md
│       └── activity-{number}-{description}/
│           ├── README.md
│           └── activity-{number}.{sub}-*.md
```

Max depth: 4 levels (phase → activities/steps → group → leaf). Deeper = flatten.

### 2. Content

Each leaf file: title, description, checklist, acceptance criteria, status.

### 3. READMEs

Every non-leaf directory needs README explaining purpose + listing children. 3 lines minimum.

### 4. Names

Format: `{type}-{number}-{kebab-description}`. Use naming heuristics from above.

### 5. Grouping

Items with same prefix (1.x, 2.x) go in same parent. See grouping heuristics.

### 6. Links

Update README links after restructuring. Verify with validate-structure.ts.

### 7. Cleanup

Remove flat files only after validation passes.

## Anti-Patterns

### NEVER Use Numeric-Only Directory Names

```
step-1/           # NEVER
activity-2/       # NEVER
```

**WHY:** Contributors cannot understand intent without opening files. Numeric names create navigation friction and force users to drill into directories just to learn what they contain. Descriptive names enable rapid comprehension at a glance.

### NEVER Mix Different Parent Groups

```
step-1-extract/
  ├── step-1.1-extract-movement.md
  ├── step-1.2-extract-collision.md
  └── step-2.1-create-libs.md      # NEVER - wrong parent group

step-2-create-structure/
  └── step-2.2-configure-nx.md     # Orphaned - should be with 2.1
```

**WHY:** Mixing groups breaks the mental model. Users expect `step-1.x` items together. Cross-pollinating directories forces users to search multiple locations for related work.

### NEVER Create Deeply Nested Structures

```
phase-1/
  └── activities/
      └── activity-1/
          └── steps/
              └── step-1/
                  └── sub-steps/     # NEVER - too deep
```

**WHY:** Maximum 4 levels deep (phase → activities/steps → group → leaf files). Deeper nesting increases cognitive load and makes navigation tedious. Flatten intermediate levels if needed.

### NEVER Skip README Files

**WHY:** Every directory must explain its purpose. Missing READMEs create dead ends where users must explore blindly. A 3-line README is better than none.

### NEVER Use Generic Directory Names

```
step-1-stuff/        # NEVER
step-2-misc/         # NEVER
step-3-various/      # NEVER
activity-1-things/   # NEVER
```

**WHY:** Generic names are worse than numeric names - they suggest organization but provide zero information. A name like "misc" forces users to open files just to learn what "miscellaneous" means in this context. Either use a specific description or keep the numeric name.

## Validation Checklist

Before marking complete:

- [ ] Every phase has its own directory
- [ ] Every directory has a README.md
- [ ] All step/activity directories have descriptive names
- [ ] Related items are grouped (1.1, 1.2 together; not 1.x with 2.x)
- [ ] All README links resolve correctly
- [ ] Old flat files removed
- [ ] Each leaf file has: title, description, checklist, acceptance criteria, status

## Example Transformation

**See `references/example-transformation.md`** for before/after structure comparison. Load only when you need a concrete example.

## Error Recovery

### Phase with 50+ Items

**Problem:** One phase has grown unmanageably large.

**Solutions:**
1. **Split by milestone** - Create phase-1a-, phase-1b- prefixes for delivery checkpoints
2. **Extract verticals** - If phase covers multiple features, create feature-based phases
3. **Promote to phase** - If a group of 10+ items is cohesive, it deserves its own phase

**Anti-pattern:** Just adding more nesting. Deep hierarchies with 50 siblings are still unusable.

### Conflicting Naming Conventions

**Problem:** Source mixes step-X.Y and activity-X.Y patterns.

**Solutions:**
1. **Choose dominant pattern** - Whichever has more items, normalize to that
2. **Phase-aware convention** - Use `steps` for implementation phases, `activities` for analysis phases
3. **Document the hybrid** - Add README note explaining the dual convention

**Recovery:** After choosing, run validate-structure.ts to catch stragglers.

### Partial Split (Abandoned Mid-Work)

**Problem:** Previous split left some flat files and some hierarchical directories.

**Diagnosis:**
```bash
# Find flat phase files still in root
ls docs/refactoring/phases/*.md

# Find directories missing READMEs  
find docs/refactoring/phases -type d -exec test ! -f {}/README.md \; -print
```

**Recovery:**
1. Identify which phases are complete (have hierarchical structure)
2. Identify which are still flat (just .md files)
3. Prioritize: complete partial splits before starting new ones
4. Use git to recover: `git diff HEAD~5 --name-only | grep phases/`

### Items Reference Non-Existent Parents

**Problem:** README links to `./step-2.1-foo.md` but file is actually at `./step-2-foo/step-2.1-bar.md`.

**Solutions:**
1. **Renumber children** - Update numbering to match actual parent
2. **Create missing parent** - If children are cohesive, create the group directory
3. **Flatten** - If only 1-2 children, they don't need a parent directory

**Validation:** Run validate-structure.ts to detect broken links automatically.

## Automation

### generate-structure.ts

Generate from JSON plan:
```bash
bun run scripts/generate-structure.ts --plan plan.json
bun run scripts/generate-structure.ts --example  # Show JSON format
```

### validate-structure.ts

Validate existing structure:
```bash
bun run scripts/validate-structure.ts docs/refactoring/phases
# Exit codes: 0 = valid, 1 = invalid
```

Checks: READMEs present, descriptive names, required sections, valid links, proper hierarchy.


