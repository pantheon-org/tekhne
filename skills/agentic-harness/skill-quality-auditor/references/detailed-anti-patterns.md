---
category: framework
priority: CRITICAL
source: evaluation criteria analysis
---

# Detailed Anti-Patterns for Skill Quality

Critical failure modes to avoid when evaluating and improving skill quality.

## Core Anti-Patterns

### NEVER skip baseline comparison in recurring audits

- **WHY**: score changes are meaningless without prior reference points.
- **BAD**: run ad hoc audits with no previous report linkage.
- **GOOD**: compare current results to previous dated audits.

### NEVER aggregate low-similarity skills

- **WHY**: merging unrelated skills harms discoverability and intent routing.
- **BAD**: aggregate different domains with weak overlap.
- **GOOD**: aggregate only when similarity and domain fit are clear.

### NEVER ship remediation plans without validation checks

- **WHY**: invalid or incomplete plans create execution drift.
- **BAD**: write plan and execute blindly.
- **GOOD**: validate schema/format and ensure deterministic success criteria.

### NEVER ignore Knowledge Delta scoring when it's below 15/20

- **WHY**: Knowledge Delta is the highest-weighted dimension and signals expert-only content gaps.
- **BAD**: accept scores of 10-14 without investigation.
- **GOOD**: prioritize Knowledge Delta improvements first, target ≥17/20 for A-grade skills.

### NEVER apply subjective scoring without deterministic checks

- **WHY**: human judgment varies and creates inconsistent audit results.
- **BAD**: rely on manual assessment for quality gates.
- **GOOD**: use automated scripts and measurable criteria for consistency.

### NEVER run evaluations without proper skill directory structure

- **WHY**: missing templates/, scripts/, or references/ directories indicate incomplete skill organization.
- **BAD**: evaluate skills with ad hoc file placement.
- **GOOD**: enforce conventional directory structure before quality assessment.

### NEVER use harness-specific paths in skill content

- **WHY**: paths like `.opencode/`, `.claude/`, `.cursor/` break cross-harness portability when skills are synced to other agents.
- **BAD**: reference `.opencode/scripts/setup.sh` in instructions.
- **GOOD**: use relative paths from skill directory: `scripts/setup.sh`.
- **IMPACT**: skill fails to load assets when synced to Cursor, Gemini CLI, Aider, or 40+ other agents.

### NEVER mention specific agent names in skill instructions

- **WHY**: skills should work across all agentic harnesses following the Agent Skills specification.
- **BAD**: "For Claude Code users, run...", "Cursor Agent should...", "GitHub Copilot can...".
- **GOOD**: use generic agent-agnostic instructions that work everywhere.
- **IMPACT**: creates confusion and excludes users of other agents unnecessarily.

### NEVER create kitchen-sink skills that cover multiple unrelated tasks

- **WHY**: broad-scope skills violate single responsibility and the D4 Task Focus Declaration ("ONE type of task"). They reduce activation accuracy because the description must be vague enough to cover all sub-topics, making it harder for agents to know when to trigger the skill. Maintenance suffers because changes to one sub-topic risk breaking unrelated sections.
- **BAD**: a single skill covering configuration, linting, formatting, and migration for a tool (e.g., "biome-complete" merging biome-configuration and biome-linting).
- **GOOD**: split into focused skills with clear single-purpose scope (e.g., `biome-generator` for creating configs, `biome-validator` for linting/checking). Use a consolidated `tile.json` to ship related skills together for distribution without bloating individual skill scope.
- **IMPACT**: agents activate the skill in wrong contexts (low precision), content becomes generic instead of expert-level (hurts D1 Knowledge Delta), and the skill resists the Navigation Hub pattern because there is no clear "hub" when the skill tries to be everything.
- **DETECTION**: description contains multiple "and"/"or" connectors suggesting unrelated tasks; skill covers >2 distinct workflows; description >300 characters to capture all sub-topics.

### NEVER list references without explicit lazy-load conditions

- **WHY**: agents default to loading all referenced files eagerly when no "When to Use" conditions are provided. This wastes context on irrelevant content and degrades effective working memory — especially harmful for large skill collections with 10+ references.
- **BAD**: a References section that is a plain bullet list of file paths, or a table where the "When to Use" column says only "for reference", "for more detail", or echoes the topic name.
- **GOOD**: a 3-column References table where every "When to Use" cell names the specific task condition that triggers loading that file (e.g. "When diagnosing a D3 failure", "Only when preparing a CI gate config", "Skip unless calculating a final grade").
- **ALSO BAD**: an AGENTS.md that says "load all references before starting" or does not explicitly instruct agents to load only what the current task requires.
- **IMPACT**: skills without lazy-load guidance consume 3-10x more context than necessary. Agents loading 15 reference files upfront lose ~40k tokens of working memory before the actual task begins.
- **DETECTION**: check the References table — if the "When to Use" column is missing, vague, or generic (not task-specific), deduct 3 points from D5.

### NEVER use non-standard heading names for the References section

- **WHY**: inconsistent headings (`## Resources`, `## Quick Reference`, `## Bundled Resources`, `## Reference Documentation`, `## Helper Scripts`) prevent automated detection and make audits non-deterministic. The only accepted heading is `## References`.
- **BAD**: `## Resources`, `## Quick Reference`, `## Bundled Resources`, `## Reference Documentation`, `## Helper Scripts`, `## See Also`.
- **GOOD**: `## References` — always, everywhere, without exception.
- **ALSO BAD**: bare file paths (`references/file.md`) or bare URLs (`https://example.com`) without markdown links; missing `— description` labels on links; placing the section anywhere other than the last H2 position.
- **IMPACT**: 36% of current skills have no References section and the remaining 64% use 6 different heading variants, creating audit noise, breaking grep-based detection, and signalling low specification compliance (D4 penalty).
- **DETECTION**: check last H2 heading in SKILL.md — if not `## References` (or absent when links exist), award 0 for the D4 References Section Format bonus.

## Impact Analysis

Each anti-pattern leads to specific failure modes:

- **Baseline skipping**: Inability to measure progress over time
- **Low-similarity aggregation**: Reduced skill activation accuracy
- **Unvalidated remediation**: Implementation drift and wasted effort
- **Knowledge Delta neglect**: Generic content that doesn't add expert value
- **Subjective scoring**: Inconsistent quality gates across evaluations
- **Structure violations**: Poor maintainability and discoverability
- **Kitchen-sink scope**: Low activation precision, generic content, maintenance fragility
- **Eager reference loading**: 3-10x context waste, degraded agent working memory
- **Non-standard References section**: Audit noise, grep-detection failures, D4 bonus not awarded
