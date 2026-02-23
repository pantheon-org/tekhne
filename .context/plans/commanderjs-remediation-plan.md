---
plan_date: 2026-02-23
skill_name: commanderjs
source_audit: .context/audits/commanderjs-audit-2026-02-22.md
---

# Remediation Plan: commanderjs

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 90/120 | 102/120 |
| **Grade** | C+ | B+ |
| **Priority** | Medium | - |
| **Effort** | Medium (S-M) | - |

**Focus Areas**: Pattern recognition (D7), Progressive disclosure (D5), Anti-pattern quality (D3)

## Critical Issues to Address

| Issue | Severity | Dimension | Impact |
| --- | --- | --- | --- |
| Vague trigger phrases | High | D7 (6/10) | Skill may not activate for CLI tasks |
| Large SKILL.md (201 lines, 6 refs) | Medium | D5 (10/15) | Navigation complexity |
| Moderate anti-pattern quality | Medium | D3 (10/15) | Common CLI mistakes not prevented |

## Detailed Remediation Steps

### Phase 1: Pattern Recognition (D7) - Priority: High

**Target**: Increase from 6/10 to 10/10 (+4 points)

#### Step 1.1: Expand frontmatter description

**File**: `skills/commanderjs/SKILL.md`

```yaml
---
name: commanderjs
description: |
  Complete Commander.js CLI framework guidance. Use when: building CLI tools,
  parsing command-line arguments, implementing subcommands, handling options/flags,
  creating interactive CLIs, migrating from yargs/meow, or adding --help/--version.
  
  Keywords: Commander.js, CLI, command-line, arguments, options, flags, subcommands,
  action handlers, version, help text, TypeScript, program, parseAsync, opts, args,
  variadic, required options, default values, custom help, error handling
---
```

#### Step 1.2: Add comprehensive "Use When" section

```markdown
## Use When

- "Create a CLI tool with Commander.js"
- "Parse command-line arguments"
- "Add subcommands to my CLI"
- "Handle --help and --version"
- "Migrate from yargs to Commander"
- "TypeScript CLI with Commander"
- "Variadic arguments in CLI"
- NOT for: Browser applications, non-CLI tools
```

---

### Phase 2: Progressive Disclosure (D5) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 2.1: Audit existing references

Current references (6 files) may be sufficient. Evaluate if SKILL.md can be further condensed.

#### Step 2.2: Extract detailed examples to reference

**File**: `skills/commanderjs/references/advanced-patterns.md`

Move complex patterns from SKILL.md:

```markdown
# Advanced Commander.js Patterns

## Async Action Handlers

```ts
program
  .command('fetch')
  .action(async (options) => {
    const data = await fetchData(options.url);
    console.log(data);
  });

// Use parseAsync for async handlers
program.parseAsync();
```

## Custom Help Formatting

[Detailed help customization examples...]

## Error Handling Patterns

[Detailed error handling patterns...]
```

#### Step 2.3: Update SKILL.md hub

Replace detailed content with:

```markdown
## Advanced Patterns

For complex scenarios:

- Async action handlers: [Advanced Patterns](references/advanced-patterns.md)
- Custom help formatting: [Advanced Patterns](references/advanced-patterns.md)
- Error handling: [Advanced Patterns](references/advanced-patterns.md)
```

---

### Phase 3: Anti-Pattern Quality (D3) - Priority: Medium

**Target**: Increase from 10/15 to 14/15 (+4 points)

#### Step 3.1: Add explicit anti-patterns

**File**: `skills/commanderjs/SKILL.md`

```markdown
## Anti-Patterns

### NEVER: Use parse() with async action handlers

WHY: Async operations will not complete before process exits.

BAD:
```ts
program
  .command('fetch')
  .action(async () => {
    const data = await fetch(url); // Never completes!
  });
program.parse(); // Sync parse exits immediately
```

GOOD:
```ts
program
  .command('fetch')
  .action(async () => {
    const data = await fetch(url);
  });
program.parseAsync(); // Waits for async
```

### NEVER: Define options after command()

WHY: Options must be declared before the command they modify.

BAD:
```ts
program
  .command('deploy')
  .action(() => { ... });
program.option('-e, --env <env>'); // Too late!
```

GOOD:
```ts
program
  .option('-e, --env <env>')
  .command('deploy')
  .action(() => { ... });
```

### NEVER: Access opts() before parse()

WHY: Options are not populated until parsing completes.

BAD:
```ts
const options = program.opts(); // Empty!
program.parse();
```

GOOD:
```ts
program.parse();
const options = program.opts(); // Now populated
```

### NEVER: Use process.exit() in action handlers

WHY: Prevents cleanup and async operations from completing.

BAD:
```ts
.action(() => {
  process.exit(1); // Abrupt exit
});
```

GOOD:
```ts
.action(async () => {
  await cleanup();
  process.exitCode = 1; // Graceful exit
});
```
```

---

### Phase 4: Specification Compliance (D4) - Priority: Low

**Target**: Increase from 10/15 to 12/15 (+2 points)

#### Step 4.1: Tighten frontmatter

```yaml
---
name: commanderjs
description: "[from Phase 1]"
version: 1.0.0
author: tekhne
tags: [cli, commander, command-line, typescript, node]
scope: CLI application development with Commander.js
---
```

---

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh commanderjs --json
bunx markdownlint-cli2 "skills/commanderjs/**/*.md"
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D7 Pattern Recognition | Score >= 10/10 |
| D5 Progressive Disclosure | Score >= 14/15 |
| D3 Anti-Pattern Quality | Score >= 14/15 |
| Overall Score | >= 102/120 (B+) |

## Estimated Effort

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Triggers | S | 20 min |
| Phase 2: Disclosure | M | 45 min |
| Phase 3: Anti-patterns | M | 45 min |
| Phase 4: Frontmatter | S | 10 min |
| **Total** | **M** | **2 hours** |

## Dependencies

- None (self-contained skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/commanderjs/
```
