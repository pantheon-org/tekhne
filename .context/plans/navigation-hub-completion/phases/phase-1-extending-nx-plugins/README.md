# Phase 1: Convert extending-nx-plugins to Navigation Hub

**Duration:** 2-3 hours
**Purpose:** Split the 432-line monolithic SKILL.md into a lean hub + focused reference files

## Current Structure

Single file: `skills/repository-mgmt/nx/extending-plugins/SKILL.md` (432 lines)

**Identified sections in the monolith:**

| Section | Lines (approx) | Target reference file |
|---------|---------------|----------------------|
| Creating a Plugin (setup, scaffold commands) | ~40 | `references/plugin-setup.md` |
| Creating Your First Generator (impl, templates, schema, validation) | ~150 | `references/generators.md` |
| Helper Functions (Nx Devkit API) | ~75 | `references/devkit-api.md` |
| Common Patterns (composing, conditional, updating projects) | ~65 | `references/common-patterns.md` |
| Anti-Patterns | ~50 | Keep in SKILL.md (summarized) |
| Learning Resources | ~20 | Keep in SKILL.md (links) |

## Target Structure

```
skills/repository-mgmt/nx/extending-plugins/
├── SKILL.md                        (hub: ≤ 150 lines)
├── tile.json
└── references/
    ├── plugin-setup.md             (~50 lines)
    ├── generators.md               (~160 lines)
    ├── devkit-api.md               (~80 lines)
    └── common-patterns.md          (~70 lines)
```

## Tasks

### 1. Create references/ Directory and Reference Files

Extract the four content blocks from SKILL.md verbatim into reference files:

**`references/plugin-setup.md`** — Extract:
- "Creating a Plugin" section (new workspace and add to existing workspace commands)
- "Plugin Components" table

**`references/generators.md`** — Extract:
- "Creating Your First Generator" section in full (Generate a Generator, Generator Structure, Template Files, Running and Validating)
- "Best Practices" section (all four sub-sections)

**`references/devkit-api.md`** — Extract:
- "Helper Functions" section in full (File Operations, Project Management, Dependency Management, String Utilities)

**`references/common-patterns.md`** — Extract:
- "Common Patterns" section in full (Composing Generators, Conditional File Generation, Updating Existing Projects)

### 2. Rewrite SKILL.md as a Hub

Replace SKILL.md with a lean hub that:
- Preserves the frontmatter (name, description, allowed-tools)
- Adds a "Navigation hub for extending Nx with plugins" opening line
- Keeps "When to Use" / "When Not to Use" sections (write fresh if absent)
- Keeps a brief "Workflow" (numbered steps, no code blocks)
- Keeps "Anti-Patterns" (all 5 — they are directive, hub-appropriate content)
- Adds a "Quick Reference" table linking to the 4 reference files
- Adds "References" (external links: Nx Devkit API, tutorials, etc.)
- Removes all extracted content (implementation details, code blocks, helper function listings)

**Hub target length: ≤ 150 lines**

### 3. Create tile.json

Model after an existing tile (e.g., `skills/repository-mgmt/nx/workspace-patterns/tile.json`).

```bash
cat skills/repository-mgmt/nx/workspace-patterns/tile.json
```

Adapt for `extending-nx-plugins`:
- Set name to `extending-nx-plugins` (matching SKILL.md frontmatter `name:`)
- Set description to match SKILL.md frontmatter `description:`

### 4. Validate

```bash
# Confirm SKILL.md line count reduced
wc -l skills/repository-mgmt/nx/extending-plugins/SKILL.md

# Confirm all reference files exist and are non-empty
find skills/repository-mgmt/nx/extending-plugins/references -type f -name '*.md' | xargs wc -l

# Confirm Quick Reference table links match actual filenames
grep "references/" skills/repository-mgmt/nx/extending-plugins/SKILL.md
```

**Success Criteria:**
- [ ] SKILL.md ≤ 150 lines
- [ ] 4 reference files created, each ≥ 30 lines
- [ ] Hub SKILL.md contains "Navigation hub" opening
- [ ] All Quick Reference links match actual file paths
- [ ] tile.json created

## Outputs

- `skills/repository-mgmt/nx/extending-plugins/SKILL.md` (hub, ≤ 150 lines)
- `skills/repository-mgmt/nx/extending-plugins/tile.json`
- `skills/repository-mgmt/nx/extending-plugins/references/plugin-setup.md`
- `skills/repository-mgmt/nx/extending-plugins/references/generators.md`
- `skills/repository-mgmt/nx/extending-plugins/references/devkit-api.md`
- `skills/repository-mgmt/nx/extending-plugins/references/common-patterns.md`

## Next Phase

[Phase 2: Conventional Commits](../phase-2-conventional-commits/)
