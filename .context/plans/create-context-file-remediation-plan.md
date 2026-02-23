---
plan_date: 2026-02-23
skill_name: create-context-file
source_audit: .context/audits/create-context-file-audit-2026-02-22.md
---

# Remediation Plan: create-context-file

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 88/120 (73%) | 104/120 (87%) |
| **Grade** | C | B+ |
| **Priority** | Medium | - |

**Verdict**: Priority improvements required. Focus on anti-pattern quality and procedural clarity.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Weak anti-pattern precision | D3 (8/15) | High | Agents may repeat common mistakes |
| Missing deterministic workflow | D2 (10/15) | Medium | Execution ambiguity |
| Limited command examples | D8 (11/15) | Medium | Reduced usability |
| Over-constrained flexibility | D6 (10/15) | Medium | Context adaptation issues |

## Detailed Remediation Steps

### Phase 1: Anti-Pattern Quality (D3) - HIGH PRIORITY

**File**: `skills/create-context-file/SKILL.md`

1. **Add explicit anti-patterns section** after the workflow section:

```markdown
## Anti-Patterns

### NEVER use generic filenames
- **WHY**: Filenames without context are unsearchable
- **BAD**: `.context/plan.md`
- **GOOD**: `.context/plans/auth-system-implementation-plan.md`

### NEVER skip frontmatter
- **WHY**: Metadata enables tooling and traceability
- **CONSEQUENCE**: Plans become untrackable and hard to reference

### NEVER create files outside `.context/`
- **WHY**: Repository organization and gitignore patterns
- **CONSEQUENCE**: Files may be committed to wrong locations

### NEVER use emoji in plan content
- **WHY**: Plain text compatibility across tools
- **BAD**: "Great progress!"
- **GOOD**: "Progress: completed authentication module"
```

2. **Add repository-specific risk warnings**:

```markdown
### Repository-Specific Risks
- This repository uses kebab-case for all context files
- Three-word IDs must be unique across all `.context/` subdirectories
- Plans exceeding 500 lines should be split into sub-plans
```

### Phase 2: Workflow Determinism (D2) - MEDIUM PRIORITY

**File**: `skills/create-context-file/SKILL.md`

1. **Rewrite workflow as numbered steps with explicit outcomes**:

```markdown
## Workflow

### Step 1: Validate Request
- **Input**: User request for context/plan/scratch file
- **Action**: Confirm file type and purpose
- **Output**: Confirmed file type (plan | scratch | note)
- **Exit condition**: User confirms intent

### Step 2: Generate Unique ID
- **Input**: File topic/context
- **Action**: Generate three-word kebab-case identifier
- **Output**: Unique identifier (e.g., `auth-flow-design`)
- **Exit condition**: ID does not exist in `.context/`

### Step 3: Create File with Frontmatter
- **Input**: File type, unique ID
- **Action**: Create file at `.context/{type}s/{id}-{type}.md`
- **Output**: File with valid YAML frontmatter
- **Exit condition**: File created and frontmatter validated

### Step 4: Populate Content
- **Input**: User requirements
- **Action**: Write structured content based on file type
- **Output**: Complete file with all required sections
- **Exit condition**: All required sections present
```

2. **Add "When to Use / When Not to Use" section**:

```markdown
## Scope Control

### Use This Skill When
- Creating implementation plans
- Writing design rationales
- Documenting scratchpad analysis
- Storing temporary working notes

### Do NOT Use This Skill When
- Creating permanent documentation (use docs/)
- Writing skill content (use skills/)
- Creating configuration files (use config/)
```

### Phase 3: Practical Usability (D8) - MEDIUM PRIORITY

**File**: `skills/create-context-file/SKILL.md`

1. **Add Quick Commands section**:

````markdown
## Quick Commands

### Create a new plan
```bash
mkdir -p .context/plans && touch ".context/plans/{three-word-id}-plan.md"
```

### Validate frontmatter
```bash
yq eval '.' ".context/plans/{filename}.md" 2>/dev/null && echo "Valid YAML"
```

### List existing context files
```bash
find .context -name "*.md" -type f | sort
```

### Check for ID collisions
```bash
find .context -name "*{three-word-id}*" -type f
```
````

2. **Add expected output examples**:

````markdown
## Expected Outputs

### Plan file structure
```
.context/plans/
├── auth-system-plan.md
├── database-migration-plan.md
└── api-redesign-plan.md
```

### Frontmatter template
```yaml
---
created: 2026-02-23
type: plan
status: draft | in-progress | complete
related: [optional list of related files]
---
```
````

### Phase 4: Freedom Calibration (D6) - MEDIUM PRIORITY

**File**: `skills/create-context-file/SKILL.md`

1. **Add flexibility guidance**:

```markdown
## Flexibility Guidelines

### Hard Constraints (NEVER violate)
- Files must be in `.context/` directory
- Frontmatter must include `created` date
- Filenames must use kebab-case

### Flexible Decisions (adapt to context)
- Section structure within files (adapt to content type)
- Level of detail (match complexity of task)
- Additional frontmatter fields (add as needed)

### Fallback Paths
- If file type unclear: default to `scratch`
- If ID collision: append numeric suffix (e.g., `-v2`)
- If directory missing: create automatically
```

## Verification Commands

```bash
# Re-run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh create-context-file --json

# Validate against format spec
skills/skill-quality-auditor/scripts/validate-review-format.sh .context/audits/create-context-file-audit-2026-02-22.md

# Check for duplication issues
skills/skill-quality-auditor/scripts/detect-duplication.sh skills

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
```

## Success Criteria

| Criterion | Measurement | Target |
| --- | --- | --- |
| D3: Anti-Pattern Quality | Score increase | >= 12/15 |
| D2: Mindset + Procedures | Score increase | >= 13/15 |
| D8: Practical Usability | Score increase | >= 13/15 |
| D6: Freedom Calibration | Score increase | >= 12/15 |
| Overall Score | Total points | >= 104/120 |
| Grade | Letter grade | >= B+ |

## Effort Estimate

- **T-shirt size**: M (4-6 hours)
- **Complexity**: Low-Medium
- **Risk**: Low (additive changes only)

## Dependencies

- None (standalone skill)

## Notes

- Consider creating a `references/` directory if content grows beyond 100 lines
- Template files in `templates/` could standardize frontmatter formats
