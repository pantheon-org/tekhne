# Phase 9: Tessl Integration

**Status:** Pending  
**Effort:** ~30 minutes  
**Dependencies:** Phase 8

## Description

Review and publish the design-principles tile to the Tessl public registry, ensuring all 4 skills meet the ≥90% score threshold.

## Tessl Workflow

### Option 1: CLI Automation (Recommended)

```bash
# Process entire tile (import, lint, review, publish)
bun cli/index.ts tessl manage software-engineering/design-principles

# With specific workspace
bun cli/index.ts tessl manage software-engineering/design-principles --workspace=pantheon-ai
```

The CLI automatically:
1. Imports skills without tile.json (if needed)
2. Lints and reviews skills with tile.json
3. Publishes skills that pass validation

### Option 2: Manual Commands

```bash
# Import tile (if needed)
tessl skill import skills/software-engineering/design-principles

# Review individual skills with optimization
tessl skill review skills/software-engineering/design-principles/solid-principles --optimize
tessl skill review skills/software-engineering/design-principles/clean-architecture --optimize
tessl skill review skills/software-engineering/design-principles/design-patterns --optimize
tessl skill review skills/software-engineering/design-principles/testable-design --optimize

# Publish tile to public registry
tessl skill publish skills/software-engineering/design-principles --public
```

## Target Scores

**Minimum for publication:** ≥90% (observed improvements: 85% → 99% with --optimize)

| Skill | Target Score | Notes |
|-------|--------------|-------|
| solid-principles | ≥90% | Focus on embedded examples quality |
| clean-architecture | ≥90% | Verify reference doc organization |
| design-patterns | ≥90% | Ensure pattern selection workflow is clear |
| testable-design | ≥90% | Validate testing architecture principles |

## Optimization Strategy

**If any skill scores <90%:**

1. **Run with --optimize flag:**
   ```bash
   tessl skill review <skill-path> --optimize
   ```
   This can dramatically improve scores (documented 85% → 99% improvements)

2. **Review optimization suggestions** in output

3. **Apply recommended changes** to SKILL.md

4. **Re-review** until ≥90% achieved

## Publication Checklist

- [ ] All 4 skills score ≥90% in Tessl review
- [ ] tile.json has `"private": false`
- [ ] Namespace is `pantheon-ai/design-principles`
- [ ] Version is set correctly (0.1.0)
- [ ] Summary is accurate and concise
- [ ] All skill paths resolve correctly

## Acceptance Criteria

- [ ] Tile successfully published to Tessl public registry
- [ ] All 4 skills accessible via `tessl search design-principles`
- [ ] Installation works: `tessl install pantheon-ai/design-principles`
- [ ] No errors during publication process

## Verification

```bash
# Search for published tile
tessl search "pantheon-ai/design-principles"

# Expected output: Shows tile with 4 skills

# Test installation (in separate test directory)
cd /tmp/test-install
tessl install pantheon-ai/design-principles

# Verify skills installed
ls .agents/skills/ | grep design-principles
```

## Post-Publication Tasks

- [ ] Update repository README with installation instructions
- [ ] Add tile to list of published skills (if maintained)
- [ ] Announce on relevant channels (if applicable)
- [ ] Tag repository with version: `git tag v0.1.0`

## Troubleshooting

**Issue:** Tessl review scores <90%  
**Solution:** Use `--optimize` flag, apply suggestions, re-review

**Issue:** Publication fails with "tile already exists"  
**Solution:** Increment version in tile.json, re-publish

**Issue:** Skills not showing in search  
**Solution:** Wait 5-10 minutes for registry indexing, check namespace spelling

**Issue:** `--skill` flag doesn't work with local paths  
**Solution:** For consolidated tiles, point to specific skill directory (not use --skill flag)

## Notes

- The `--optimize` flag is critical for skills scoring 85-89%
- Tessl reviews are complementary to internal quality audits (don't skip Phase 8)
- For consolidated tiles, review each skill directory individually
- Publication is all-or-nothing at the tile level (all 4 skills published together)
