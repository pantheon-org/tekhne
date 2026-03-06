# Phase 3: Reference File Audit (Existing Hubs)

**Duration:** 30-45 minutes
**Purpose:** Verify that the three already-converted hubs have substantive reference files — not empty stubs

## Skills to Verify

| Skill | Hub SKILL.md | References |
|-------|-------------|------------|
| `nx-bun-integration` | 128 lines | bun-runtime-api.md, migration-from-node.md, nx-executor-configuration.md, testing-with-bun.md |
| `nx-biome-integration` | 135 lines | biome-configuration-deep-dive.md, migration-guide.md, plugin-patterns.md |
| `nx-workspace-patterns` | 178 lines | project-graph-configuration.md, caching-strategies.md, project-boundaries.md, affected-commands.md |

## Tasks

### 1. Check Line Counts on All Reference Files

```bash
find skills/repository-mgmt/nx/bun-integration/references \
     skills/repository-mgmt/nx/biome-integration/references \
     skills/repository-mgmt/nx/workspace-patterns/references \
  -type f -name '*.md' | xargs wc -l | sort -n
```

**Threshold:** Any file ≤ 10 lines is a stub and must be filled.

**Success Criteria:**
- [ ] All reference files > 10 lines

### 2. Spot-Read Each Reference File Header

Read the first 10 lines of each reference file to confirm it has a real heading and content:

```bash
for f in $(find skills/repository-mgmt/nx/bun-integration/references \
               skills/repository-mgmt/nx/biome-integration/references \
               skills/repository-mgmt/nx/workspace-patterns/references \
           -name '*.md'); do
  echo "=== $f ==="; head -5 "$f"; echo
done
```

**Success Criteria:**
- [ ] Each file starts with a `#` heading and has substantive content

### 3. Verify Hub SKILL.md Quick Reference Links Are Accurate

For each hub, confirm that every link in the Quick Reference table points to an existing file:

```bash
grep "references/" skills/repository-mgmt/nx/bun-integration/SKILL.md
grep "references/" skills/repository-mgmt/nx/biome-integration/SKILL.md
grep "references/" skills/repository-mgmt/nx/workspace-patterns/SKILL.md
```

Compare each path against the actual files found in step 1.

**Success Criteria:**
- [ ] All links resolve to existing files (no broken references)

### 4. Fill Any Stubs Found

If any reference file is a stub (≤ 10 lines or empty):
- Read the hub's SKILL.md to understand what the reference should cover
- Write substantive content based on the topic — real commands, patterns, configuration examples
- Minimum target: 40 lines per reference file

## Outputs

- Line count table for all reference files
- List of stubs found (if any) and whether they were filled
- Broken link list (if any) and corrections applied

## Next Phase

[Phase 4: Quality Validation](../phase-4-quality-validation/)
