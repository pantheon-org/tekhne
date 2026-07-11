# Edge Cases — Deep Dive Reference

## Standards in Non-Prose Formats

Users may supply standards in structured formats rather than plain text:

| Format | How to handle |
|--------|---------------|
| `.eslintrc` / ESLint flat config | Extract rule names, map to human-readable conventions, check if violations exist |
| `.prettierrc` | Treat as formatting standards; use prettier --check |
| `tsconfig.json` | Extract `strict` flags, `noUnusedLocals`, etc. |
| `editorconfig` | Map properties to formatting conventions |
| PR template / CONTRIBUTING.md | Extract prose conventions from markdown sections |
| Link to external style guide (Airbnb, Google) | Fetch the guide, extract the subset relevant to this codebase's languages |

**Rule of thumb:** If the format is parseable, extract rules programmatically. If it's prose, extract conventions as structured checklist items.

## Standards in a Language Not in This Project

If the user says "use Python naming conventions" but the codebase is TypeScript:

1. Flag the mismatch immediately
2. Ask: "Your codebase is TypeScript — should I apply TypeScript equivalents of those conventions? (e.g., camelCase variables, PascalCase classes)"
3. Do not attempt to check Python-specific rules against TS files

## Standards That Reference Multiple File Types

When a standard applies to some file types but not others:

- "No `var`" → applies to JS/TS only, not JSON/YAML
- "No `console.log`" → applies to JS/TS, not shell scripts or config files
- "Kebab-case filenames" → applies to source files, not assets or generated output

Classify by file extension and exclude non-applicable files from each scan. Report the scope: "Checked 98 JS/TS files, excluded 12 asset/config files."

## Codebase Is Very Large (>1000 files)

Ask the user to scope before scanning:

> "This codebase has ~2,400 source files. Checking all of them may take time. Should I scan the whole codebase, or focus on a specific directory/module?"

If the user picks a directory, note the scope in the report: "Scanned src/api/ (142 files)."

## User Refuses to Provide Standards

If the user insists on proceeding without standards:

> "I can't check alignment without knowing what to align against. At minimum, can you give me 1-2 concrete rules? For example: naming convention, import style, or something to avoid."

If they still refuse, do not proceed — explain that the skill requires standards to function.

## Standards Conflict with Each Other

When two standards contradict:

1. Identify the specific conflict: "Standard A says 2-space indent, Standard B (Prettier config) says 4-space tabs"
2. Present both sides neutrally
3. Ask the user which takes precedence
4. If the user says "use both" — explain why they're incompatible and ask for a single ruling

Do not guess. Do not pick one silently.

## Standard Cannot Be Checked Objectively

Subjective standards ("code should be readable", "use clean architecture"):

- Label as manual-only
- Provide a shortlist of files/functions where the standard *might* apply
- Note that human review is required
- Example: "**Clean architecture** (manual): The following modules mix concerns and may violate separation of layers — src/controllers/, src/utils/helpers.js"

## Standards Changed After Partial Report

If the user revises standards after seeing initial results:

1. Acknowledge the change
2. Identify which standards are new, removed, or modified
3. Re-scan only the affected standards (do not re-run unchanged ones)
4. Produce a new incremental report showing only the delta
5. Offer to merge with the previous report
