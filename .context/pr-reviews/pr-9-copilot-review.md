# PR #9 — Copilot Review Summary

**Title:** feat(cli): improve install command robustness and filtering  
**Author:** thoroc  
**Branch:** `feat/cli-install-improvements` → `main`  
**Status:** Open  
**Date reviewed:** 2026-03-13  
**URL:** https://github.com/pantheon-org/tekhne/pull/9

## Overview

This PR fixes six issues identified during a review of the install CLI command, covering correctness, usability, and test coverage.

### Changes

**Correctness:**
- Unknown agent exits non-zero — previously `-a unknown` logged an error but exited 0. Now validates all agents upfront and throws a `CLIError` (exit 1) before doing any work.
- Relative symlinks — symlinks were created with absolute source paths, breaking silently if the repository was moved or re-cloned. Now relative.
- CWD guard in `findSkills()` — running outside the repository root silently returned "0 skills found". Now checks for `skills/` existence first and throws a clear error message.

**Usability:**
- `--global` warning for always-global agents — `cursor`, `gemini`, `claude`, and `codex` always install to `~/.config/` regardless of the flag. Passing `--global` for these agents now emits a warning.
- `--skill-domain <domains...>` — filter by top-level domain (e.g. `ci-cd`, `infrastructure`).
- `--skill-subdomain <subdomains...>` — filter by subdomain (e.g. `github-actions`, `terraform`).
- `-i / --interactive` flag — presents a checkbox prompt via `@inquirer/checkbox` for per-skill selection after domain/subdomain pre-filtering. Gracefully ignored when stdin is not a TTY.

**Tests:**
- Internal functions (`findSkills`, `filterSkills`, `createSymlink`, `ensureDirectory`) are now exported and covered by 24 filesystem-level tests using Bun's tmp utilities.

**Files changed:** `bun.lock`, `cli/commands/install.ts`, `cli/lib/install/install-skills.ts`, `cli/lib/install/install-skills.test.ts`, `package.json`

---

## Copilot AI Review — 6 Inline Comments

### Comment 1 — Incomplete relative symlink assertion

**File:** `cli/lib/install/install-skills.test.ts:238–242`  
**Severity:** Low

The test checks that `linkDest` is not absolute (`not.toMatch(/^\//)`), but that only verifies it is not absolute — it does not verify the path is *correctly* relative and resolves to the expected target.

```ts
const linkDest = require("node:fs").readlinkSync(target);
// Should be relative, not absolute
expect(linkDest).not.toMatch(/^\//);
```

**Recommendation:** Strengthen to assert the resolved path equals the source directory.

---

### Comment 2 — `resolve()` may fail on macOS due to `/tmp` symlink

**File:** `cli/lib/install/install-skills.test.ts:275–277`  
**Severity:** Medium

The test asserts that `resolve(dirname(target), linkDest)` equals `resolve(sourceDir)`, but `sourceDir` is created inside `tmpDir` which may itself be a symlink on macOS (`/tmp` → `/private/tmp`). This could cause the assertion to fail on macOS CI.

```ts
expect(resolve(require("node:path").dirname(target), linkDest)).toBe(
  resolve(sourceDir),
);
```

**Recommendation:** Use `realpathSync` on both sides to normalize symlinks before comparing.

---

### Comment 3 — Misleading test name contradicts assertion

**File:** `cli/lib/install/install-skills.test.ts:246–250`  
**Severity:** Low

The test is named `"returns false without creating symlink in dry-run mode"` but the assertion is `expect(result).toBe(true)`. The test name contradicts the assertion — either the name is wrong, or the expected return value is wrong.

```ts
test("returns false without creating symlink in dry-run mode", () => {
  const target = join(tmpDir, "link");
  const result = createSymlink(sourceDir, target, true);

  expect(result).toBe(true); // dry-run still signals "would install"
  expect(require("node:fs").existsSync(target)).toBe(false);
```

**Recommendation:** Rename to `"returns true (would-install signal) without creating symlink in dry-run mode"`.

---

### Comment 4 — `process.exit(0)` in business logic

**File:** `cli/lib/install/install-skills.ts` (interactive selection path)  
**Severity:** Medium

`process.exit(0)` is called directly inside `selectSkillsInteractively()` when no skills are selected. This makes the function non-testable and breaks separation of concerns — the function should throw or return early and let the caller decide the exit behaviour.

```ts
if (selected.length === 0) {
  logger.info("No skills selected. Aborting.");
  process.exit(0);
```

**Recommendation:** Return an empty array and let `installSkills()` handle early exit, or throw a typed error caught by the top-level action handler.

---

### Comment 5 — Undocumented Bun runtime dependency

**File:** `cli/lib/install/install-skills.ts:10–11`  
**Severity:** Low

`import { $ } from "bun"` is used only in `findSkills()` to run `find`. This is a Bun-specific dependency that prevents running the module under Node.js, with no documentation or comment explaining the requirement.

```ts
import checkbox from "@inquirer/checkbox";
import { $ } from "bun";
```

**Recommendation:** Add a comment noting the Bun runtime requirement, or replace the shell `find` call with a pure `node:fs`/glob approach to remove the runtime coupling.

---

### Comment 6 — `ensureDirectory` test does not fully exercise nested creation

**File:** `cli/lib/install/install-skills.test.ts:197–201`  
**Severity:** Low

The test creates a nested directory via `ensureDirectory(dir, false)` and asserts `existsSync(dir)` is true, but does not verify that the intermediate parent directory (`new/`) was also created by the `recursive: true` call.

```ts
test("creates directory in live mode", () => {
  const dir = join(tmpDir, "new", "nested");
  ensureDirectory(dir, false);
  expect(require("node:fs").existsSync(dir)).toBe(true);
});
```

**Recommendation:** Also assert `existsSync(join(tmpDir, "new"))` to confirm recursive creation.

---

## Issue Summary

| # | File | Issue | Severity |
|---|------|-------|----------|
| 1 | `install-skills.test.ts:238` | Symlink relative-path assertion is incomplete | Low |
| 2 | `install-skills.test.ts:275` | `resolve()` may fail on macOS — use `realpathSync` | Medium |
| 3 | `install-skills.test.ts:246` | Test name contradicts assertion (`false` vs `true`) | Low |
| 4 | `install-skills.ts` | `process.exit(0)` in business logic makes it untestable | Medium |
| 5 | `install-skills.ts:10` | `import { $ } from "bun"` undocumented runtime dependency | Low |
| 6 | `install-skills.test.ts:197` | `ensureDirectory` test doesn't assert intermediate directory creation | Low |
