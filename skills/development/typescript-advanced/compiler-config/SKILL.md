---
name: typescript-advanced-compiler-config
description: "TypeScript compiler configuration — tsconfig.json structure, strict-mode flags and what each one catches, module resolution strategies (node/bundler/nodenext), and diagnosing/fixing slow type-checking. Use when bootstrapping a tsconfig.json for a new project, deciding which strictness flags to enable beyond `strict: true`, debugging a module-resolution error, or a `tsc --noEmit` run that has gotten too slow."
allowed-tools: Read, Write, Edit, Bash
---

# TypeScript Compiler Configuration

The tsconfig.json options that decide what the compiler catches and how fast it catches it.

## Mindset

Keep `strict: true` as the non-negotiable baseline and treat every additional strictness flag (`noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitReturns`) as a default-on decision, not an opt-in one — the cost of a flag is some extra narrowing at the call sites it affects, and the cost of not having it is a defect the compiler could have caught but didn't. Never disable a strict-mode finding with `@ts-ignore` or `strict: false`; the defect it surfaced is still there, you've only removed the compiler's ability to tell you about it. When type-checking is slow, profile before guessing — `tsc --extendedDiagnostics` and project references usually explain the slowdown faster than trimming `include` patterns at random.

## When to Apply

Use this skill when:

- Bootstrapping a `tsconfig.json` for a new project
- Deciding which strictness flags to enable beyond the `strict` bundle
- Debugging a module-resolution error (`Cannot find module`, wrong resolution strategy for ESM/bundler setups)
- Diagnosing and fixing slow `tsc --noEmit` runs

## Use When

- "What should my tsconfig.json look like for a new Node/ESM project?"
- "Which strictness flags should I turn on beyond `strict: true`?"
- "Why is TypeScript not resolving this module the way I expect?"
- "Why is type-checking so slow, and how do I speed it up?"

## Scope

### In Scope

- `tsconfig.json` structure and common option choices.
- Strict-mode flags and what each one catches.
- Module resolution strategies (`node`, `bundler`, `node16`/`nodenext`) and their trade-offs.
- Diagnosing and improving type-checking performance.

### Out of Scope

- Type-system design (generics, conditional types, unions) — see the sibling `typescript-type-system` skill.
- Runtime validation and type guards — see the sibling `typescript-type-guards` and `typescript-practices` skills.
- Build-pipeline or bundler configuration unrelated to the TypeScript compiler itself.

## When NOT to Use

Do not use this skill for end-to-end test framework setup, CI pipeline configuration, or bundler configuration that isn't a `tsconfig.json` compiler option. Do not use it as a substitute for running `tsc --noEmit` — this skill explains what a setting does, but the compiler is the source of truth on whether the project actually type-checks under it.

## Quick Commands

### Type Check

```bash
npx tsc --noEmit
```

### Diagnose Slow Type-Checking

```bash
npx tsc --noEmit --extendedDiagnostics
```

### Generate a tsconfig.json

```bash
npx tsc --init
```

## Anti-Patterns

### NEVER disable a strict-mode finding instead of fixing the underlying type model

**WHY:** the defect strict mode caught is still present in the code; `@ts-ignore` and `strict: false` only remove the compiler's ability to report it.

**BAD**:

```jsonc
{ "compilerOptions": { "strict": false } }
```

**GOOD**:

```jsonc
{ "compilerOptions": { "strict": true, "noUncheckedIndexedAccess": true } }
```

### NEVER set `strict: true` and stop there when the project can afford more

**WHY:** `strict` is a floor, not a ceiling — flags like `noUncheckedIndexedAccess` and `exactOptionalPropertyTypes` catch real classes of bugs `strict` alone leaves open.

**BAD**:

```jsonc
{ "compilerOptions": { "strict": true } }
```

**GOOD**:

```jsonc
{
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true
  }
}
```

### NEVER guess at a performance fix before profiling

**WHY:** trimming `include` patterns or splitting files at random can miss the actual cause (often deep conditional-type recursion or an oversized project graph) while adding maintenance overhead for no measured gain.

**BAD**: Randomly excluding directories from `include` because "it feels slow."

**GOOD**: Run `tsc --noEmit --extendedDiagnostics` first, then act on what it reports (files checked, instantiation count, memory).

## References

| File | Covers |
| --- | --- |
| `references/tsconfig.md` | `tsconfig.json` structure and options |
| `references/strict-mode.md` | Strict-mode flags and what each one catches |
| `references/module-resolution.md` | Module resolution strategies |
| `references/performance.md` | Diagnosing and improving type-checking speed |

- [TSConfig Reference](https://www.typescriptlang.org/tsconfig)
- [TypeScript Handbook: Compiler Options](https://www.typescriptlang.org/docs/handbook/compiler-options.html)
