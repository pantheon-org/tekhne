# CLI Architecture Review

**Date:** 2026-03-03  
**Scope:** `/cli` directory  
**Reviewer:** Claude  
**Status:** ✅ Good with minor improvements needed

## Executive Summary

The CLI architecture demonstrates **solid engineering practices** with clear separation of concerns, good module organization, and consistent patterns. TypeScript compiles cleanly, Biome reports no issues, and the code follows Commander.js best practices.

**Grade: B+ (87/100)**

**Key Strengths:**
- Clean command-handler-implementation separation
- Domain-based lib organization mirrors repository structure
- Minimal external dependencies
- Type-safe interfaces throughout

**Improvement Areas:**
- Missing test coverage (0 tests)
- Error handling mixes `process.exit()` and `throw` strategies
- No input validation layer
- Hard dependency on shell commands

---

## Architecture Analysis

### 1. Directory Structure ✅ EXCELLENT

```
cli/
├── index.ts              # Entry point, Commander.js orchestration
├── commands/             # Command definitions (thin layer)
│   ├── audit.ts
│   ├── install.ts
│   ├── readme.ts
│   └── tessl.ts
└── lib/                  # Business logic (thick layer)
    ├── audit/           # Quality auditing
    ├── install/         # Agent installation
    ├── readme/          # README maintenance
    ├── tessl/           # Registry management
    └── utils/           # Shared utilities
```

**Strengths:**
- Command pattern correctly implemented
- Commands delegate to lib functions immediately
- Lib directories organized by domain (audit, tessl, install, readme)
- Utils isolated for cross-cutting concerns

**Rationale:** Follows Clean Architecture principle of dependency flow inward. Commands depend on lib, lib depends on utils, no circular dependencies detected.

### 2. Command Layer Design ✅ GOOD

**Pattern:**
```typescript
// commands/audit.ts
export const auditCommand = new Command("audit");
auditCommand
  .command("skill <path>")
  .action(async (path: string) => {
    await auditSkill(path);  // Delegate immediately
  });
```

**Strengths:**
- Thin command definitions (7-39 lines)
- No business logic in command files
- Consistent delegation pattern
- Type-safe option interfaces

**Concerns:** None significant.

### 3. Business Logic Layer ✅ GOOD

**Key Files:**
- `lib/audit/audit-skill.ts` - Single skill auditing
- `lib/tessl/manage.ts` - Tessl lifecycle orchestration
- `lib/install/install-skills.ts` - Symlink-based installation
- `lib/readme/update-readme.ts` - Table generation

**Strengths:**
- Functions are focused (56-182 lines)
- Clear input/output types via interfaces
- Consistent async/await patterns
- No god objects detected

**Concerns:**
- Functions still somewhat large (150-180 lines suggests refactoring opportunities)
- Some functions do multiple things (e.g., `tesslManage` handles both single + batch operations)

### 4. Error Handling ⚠️ MIXED STRATEGY

**Current State:**
- `process.exit()` used 8 times (mostly in lib/)
- `throw new Error()` used 4 times
- Some functions return boolean success flags

**Problem:** Inconsistent error propagation makes testing and error recovery difficult.

**Example from audit-skill.ts:12-13:**
```typescript
if (!existsSync(skillFile)) {
  logger.error(`No SKILL.md found at ${fullPath}`);
  process.exit(1);  // ❌ Hard exit in lib layer
}
```

**Recommendation:**
```typescript
if (!existsSync(skillFile)) {
  throw new SkillNotFoundError(`No SKILL.md found at ${fullPath}`);
}
```

Then handle in command layer:
```typescript
.action(async (path: string) => {
  try {
    await auditSkill(path);
  } catch (error) {
    logger.error(error.message);
    process.exit(1);
  }
});
```

**Impact:** Medium priority. CLI still functions correctly, but harder to test and compose.

### 5. Type Safety ✅ EXCELLENT

**Observations:**
- 8 interfaces defined for options/data structures
- No `any` types detected in sampled files
- TypeScript strict mode appears enabled
- Bun's native TypeScript support leveraged

**Strengths:**
- Options interfaces for every command (InstallOptions, ManageOptions, etc.)
- Return types explicitly typed
- Type inference used appropriately

### 6. Dependencies ✅ MINIMAL

**External:**
- `commander` - CLI framework
- `bun` - Runtime with native TypeScript, shell, and file APIs

**Internal:**
- Leverages Bun's `$` template for shell commands
- Uses Node.js `fs`, `path`, `os` modules
- No heavyweight frameworks or ORMs

**Strengths:**
- Low dependency footprint
- Bun integration provides shell/FS utilities without extra deps
- No transitive dependency bloat

**Concerns:**
- Shell dependency (`tessl`, `find`, `sh`) creates external coupling
- No abstraction layer for shell commands (makes testing harder)

### 7. Code Quality ✅ EXCELLENT

**Biome Report:** ✅ 0 issues  
**TypeScript:** ✅ Compiles cleanly  
**Style:** ✅ Consistent formatting

**Patterns Observed:**
- Clear function naming (verb-noun: `auditSkill`, `installSkills`)
- Consistent indentation and formatting
- Early returns for validation
- Async/await over promises

### 8. Testing ❌ MISSING

**Current State:**
- 0 test files (no `.test.ts` or `.spec.ts` found)
- No test framework configured
- No mocking/stubbing infrastructure

**Impact:** High risk. CLI has 19 TypeScript files with 0 tests covering:
- Shell command execution
- File system operations
- Complex orchestration logic (tesslManage, updateReadme)

**Recommendation:** Add tests for:
1. Command parsing (Commander.js integration)
2. Shell execution utilities (mock `exec` function)
3. Business logic functions (audit, tessl, install)
4. Error scenarios (missing files, failed commands)

**Suggested Framework:** Bun's built-in test runner
```typescript
// lib/audit/audit-skill.test.ts
import { test, expect, mock } from "bun:test";
import { auditSkill } from "./audit-skill";

test("auditSkill throws when SKILL.md missing", async () => {
  expect(async () => {
    await auditSkill("/nonexistent/path");
  }).toThrow("No SKILL.md found");
});
```

### 9. Input Validation ⚠️ MINIMAL

**Current Approach:**
- File existence checks via `existsSync`
- Basic type checking from Commander.js
- No schema validation for complex inputs

**Example from tessl/manage.ts:73-87:**
```typescript
const tileData = await Bun.file(tileJsonPath).json();
const tileName = tileData.name;  // ❌ No validation of structure
```

**Recommendation:** Add validation layer:
```typescript
import { z } from "zod";

const TileSchema = z.object({
  name: z.string().min(1),
  version: z.string().regex(/^\d+\.\d+\.\d+$/),
  skills: z.array(z.string()).optional(),
});

const tileData = TileSchema.parse(await Bun.file(tileJsonPath).json());
```

**Impact:** Low-medium priority. Current approach works but fragile to malformed inputs.

### 10. Utility Modules ✅ GOOD

**logger.ts (21 lines):**
- Provides consistent CLI output formatting
- ANSI color codes for terminal
- Five log levels (info, success, warning, error, debug)
- No external dependencies

**shell.ts (30 lines):**
- Wraps Bun's `$` shell execution
- Provides `exec` (returns result) and `execOrThrow` (throws on error)
- Consistent error handling

**Strengths:**
- Small, focused modules
- Clear contracts (input/output types)
- Reusable across commands

---

## Design Principles Assessment

### SOLID Principles

**Single Responsibility ✅**
- Commands: Route to lib functions
- Lib functions: Single domain operation
- Utils: Cross-cutting concerns

**Open/Closed ⚠️**
- Adding new commands requires editing index.ts
- Could use plugin pattern for discoverability
- Not critical for internal tool

**Liskov Substitution ✅**
- No inheritance detected (composition preferred)

**Interface Segregation ✅**
- Small, focused interfaces (3-5 properties each)
- No fat interfaces

**Dependency Inversion ⚠️**
- Logger/shell utilities properly abstracted
- Shell commands directly invoked (no abstraction for `tessl`, `find`)

### Other Principles

**DRY ✅**
- Shared utilities extracted (logger, shell)
- Common patterns abstracted (audit-all reuses audit-skill)

**KISS ✅**
- Straightforward implementation
- No over-engineering

**YAGNI ✅**
- Only builds what's needed
- No speculative features

**Separation of Concerns ✅**
- Commands separate from logic
- Logic grouped by domain
- Utils isolated

---

## Architectural Patterns

### Current Pattern: **Transaction Script**
- Commands trigger scripts (functions) that execute top-to-bottom
- Appropriate for CLI tools with linear workflows
- Low ceremony, easy to understand

### Alternative Considered: **Hexagonal Architecture**
- Would add ports/adapters for shell, filesystem
- Enables better testability and swapping implementations
- Overkill for current CLI scope

### Recommendation: **Keep Transaction Script, Add Boundaries**
- Current pattern appropriate for CLI tool
- Add interface boundaries for external dependencies (shell, fs)
- Enable testing without major refactoring

---

## Specific Issues Found

### Issue 1: Hard Process Exits in Lib Layer
**Severity:** Medium  
**Files:** `audit-skill.ts:12`, `audit-skill.ts:24`, `audit-skill.ts:34`, `tessl/manage.ts:145`, `tessl/manage.ts:180`, `readme/update-readme.ts:78`, `readme/update-readme.ts:84`

**Problem:** `process.exit()` in lib functions prevents:
- Unit testing
- Error recovery
- Composition of functions

**Fix:** Throw typed errors, catch in command layer.

### Issue 2: Missing Tests
**Severity:** High  
**Files:** Entire codebase

**Problem:** 0% test coverage for 19 TypeScript files

**Fix:** Add Bun tests for critical paths:
- Shell execution wrapper
- Audit orchestration
- Tessl lifecycle management
- Install symlink logic

### Issue 3: No Input Validation
**Severity:** Medium  
**Files:** All functions accepting JSON files

**Problem:** No schema validation for `tile.json`, `audit.json` structures

**Fix:** Add Zod schemas for external data formats

### Issue 4: Shell Command Coupling
**Severity:** Low  
**Files:** All lib functions using `exec()`

**Problem:** Hard dependency on external commands (`tessl`, `find`, `sh`)

**Fix:** Extract interface for shell operations:
```typescript
interface ShellExecutor {
  exec(cmd: string): Promise<{ stdout: string; exitCode: number }>;
}
```

Then inject in lib functions for testability.

### Issue 5: Large Functions
**Severity:** Low  
**Files:** `tessl/manage.ts` (182 lines), `install/install-skills.ts` (174 lines)

**Problem:** Functions approaching 200 lines harder to understand/test

**Fix:** Extract helper functions:
- `tesslManage` → split into `processSkill` + `processBatch`
- `installSkills` → extract `createAgentSymlinks`

---

## Recommendations

### Priority 1: Critical (Do Now)
1. **Add test infrastructure**
   - Set up Bun test runner
   - Add tests for utils (logger, shell)
   - Add tests for core business logic

2. **Fix error handling strategy**
   - Remove `process.exit()` from lib layer
   - Throw typed errors
   - Catch in command layer

### Priority 2: Important (Do Soon)
3. **Add input validation**
   - Use Zod for JSON file parsing
   - Validate command arguments
   - Provide clear error messages

4. **Refactor large functions**
   - Split `tesslManage` into smaller functions
   - Extract `installSkills` helper functions
   - Keep functions under 100 lines

### Priority 3: Nice to Have (Do Later)
5. **Add shell abstraction layer**
   - Create `ShellExecutor` interface
   - Inject into lib functions
   - Enable mocking for tests

6. **Add integration tests**
   - Test end-to-end command execution
   - Use temp directories for file operations
   - Mock external `tessl` commands

7. **Consider plugin architecture**
   - Auto-discover commands from `commands/`
   - Eliminate manual registration in index.ts

---

## Conclusion

The CLI architecture is **well-structured and follows solid engineering practices**. The command-handler-implementation pattern is correctly applied, modules are organized logically, and the code is type-safe and maintainable.

**Primary gaps:**
1. Missing tests (highest priority)
2. Inconsistent error handling (medium priority)
3. No input validation (medium priority)

**Bottom line:** This is production-quality code that would benefit from a testing layer before expanding functionality. The architecture supports growth without major refactoring.

---

## Appendix: File Inventory

| Category | Files | Lines | Notes |
|----------|-------|-------|-------|
| Commands | 4 | ~80 | Thin delegation layer |
| Audit | 4 | ~250 | Quality auditing domain |
| Tessl | 2 | ~200 | Registry management |
| Install | 1 | ~174 | Agent installation |
| README | 5 | ~350 | Table generation |
| Utils | 2 | ~51 | Logger + shell |
| **Total** | **19** | **~1200** | Well-scoped CLI |

**Dependencies:**
- External: 1 (commander)
- Runtime: Bun (provides TS, shell, fs)
- Scripts: Depends on external `tessl` CLI

---

## References

- Clean Architecture (Robert C. Martin)
- SOLID Principles
- Commander.js Documentation
- Bun Runtime API
