# CLI TypeScript Conventions

These rules apply to all TypeScript source files under `cli/`.
They are enforced automatically by pre-commit hooks and Biome linting.

## Rules

### 1. Arrow Functions Only

Use `const` arrow functions. Never use named function declarations.

```ts
// ✓ correct
export const doThing = async (path: string): Promise<void> => { ... };

// ✗ wrong
export async function doThing(path: string): Promise<void> { ... }
```

**Enforced by:** `cli/scripts/validate-ts-conventions.ts` (pre-commit).

---

### 2. Barrel Modules

Every `lib/` subdirectory must have an `index.ts` that re-exports its public API.
Consumers import from the barrel, not from individual files.

```ts
// ✓ correct
import { auditSkill } from "../lib/audit";

// ✗ wrong
import { auditSkill } from "../lib/audit/audit-skill";
```

**Enforced by:** Code review and `cli/scripts/validate-ts-conventions.ts`.

---

### 3. Collocated Tests

Test files live beside the source file they test, using the `.test.ts` suffix.
There is no separate `tests/` directory.

```
lib/audit/audit-skill.ts       ← source
lib/audit/audit-skill.test.ts  ← its test
```

**Enforced by:** Code review.

---

### 4. No Internal Functions

Do not define functions inside other functions. Extract helpers to their own module file.

```ts
// ✓ correct — helper is a separate file
// lib/audit/format-grade.ts
export const formatGrade = (grade: string): string => `[${grade}]`;

// lib/audit/audit-skill.ts
import { formatGrade } from "./format-grade";
export const auditSkill = async (path: string) => { ... formatGrade(grade) ... };

// ✗ wrong — internal helper
export const auditSkill = async (path: string) => {
  const formatGrade = (grade: string) => `[${grade}]`; // internal function
  ...
};
```

**Enforced by:** `cli/scripts/validate-ts-conventions.ts` (pre-commit).

---

### 5. Function Body ≤ 150 Lines

No single function body may exceed 150 lines. If it does, split the logic into
separate files (each containing one function) and compose them.

**Enforced by:** `cli/scripts/validate-ts-conventions.ts` (pre-commit).

---

### 6. One Function Per Module

Each non-barrel `*.ts` file exports exactly one arrow function.
Type definitions, constants, and schema objects do not count as functions.

```
lib/audit/audit-skill.ts    → exports `auditSkill`
lib/audit/format-grade.ts   → exports `formatGrade`
lib/audit/index.ts          → barrel: re-exports both (exempt from this rule)
```

**Enforced by:** `cli/scripts/validate-ts-conventions.ts` (pre-commit).

---

## Exemptions

| File type | Exempt from |
|-----------|------------|
| `index.ts` (barrel) | Rule 6 (multiple re-exports are expected) |
| `*.test.ts` | All rules (test helpers/describe blocks are fine) |
| `*.schema.ts` | Rule 6 (exports types + schema object, no functions) |

## Fixing Violations

When a violation is reported:

1. Extract the violating helper into its own `lib/<domain>/<helper-name>.ts` file
2. Export it as the single arrow function from that file
3. Add it to the barrel `index.ts`
4. Import it from the barrel in the original file
