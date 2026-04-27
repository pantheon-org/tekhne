---
name: input-validation-skill
description: Validates and sanitizes user inputs before processing to prevent injection attacks, data corruption, and runtime errors in production systems
---

# Input Validation Skill

Enforce strict input validation before any processing to prevent security vulnerabilities and data corruption. See `references/deep-reference.md` for schema details.

## Mindset

ALWAYS treat external input as untrusted. Validate at the boundary, not deep in business logic. Every gotcha in production traces back to skipped validation.

Core rules:

1. Validate at entry points, not deep in business logic
2. Reject early — fail fast on malformed input
3. NEVER assume internal callers are trusted without proof

## When to Use

- Processing user-supplied data from forms, APIs, or CLI arguments
- Ingesting files or configuration from external sources
- Handling webhook payloads or third-party integrations

## When NOT to Use

- Validating internal constants or compile-time values
- Re-validating data that has already passed a trusted validation boundary in the same request lifecycle
- optionally skip for internal microservice calls behind a trusted network boundary

## Procedures

### 1. Define the schema

```typescript
import { z } from "zod";

const InputSchema = z.object({
  name: z.string().min(1).max(100),
  email: z.string().email(),
  age: z.number().int().min(0).max(150),
});
```

### 2. Parse and validate at the entry point

```typescript
export const validateInput = (raw: unknown) => {
  const result = InputSchema.safeParse(raw);
  if (!result.success) {
    throw new Error(`Validation failed: ${result.error.message}`);
  }
  return result.data;
};
```

### 3. Run validation before processing

```bash
bun run validate --input ./data/input.json
```

### 4. Handle errors explicitly

```typescript
try {
  const validated = validateInput(req.body);
  await process(validated);
} catch (err) {
  return res.status(400).json({ error: err.message });
}
```

## Anti-Patterns

BAD: Skipping validation in a "trusted" internal path
```typescript
// BAD
function processInternal(data: any) {
  db.insert(data); // no validation
}
```

WHY: Internal paths are often reachable from untrusted callers via indirection. NEVER assume trust without verifying the call chain.

GOOD:
```typescript
// GOOD
function processInternal(data: unknown) {
  const safe = InternalSchema.parse(data);
  db.insert(safe);
}
```

BAD: Validating after side effects
```typescript
// BAD
await db.insert(data);
validate(data); // too late
```

WHY: Side effects are already applied when validation fails. ALWAYS validate before any mutation.

## References

- [Deep Reference: Schema Design](./references/deep-reference.md) — Schema design and progressive disclosure patterns
- consider reviewing Zod documentation for advanced schema composition
