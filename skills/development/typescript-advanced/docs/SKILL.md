---
name: typescript-advanced-docs
description: Documentation tooling for TypeScript projects — JSDoc comment patterns that surface in IDE tooltips, TypeDoc configuration for generating API documentation sites, Architectural Decision Record (ADR) templates for recording design decisions, and framework-specific documentation patterns (NestJS, React, Angular/Compodoc). Use when writing JSDoc for a public API, setting up TypeDoc to generate documentation from source, recording why a technical decision was made, or documenting a framework-specific pattern.
allowed-tools: Read, Write, Edit, Bash
---

# TypeScript Documentation

Documentation tooling and conventions for TypeScript codebases: JSDoc, TypeDoc, ADRs, and framework-specific patterns.

## Mindset

Write documentation that a compiler-checked type can't already express — a JSDoc `@param` restating a parameter's type the signature already shows is noise; a JSDoc comment explaining *why* a parameter exists, what invariant it must satisfy, or what happens at its edge cases earns its place. Treat an ADR as a record for a future reader who wasn't in the room, not a justification written for the reviewer who is: state the decision, the alternatives considered, and the consequences accepted, in that order. Generated documentation (TypeDoc output) is only as good as the JSDoc it's generated from — investing in TypeDoc configuration without investing in the comments it reads from produces a well-formatted site with nothing useful in it.

## When to Apply

Use this skill when:

- Writing JSDoc comments for a public API surface
- Configuring TypeDoc to generate a documentation site from TypeScript source
- Recording an architectural decision with an ADR
- Documenting a framework-specific pattern (NestJS decorators, React components, Angular/Compodoc)

## Use When

- "How should I document this public function/class with JSDoc?"
- "How do I set up TypeDoc to generate our API docs?"
- "How do I write an ADR for this decision?"
- "What's the convention for documenting a NestJS/React/Angular component?"

## Scope

### In Scope

- JSDoc comment patterns and best practices.
- TypeDoc configuration and setup.
- ADR templates and structure.
- Framework-specific documentation patterns (NestJS, React, Angular).

### Out of Scope

- The type-first design decisions an ADR might record — see the sibling `typescript-practices` skill for the workflow itself.
- Generating documentation content from types mechanically covered elsewhere (e.g. what a mapped type does) — see the sibling `typescript-type-system` skill for the mechanics; this skill covers how to document the result.

## When NOT to Use

Do not use this skill for deciding *what* a type or pattern should do — it covers documenting a decision or API, not making one. Do not use it as a substitute for actually running `npx typedoc` and checking the generated output — a JSDoc comment that looks right can still render incorrectly (a malformed `@example` block, a broken `{@link}`).

## Quick Commands

### Generate Docs

```bash
npx typedoc --out docs src/index.ts
```

## Anti-Patterns

### NEVER write a JSDoc comment that only restates the type signature

**WHY:** a comment that duplicates what the signature already says adds reading overhead with no new information, and drifts out of sync with the signature over time.

**BAD**:

```typescript
/**
 * @param name - a string
 * @returns a string
 */
function greet(name: string): string { return `Hello, ${name}`; }
```

**GOOD**:

```typescript
/**
 * Formats a greeting for display. Falls back to "Guest" if `name` is empty,
 * since an empty greeting would render as blank in the header component.
 */
function greet(name: string): string {
  return `Hello, ${name || "Guest"}`;
}
```

### NEVER write an ADR that only justifies a decision already made

**WHY:** an ADR's value is to a future reader deciding whether the reasoning still holds; a one-sided justification omits the alternatives and trade-offs that reader needs to judge that.

**BAD**: "We chose Zod because it's good." (no alternatives, no consequences, no context)

**GOOD**: A record with Context (what problem existed), Decision (what was chosen), Alternatives Considered (what else, and why not), and Consequences (what this commits the codebase to).

### NEVER configure TypeDoc without first ensuring the source has real JSDoc to generate from

**WHY:** TypeDoc renders whatever comments exist; a well-configured generator over undocumented source produces a polished site of empty pages.

**BAD**: Running `npx typedoc --out docs src/index.ts` as the first and only documentation step on undocumented code.

**GOOD**: Add JSDoc to the public API surface first, then configure and run TypeDoc to publish it.

## References

| File | Covers |
| --- | --- |
| `references/jsdoc-patterns.md` | JSDoc comment best practices and patterns |
| `references/typedoc-config.md` | TypeDoc configuration and setup |
| `references/adr-templates.md` | Architectural Decision Record templates |
| `references/framework-docs.md` | Framework-specific documentation patterns (NestJS, React, Angular/Compodoc) |

- [TSDoc Specification](https://tsdoc.org/)
- [TypeDoc Documentation](https://typedoc.org/)
- [ADR GitHub Organization](https://adr.github.io/)
