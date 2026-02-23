---
plan_date: 2026-02-23
skill_name: bun-development
source_audit: .context/audits/bun-development-audit-2026-02-22.md
---

# Remediation Plan: bun-development

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 85/120 | 102/120 |
| **Grade** | C | B+ |
| **Priority** | High | - |
| **Effort** | Medium (S) | - |

**Focus Areas**: Anti-pattern quality (D3), Practical usability (D8), Pattern recognition (D7)

## Critical Issues to Address

| Issue | Severity | Dimension | Impact |
| --- | --- | --- | --- |
| Weak anti-pattern examples | High | D3 (8/15) | Agents may repeat common mistakes |
| Missing executable commands | High | D8 (8/15) | Users cannot verify guidance |
| Vague trigger phrases | Medium | D7 (6/10) | Skill may not activate when needed |
| Imprecise frontmatter | Medium | D4 (10/15) | Routing ambiguity |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 1.1: Add explicit anti-patterns to SKILL.md

**File**: `skills/bun-development/SKILL.md`

Add a dedicated Anti-Patterns section with NEVER/WHY/BAD-GOOD format:

````markdown
## Anti-Patterns

### NEVER: Use Bun.serve with synchronous file reads

WHY: Blocks the event loop and defeats Bun's performance advantages.

BAD:
```ts
Bun.serve({
  fetch(req) {
    const data = fs.readFileSync('./data.json'); // Blocking!
    return new Response(data);
  }
});
```

GOOD:
```ts
Bun.serve({
  async fetch(req) {
    const file = Bun.file('./data.json');
    return new Response(file); // Non-blocking streaming
  }
});
```

### NEVER: Import Node crypto when Bun.crypto is available

WHY: Bun's native crypto is 3-5x faster for common operations.

BAD:
```ts
import { createHash } from 'crypto'; // Node fallback
```

GOOD:
```ts
const hash = new Bun.CryptoHasher('sha256');
hash.update(data);
```
````

#### Step 1.2: Add repository-specific risk tie-ins

Document how anti-patterns relate to this repository's conventions (e.g., `Bun.file` vs `fs` in the codebase).

---

### Phase 2: Practical Usability (D8) - Priority: High

**Target**: Increase from 8/15 to 13/15 (+5 points)

#### Step 2.1: Add copy/paste command examples

**File**: `skills/bun-development/SKILL.md`

Add a Quick Commands section:

```markdown
## Quick Commands

### Initialize Bun project
```bash
bun init
bun add -d typescript
```

### Run with hot reload
```bash
bun --hot src/index.ts
```

### Test with coverage
```bash
bun test --coverage
```

### Bundle for production
```bash
bun build ./src/index.ts --outdir ./dist --target bun
```

### SQLite quick start
```bash
bun add bun:sqlite
bun run src/db-init.ts
```


#### Step 2.2: Add expected output verification

For each command, specify what success looks like:

```markdown
### Expected outputs

- `bun init`: Creates `index.ts`, `package.json`, `tsconfig.json`
- `bun test --coverage`: Reports coverage percentage > 80%
- `bun build`: Creates `dist/index.js` with size < 100KB
```

---

### Phase 3: Pattern Recognition (D7) - Priority: Medium

**Target**: Increase from 6/10 to 9/10 (+3 points)

#### Step 3.1: Expand frontmatter description

**File**: `skills/bun-development/SKILL.md`

Update frontmatter description field:

```yaml
---
name: bun-development
description: |
  Complete Bun.js runtime guidance. Use when: working with Bun APIs 
  (Bun.file, Bun.serve, Bun.serveHTTP), bun:sqlite, Bun.password, 
  Bun.$ shell, package management with bun install/add/remove, 
  testing with bun test, or migrating from Node.js to Bun.
  
  Keywords: Bun, Bun.file, Bun.serve, bun:sqlite, bun test, 
  hot reload, TypeScript-first, edge runtime, fast bundler
---
```

#### Step 3.2: Add "Use When" section at top of SKILL.md

```markdown
## Use When

- "How do I use Bun.file?"
- "Set up bun:sqlite database"
- "Bun.serve HTTP server"
- "Migrate from Node to Bun"
- "bun test configuration"
- NOT for: Node.js-specific APIs, npm/yarn package management
```

---

### Phase 4: Specification Compliance (D4) - Priority: Medium

**Target**: Increase from 10/15 to 12/15 (+2 points)

#### Step 4.1: Tighten frontmatter

Ensure all required fields are present and precise:

```yaml
---
name: bun-development
description: "[updated description from Phase 3]"
version: 1.0.0
author: tekhne
tags: [bun, runtime, sqlite, http-server, testing]
scope: bun-specific APIs and toolchain
---
```

---

## Verification Commands

```bash
# Re-run evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh bun-development --json

# Validate format
bunx markdownlint-cli2 "skills/bun-development/SKILL.md"

# Check for duplication
skills/skill-quality-auditor/scripts/detect-duplication.sh skills
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D3 Anti-Pattern Quality | Score >= 13/15 |
| D8 Practical Usability | Score >= 13/15 |
| D7 Pattern Recognition | Score >= 9/10 |
| Overall Score | >= 102/120 (B+) |
| No markdown lint errors | `bunx markdownlint-cli2` passes |

## Estimated Effort

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Anti-patterns | S | 30 min |
| Phase 2: Commands | S | 30 min |
| Phase 3: Triggers | S | 20 min |
| Phase 4: Frontmatter | S | 10 min |
| **Total** | **S** | **1.5 hours** |

## Dependencies

- None (self-contained skill)

## Rollback Plan

If remediation causes regression, restore from git:

```bash
git checkout HEAD~1 -- skills/bun-development/SKILL.md
```
