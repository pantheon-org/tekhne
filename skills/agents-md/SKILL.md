---
name: agents-md
description: Complete AGENTS.md management system. Create, generate, update, and maintain AI agent documentation for any codebase size. Use when user asks to "create AGENTS.md", "update AGENTS.md", "generate agents", "maintain agent docs", "set up AI agent docs", analyze codebase for AI documentation, or set up AI-friendly project documentation. Handles simple projects to complex hierarchical monorepos.
---

# Complete AGENTS.md Management

Comprehensive system for creating, generating, and maintaining AI agent documentation across all project types—from simple single-package projects to complex hierarchical monorepos.

## Core Principles

1. **Context-sensitive approach** - Simple projects get simple AGENTS.md, complex codebases get hierarchical structure
2. **Minimal token usage** - Keep files focused and actionable, avoid encyclopedic documentation  
3. **Nearest-wins hierarchy** - AI agents read closest AGENTS.md to file being edited
4. **JIT indexing** - Provide paths/globs/commands, NOT full content
5. **Reference over duplication** - Point to skills and existing documentation

## Decision Tree: Which Approach?

Before choosing your approach, ask yourself:
- **Complexity**: How many distinct tech stacks or patterns exist?
- **Team structure**: Do different teams own different parts?
- **Maintenance**: Who will keep documentation updated?

### Simple Project (Single AGENTS.md)
Use when:
- Single package/service
- Under 10 major directories  
- Uniform tech stack
- Small team or personal project

### Hierarchical Structure (Multiple AGENTS.md)
Use when:
- Monorepo with multiple packages/services
- Mixed tech stacks
- Different patterns per package
- Large team with specialized domains

## Phase 1: Repository Analysis

**MANDATORY - READ ENTIRE FILE**: Before proceeding with analysis, load `references/discovery-commands.md` for complete repository analysis commands.

### Critical Rule: NEVER Assume Technology Stack
**Before creating any AGENTS.md content, you MUST:**
1. **Run discovery commands** to detect actual technologies in use
2. **Analyze findings** to understand project structure and patterns
3. **Present analysis report** before generating any documentation
4. **Use detected patterns** - never assume React, Express, Jest, etc.

### Required Analysis Steps
Always start by analyzing the codebase structure and present findings before generating files:

1. **Repository type**: Simple, multi-package, or monorepo?
2. **Tech stack**: Languages, frameworks, key tools (DETECTED, not assumed)
3. **Major directories** needing own AGENTS.md
4. **Build system**: pnpm/npm/yarn workspaces? Turborepo? Lerna? (DETECTED)
5. **Testing setup**: Jest, Vitest, Playwright, pytest? (DETECTED)
6. **Existing skills**: Local skills to reference
7. **Key patterns**: Organization, conventions, examples (FROM ACTUAL CODE)

## Phase 2: Content Creation

**MANDATORY - READ ENTIRE FILE**: Before writing any content, load `references/anti-patterns.md` to understand what to avoid.

**Do NOT load** domain-specific templates until you identify the package type.

### Simple Project AGENTS.md

For straightforward projects, create single comprehensive file with these required sections:

```markdown
# Project Name

## Project Identity
[2-3 lines: what it does, primary tech]

## Package Manager
Use **[tool]**: `[install]`, `[dev]`, `[test]`

## Setup & Run Commands
[5-10 lines: essential commands only]

## Key Conventions
[10-15 lines - MOST IMPORTANT SECTION]
- File organization rules with examples
- Naming conventions  
- Architecture patterns
- Code style (or reference to config)

## Local Skills
[Reference each discovered skill]
Use `[skill-name]` skill for [purpose]. See `[path]/SKILL.md`

## Security & Secrets
[3-5 lines: never commit tokens, .env patterns, PII handling]

## Definition of Done
[3-5 lines: what must pass before PR/merge]
```

### Hierarchical Structure

For complex codebases, create lightweight root + detailed sub-files.

#### Root AGENTS.md (~100-200 lines)
```markdown  
# Project Name

## Project Snapshot
[3-5 lines: repo type, tech stack, note about sub-AGENTS.md files]

## Root Setup Commands
[5-10 lines: install all, build all, typecheck all, test all]

## Universal Conventions
[5-10 lines: code style, commit format, branch strategy, PR requirements]

## Security & Secrets
[3-5 lines: never commit tokens, .env patterns, PII handling]

## JIT Index
### Package Structure
- [Package]: `[path]/` -> [see [path]/AGENTS.md]([path]/AGENTS.md)
- [Service]: `[path]/` -> [see [path]/AGENTS.md]([path]/AGENTS.md)

### Quick Find Commands
- Search function: `rg -n "functionName" apps/** packages/**`
- Find component: `rg -n "export.*ComponentName" apps/web/src`  
- Find API routes: `rg -n "export const (GET|POST)" apps/api`

## Definition of Done
[3-5 lines: what must pass before PR]
```

#### Sub-Folder AGENTS.md

**CRITICAL: Technology Detection Required**
Before loading any template, you MUST detect the actual technologies used in each package.

**CONDITIONAL LOADING**: Load domain-specific templates based on DETECTED package type:
- **Design System/UI**: Load `references/design-system-template.md` ONLY if UI components detected
- **Database/Data Layer**: Load `references/database-template.md` ONLY if database/ORM detected
- **API/Backend**: Load `references/api-template.md` ONLY if backend framework detected
- **Testing Focus**: Load `references/testing-template.md` ONLY if testing framework detected

**Do NOT load** multiple templates for a single package.
**Do NOT load** templates without first detecting matching technologies.

For each major package/service, use this structure:

```markdown
# Package Name

## Package Identity
[2-3 lines: what it does, primary tech]

## Setup & Run
[5-10 lines: install, dev, build, test, lint commands for this package]

## Patterns & Conventions
[10-20 lines - MOST IMPORTANT SECTION]
- File organization rules
- Naming conventions
- Examples with actual file paths:
  - DO: Use pattern from `src/components/Button.tsx`
  - DON'T: Class components like `src/legacy/OldButton.tsx`

## Key Files  
[5-10 lines: important files to understand this package]
- Auth: `src/auth/provider.tsx`
- API client: `src/lib/api.ts`
- Types: `src/types/index.ts`

## JIT Index Hints
[5-10 lines: search commands specific to this package]
- Find component: `rg -n "export function .*" src/components`
- Find hook: `rg -n "export const use" src/hooks`

## Local Skills
[Reference applicable skills for this package]

## Common Gotchas
[3-5 lines if applicable - package-specific issues]

## Pre-PR Checks
[2-3 lines: copy-paste command for this package]
```

## Phase 3: Quality Validation

Before finalizing, verify:
- [ ] Root AGENTS.md under 200 lines (for hierarchical)
- [ ] Root links to all sub-AGENTS.md files  
- [ ] Each sub-file has concrete examples (actual paths)
- [ ] Commands are copy-paste ready
- [ ] No duplication between root and sub-files
- [ ] JIT hints use actual patterns (ripgrep, find, glob)
- [ ] Every "DO" has real file example
- [ ] Every "DON'T" references real anti-pattern
- [ ] Pre-PR checks are single commands
- [ ] Local skills are properly referenced

## Troubleshooting

**When encountering issues**, load `references/troubleshooting.md` for solutions to common problems like unclear monorepo structure, too many skills, or mixed tech stacks.

## File Management

### Setup
1. Create `AGENTS.md` at appropriate location(s)
2. Create symlink: `ln -s AGENTS.md AI-DOCS.md` (optional)

### Location Strategy
- **Project root**: `AGENTS.md` - Primary instructions and references
- **Subdirectories**: `subdirectory/AGENTS.md` - Package/folder-scoped rules
- **Nested support**: AI agents combine closest AGENTS.md with parent ones

### When to Create Sub-Files
Create subdirectory AGENTS.md when:
- Package has significantly different patterns
- Different tech stack from main project
- Complex domain-specific rules  
- Different team ownership

## Workflow Summary

1. **Analyze** - Run discovery commands, assess complexity
2. **Choose approach** - Simple vs hierarchical structure  
3. **Generate** - Create appropriate AGENTS.md file(s)
4. **Reference skills** - Link to existing local skills
5. **Validate** - Check against quality checklist
6. **Test** - Verify commands work and files are accessible

This unified approach handles everything from simple project setup to complex monorepo documentation generation while maintaining the minimal, actionable style that makes AGENTS.md effective for AI agents.