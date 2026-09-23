---
name: publish-public
description: "Ensure Tessl plugins meet all requirements for public registry publication with comprehensive validation. Use when publishing skills to public registry, validating .tessl-plugin/plugin.json configuration, creating evaluation scenarios, checking quality thresholds (A-grade >=126/140 on the nine-dimension skill-quality-auditor scale), or preparing plugins for release. Validates eval scenario coverage, plugin.json fields (name, version, private, description, skills), agent-agnostic compliance, and publication readiness."
---

# Tessl Public Publication Skill

Ensure Tessl plugins meet all requirements for public registry publication, including evaluation scenario coverage, quality thresholds, and proper plugin configuration.

## Principles

Public plugins represent a quality commitment to every agent that installs them. Apply this skill when you need to validate, prepare, or execute a public registry publication.

**When to use**: Publishing a plugin to the public Tessl registry, verifying publication readiness, or enforcing quality gates before release.

**When not to**: Private workspace publishing, internal-only plugins, or pre-alpha prototypes where A-grade quality is not yet expected.

The three non-negotiable gates are:

1. Quality audit passes (>=126/140 A-grade via skill-quality-auditor's nine-dimension framework)
2. Evaluation scenarios exist (minimum 5 scenarios with measurable success criteria)
3. plugin.json is correctly configured (`private: false`, valid fields)

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

### 3. Configure .tessl-plugin/plugin.json

```json
{
  "name": "workspace/skill-name",
  "version": "1.0.0",
  "private": false,
  "description": "Descriptive text with use cases. Keywords: term1, term2",
  "skills": ["SKILL.md"]
}
```

All 5 fields are required. `private: false` must be boolean, not string. `skills` is an array of path strings.

See `references/plugin-json-schema.md` for field validation rules and examples.

### 4. Run Tessl Review

```bash
tessl review run skills/<domain>/<skill-name>
tessl review run skills/<domain>/<skill-name> --optimize
```

Target >=90% from Tessl (independent of skill-quality-auditor).

### 5. Validate Agent-Agnostic Compliance

Prohibited: harness-specific tool references, IDE-specific commands, platform-specific paths.
Required: universal tools only (Read, Write, Edit, Bash, Grep, Glob).

### 6. Publish

```bash
sh scripts/check-publication-readiness.sh skills/<domain>/<skill-name>
tessl plugin publish --workspace pantheon-ai skills/<domain>/<skill-name> --bump patch
tessl search <skill-name>
```

## Anti-Patterns

- **NEVER skip evaluation scenarios** -- WHY: public registry requires proof of effectiveness via measurable scenarios

  ```bash
  # BAD - publish with no evals/ directory at all
  tessl plugin publish --workspace pantheon-ai skills/domain/skill-name --bump patch

  # GOOD - scenarios exist and pass before publishing
  ls skills/domain/skill-name/evals/scenario-*/  # >= 5 scenarios present
  tessl plugin publish --workspace pantheon-ai skills/domain/skill-name --bump patch
  ```

- **NEVER publish below A-grade (126/140)** -- WHY: sub-threshold plugins erode registry quality and may be flagged

  ```bash
  # BAD - trusts memory of a score from a previous session
  tessl plugin publish --workspace pantheon-ai skills/domain/skill-name --bump patch

  # GOOD - re-run the audit immediately before publishing, on the current content
  sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh domain/skill-name --json --store
  # confirm total >= 126/140 before proceeding
  tessl plugin publish --workspace pantheon-ai skills/domain/skill-name --bump patch
  ```

- **NEVER set `private: true`** -- WHY: plugin.json defaults to private; must be explicitly set to `false`

  ```json
  // BAD - field omitted, defaults to private and publish is a no-op for the public registry
  { "name": "workspace/skill-name", "version": "1.0.0" }

  // GOOD - explicitly false
  { "name": "workspace/skill-name", "version": "1.0.0", "private": false }
  ```

- **NEVER skip `--optimize` when below 90%** -- WHY: optimization routinely lifts scores from 85% to 99%
- **NEVER use harness-specific tool calls** -- WHY: public plugins must work across all agent platforms

## Gotchas

- **Eval scenarios are NOT optional**: Public plugins require evaluation scenarios
- **Quality audit != Tessl review**: Both are independent checks; both must pass
- **Version bumping required**: Republishing requires a version increment via `--bump`
- **No rollback**: Published versions are immutable; only newer versions supersede

## References

- [Plugin JSON Schema](references/plugin-json-schema.md) -- field documentation, validation rules, and examples for .tessl-plugin/plugin.json
- [Public Publication Requirements](references/public-publication-requirements.md) -- full checklist of gates, compliance rules, and registry acceptance criteria
