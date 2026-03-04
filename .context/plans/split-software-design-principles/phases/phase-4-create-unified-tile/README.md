# Phase 4: Create Unified tile.json

**Status:** Pending  
**Effort:** ~15 minutes  
**Dependencies:** Phase 3

## Description

Create a single `tile.json` at the design-principles root that references all 4 focused skills.

## Content

**Path:** `skills/software-engineering/design-principles/tile.json`

```json
{
  "name": "pantheon-ai/design-principles",
  "version": "0.1.0",
  "private": false,
  "summary": "Apply software design principles across tactical class design (SOLID), strategic architecture (Clean Architecture), structural patterns, and testable system design; use for code reviews, refactoring, and design decisions.",
  "skills": {
    "solid-principles": {
      "path": "solid-principles/SKILL.md"
    },
    "clean-architecture": {
      "path": "clean-architecture/SKILL.md"
    },
    "design-patterns": {
      "path": "design-patterns/SKILL.md"
    },
    "testable-design": {
      "path": "testable-design/SKILL.md"
    }
  }
}
```

## Acceptance Criteria

- [ ] `tile.json` created at design-principles root
- [ ] All 4 skills referenced with correct paths
- [ ] Version set to 0.1.0
- [ ] Private set to false (for Tessl publication)
- [ ] Summary accurately describes all 4 skills
- [ ] Valid JSON format

## Verification

```bash
# Validate JSON syntax
bunx @biomejs/biome check skills/software-engineering/design-principles/tile.json

# Verify skill paths exist
for skill_path in $(jq -r '.skills[].path' skills/software-engineering/design-principles/tile.json); do
  full_path="skills/software-engineering/design-principles/$skill_path"
  if [ -f "$full_path" ]; then
    echo "✓ $skill_path exists"
  else
    echo "✗ $skill_path NOT FOUND"
  fi
done

# Check required fields
jq -e '.name, .version, .private, .summary, .skills' skills/software-engineering/design-principles/tile.json > /dev/null && echo "✓ All required fields present"
```

## Notes

- Follows existing tile.json conventions from other skills
- Single tile with multiple skills (similar to nx-plugin-toolkit pattern)
- Summary must be concise (<200 chars) but comprehensive
