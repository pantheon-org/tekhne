# Phase 0: State Audit

**Duration:** 15-20 minutes
**Purpose:** Confirm exact current state of all 5 skills before making any changes

## Tasks

### 1. Count Lines in Each SKILL.md

```bash
wc -l \
  skills/repository-mgmt/nx/bun-integration/SKILL.md \
  skills/repository-mgmt/nx/biome-integration/SKILL.md \
  skills/repository-mgmt/nx/workspace-patterns/SKILL.md \
  skills/documentation/conventional-commits/SKILL.md \
  skills/repository-mgmt/nx/extending-plugins/SKILL.md
```

**Success Criteria:**
- [ ] Line counts recorded

### 2. List All Files in Each Skill Directory

```bash
find skills/repository-mgmt/nx/bun-integration \
     skills/repository-mgmt/nx/biome-integration \
     skills/repository-mgmt/nx/workspace-patterns \
     skills/documentation/conventional-commits \
     skills/repository-mgmt/nx/extending-plugins \
  -type f | sort
```

**Success Criteria:**
- [ ] File inventory complete; confirm which have tile.json, which have references/

### 3. Check for Hub Pattern in Each SKILL.md

For each skill, grep for the Navigation Hub marker:

```bash
grep -l "Navigation hub\|navigation hub" \
  skills/repository-mgmt/nx/bun-integration/SKILL.md \
  skills/repository-mgmt/nx/biome-integration/SKILL.md \
  skills/repository-mgmt/nx/workspace-patterns/SKILL.md \
  skills/documentation/conventional-commits/SKILL.md \
  skills/repository-mgmt/nx/extending-plugins/SKILL.md
```

**Success Criteria:**
- [ ] Confirmed which skills use hub pattern

### 4. Check tile.json Presence

```bash
find skills/repository-mgmt/nx/extending-plugins \
     skills/documentation/conventional-commits \
  -name "tile.json"
```

**Success Criteria:**
- [ ] Confirmed tile.json presence/absence for the two incomplete skills

## Outputs

- Confirmed line counts for all 5 skills
- File inventory (which have references/, tile.json)
- List of skills without hub structure

## Next Phase

[Phase 1: Convert extending-nx-plugins](../phase-1-extending-nx-plugins/)
