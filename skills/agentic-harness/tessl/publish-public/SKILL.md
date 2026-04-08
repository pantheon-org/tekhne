---
name: publish-public
description: "Ensure Tessl tiles meet all requirements for public registry publication with comprehensive validation. Use when publishing skills to public registry, validating tile.json configuration, creating evaluation scenarios, checking quality thresholds (A-grade ≥108/120), or preparing skills for release. Validates eval scenario coverage, tile.json fields (name, version, private, summary, skills), agent-agnostic compliance, and publication readiness."
---

# Tessl Public Publication Skill

Ensure Tessl tiles (skills) meet all requirements for public registry publication, including evaluation scenario coverage, quality thresholds, and proper tile configuration.

## Principles

Public skills represent a quality commitment to every agent that installs them. Apply this skill when you need to validate, prepare, or execute a public registry publication.

**When to use**: Publishing a skill to the public Tessl registry, verifying publication readiness, or enforcing quality gates before release.

**When not to**: Private workspace publishing, internal-only skills, or pre-alpha prototypes where A-grade quality is not yet expected.

The three non-negotiable gates are:

1. Quality audit passes (≥108/120 A-grade via skill-quality-auditor)
2. Evaluation scenarios exist (minimum 5 scenarios with measurable success criteria)
3. Tile.json is correctly configured (`private: false`, valid fields)

## Workflow

### 1. Pre-Publication Quality Audit

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store
cat .context/audits/<domain>/<skill-name>/latest/analysis.md
```

If below A-grade, review the remediation plan and re-audit after improvements.

### 2. Create Evaluation Scenarios

```bash
mkdir -p skills/<domain>/<skill-name>/evals/scenario-01
```

Each scenario needs: `task.md`, `criteria.json` (checklist summing to 100), `capability.txt`. Target 5-8 scenarios covering the full trigger surface.

### 3. Configure Tile.json

```json
{
  "name": "workspace/skill-name",
  "version": "1.0.0",
  "private": false,
  "summary": "Descriptive text with use cases. Keywords: term1, term2",
  "skills": { "skill-id": { "path": "SKILL.md" } }
}
```

All 5 fields are required. `private: false` must be boolean, not string.

See `references/tile-json-schema.md` for field validation rules and examples.

### 4. Run Tessl Optimization

```bash
tessl skill review skills/<domain>/<skill-name>
tessl skill review skills/<domain>/<skill-name> --optimize
```

Target ≥90% from Tessl (independent of skill-quality-auditor).

### 5. Validate Agent-Agnostic Compliance

Prohibited: harness-specific tool references, IDE-specific commands, platform-specific paths.
Required: universal tools only (Read, Write, Edit, Bash, Grep, Glob).

### 6. Publish

```bash
sh scripts/check-publication-readiness.sh skills/<domain>/<skill-name>
tessl skill publish skills/<domain>/<skill-name> --public
tessl search <skill-name>
```

## Anti-Patterns

<!-- BAD: publish directly | GOOD: run all gates first -->

- **NEVER skip evaluation scenarios** — WHY: public registry requires proof of effectiveness via measurable scenarios
- **NEVER publish below A-grade (108/120)** — WHY: sub-threshold skills erode registry quality and may be flagged
- **NEVER set `private: true`** — WHY: tile.json defaults to private; must be explicitly set to `false`
- **NEVER skip `--optimize` when below 90%** — WHY: optimization routinely lifts scores from 85% → 99%
- **NEVER use harness-specific tool calls** — WHY: public skills must work across all agent platforms

See `references/anti-patterns.md` for detailed examples with remediation steps.

## Gotchas

- **Eval scenarios are NOT optional**: Public skills require evaluation scenarios
- **Quality audit ≠ Tessl review**: Both are independent checks; both must pass
- **Version bumping required**: Republishing requires a version increment
- **No rollback**: Published versions are immutable; only newer versions supersede

## References

- [Tile JSON Schema](references/tile-json-schema.md) — field documentation, validation rules, and examples for tile.json
- [Public Publication Requirements](references/public-publication-requirements.md) — full checklist of gates, compliance rules, and registry acceptance criteria
