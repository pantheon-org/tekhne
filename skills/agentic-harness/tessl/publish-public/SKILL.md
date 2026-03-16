---
name: tessl-publish-public
description: >
  Ensure Tessl tiles meet all requirements for public registry publication with
  comprehensive validation. Use when publishing skills to public registry, validating
  tile.json configuration, creating evaluation scenarios, checking quality thresholds
  (≥108/120 A-grade), or preparing skills for release. Validates eval scenario coverage,
  tile.json fields (name, version, private, summary, skills), agent-agnostic compliance,
  and publication readiness.
---

# Tessl Public Publication Skill

Ensure Tessl tiles (skills) meet all requirements for public registry publication, including evaluation scenario coverage, quality thresholds, and proper tile configuration.

> **Not applicable for**: private skill publishing, internal workspace-only development, or pre-alpha prototyping.

## Scope

| What's Included | What's NOT Included |
|----------------|---------------------|
| Evaluation scenario creation and validation | Writing the skill content itself |
| Quality threshold enforcement (A-grade ≥108/120) | General skill authoring guidance |
| Tile.json configuration for public publishing | Private workspace skill publishing |
| Publication readiness validation workflow | Skill content optimization |
| Agent-agnostic compliance checking | Skill-quality-auditor execution (separate tool) |
| Cross-platform compatibility validation | Post-publication monitoring |
| Registry submission preparation | |

## Workflow

Every public skill must prove effectiveness via eval scenarios, meet the A-grade quality bar (≥108/120), be agent-agnostic, and show measurable ROI in agent performance. The following steps enforce these principles.

### 1. Pre-Publication Quality Audit

Run skill-quality-auditor to ensure A-grade minimum:

```bash
# Run quality audit and store results
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store

# Review audit results
cat .context/audits/<domain>/<skill-name>/latest/analysis.md
```

**Quality Gate**: Skill MUST score ≥108/120 (A-grade) to proceed. If below threshold:

```bash
# Review remediation plan
cat .context/audits/<domain>/<skill-name>/latest/remediation-plan.md

# Address critical issues (D1, D2, D3, D5 dimensions typically)
# Re-run audit after improvements
```

### 2. Create Evaluation Scenarios

Generate comprehensive eval scenarios that prove skill effectiveness:

```bash
# Create evaluation scenarios directory if not exists
mkdir -p skills/<domain>/<skill-name>/evaluation-scenarios

# Create scenario files
touch skills/<domain>/<skill-name>/evaluation-scenarios/scenario-{01..05}.md
```

Each scenario file must include:

- **Scenario name**: Clear, descriptive title
- **User prompt**: Exact input the agent receives
- **Expected behavior**: What the agent should do (step-by-step)
- **Success criteria**: Measurable outcomes (files created, commands run, outputs produced)
- **Failure conditions**: What indicates the skill was not used or failed

**Example scenario structure**:

```markdown
# Scenario 01: Basic Public Publication Check

## User Prompt
"Is this skill ready to publish publicly on Tessl?"

## Expected Behavior
1. Agent locates the skill directory
2. Checks for evaluation-scenarios/ directory
3. Verifies tile.json exists with private: false
4. Runs or checks for recent skill-quality-auditor results
5. Reports publication readiness status

## Success Criteria
- Agent identifies all missing requirements
- Provides specific remediation steps
- Does NOT attempt to publish if requirements unmet
- Reports A-grade score if available

## Failure Conditions
- Agent skips evaluation scenario check
- Agent publishes without verifying private: false
- Agent ignores quality audit results
- Agent provides generic "looks good" without validation
```

**Aim for 5-8 scenarios** covering:

- Basic publication readiness check
- Creating missing eval scenarios
- Quality threshold enforcement
- Tile.json configuration validation
- Agent-agnostic compliance check
- Cross-platform compatibility validation
- Full publication workflow execution
- Edge cases (missing files, low scores, private: true)

### 3. Configure Tile.json for Public Publishing

Validate and configure tile.json with ALL required fields. Missing any required field blocks publication.

**5 Required Fields**:

```json
{
  "name": "workspace/skill-name",
  "version": "1.0.0",
  "private": false,
  "summary": "Descriptive text with use cases. Keywords: term1, term2, term3",
  "skills": {
    "skill-id": {
      "path": "SKILL.md",
      "references": ["references/guide.md"]
    }
  }
}
```

**Critical validations**:

- `private: false` - Must be boolean (not string "false")
- `name` - Format: `workspace/tile-name` (lowercase, kebab-case)
- `version` - Semantic: `x.y.z` (no `v` prefix)
- `summary` - 150-300 chars, embed keywords inline (NOT separate array)
- `skills` - Non-empty, paths exist, SKILL.md has frontmatter

**Optional fields**: `files` (root-level), `references`/`resources` (skill-level), `docs`

See `references/tile-json-schema.md` for complete documentation with validation rules, examples, and anti-patterns.

### 4. Run Tessl Optimization

Use Tessl's built-in optimization to maximize quality scores:

```bash
# Initial quality assessment
tessl skill review skills/<domain>/<skill-name>

# If score < 90%, run optimization
tessl skill review skills/<domain>/<skill-name> --optimize

# Verify improvements
tessl skill review skills/<domain>/<skill-name>
```

**Target**: ≥90% quality score from Tessl (independent of skill-quality-auditor)

### 5. Validate Agent-Agnostic Compliance

Ensure skill works across ALL agent harnesses:

**Prohibited**:

- Agent-specific tools (claude-code-specific, cursor-specific)
- Harness-specific instructions ("use Claude Code's X feature")
- Platform-specific behaviors (VS Code commands, IDE integrations)

**Required**:

- Universal tools only (Read, Write, Edit, Bash, Grep, Glob)
- Conditional capability checks when using advanced features
- Cross-platform compatibility (works in CLI agents, IDE agents)

### 6. Execute Publication

Once all gates pass, publish to public registry:

```bash
# Final readiness check
sh skills/agentic-harness/tessl/publish-public/scripts/check-publication-readiness.sh skills/<domain>/<skill-name>

# Publish to public registry
tessl skill publish skills/<domain>/<skill-name> --public

# Verify publication
tessl search <skill-name>
```

## Quick Commands

```bash
# Full publication readiness check
sh skills/agentic-harness/tessl/publish-public/scripts/check-publication-readiness.sh skills/<domain>/<skill-name>

# Create eval scenario template
mkdir -p skills/<domain>/<skill-name>/evaluation-scenarios
cat > skills/<domain>/<skill-name>/evaluation-scenarios/scenario-01.md << 'EOF'
# Scenario 01: [Description]

## User Prompt
"[Exact user input]"

## Expected Behavior
1. [Step 1]
2. [Step 2]

## Success Criteria
- [Measurable outcome 1]
- [Measurable outcome 2]

## Failure Conditions
- [Condition indicating skill not used]
- [Condition indicating skill failed]
EOF

# Validate tile.json for public publishing
jq '.private' skills/<domain>/<skill-name>/tile.json

# Check quality audit score
cat .context/audits/<domain>/<skill-name>/latest/analysis.md | grep "Total Score"

# Publish to public registry
tessl skill publish skills/<domain>/<skill-name> --public
```

## Gotchas

- **Eval scenarios are NOT optional**: Public skills require evaluation scenarios to prove effectiveness
- **Private: true blocks publishing**: tile.json must have `private: false` explicitly set
- **Quality audit != Tessl review**: Both are independent; skill-quality-auditor (≥108/120) and Tessl (≥90%) must pass
- **Version bumping required**: Republishing existing skill requires version increment
- **Agent-agnostic violations fail silently**: Skills with harness-specific tools work locally but fail cross-platform
- **Optimization is critical**: `tessl skill review --optimize` can boost scores 85% → 99%
- **Eval scenario quality matters**: Generic "agent does X" scenarios fail validation; need specific success criteria

## Production Caveats

- **No rollback mechanism**: Published skills cannot be unpublished; only newer versions can supersede
- **Registry propagation delay**: Public skills may take 5-10 minutes to appear in search results
- **Version immutability**: Once published, a specific version cannot be modified
- **Quality degradation**: Skills scoring <80% on Tessl review may be flagged or delisted
- **Eval scenario enforcement**: Future Tessl versions may require passing automated eval runs before publishing
- **License compliance**: Ensure license in tile.json matches actual content licensing
- **Keyword spam detection**: Excessive or irrelevant keywords may trigger registry moderation

## Anti-Patterns

See `references/anti-patterns.md` for detailed examples. Summary:

- **NEVER skip evaluation scenarios** — minimum 5 scenarios with specific success criteria required
- **NEVER publish below A-grade (108/120)** — audit, remediate, and re-audit until threshold is met
- **NEVER omit `private: false`** — tile.json defaults to `private: true`; must be set explicitly
- **NEVER skip `--optimize` when below 90%** — optimization can boost scores from 85% → 99%
- **NEVER use agent-specific tools** — public skills must use universal tools only (Bash, Edit, Read, Write, Grep, Glob)

## Related Skills

- **skill-quality-auditor**: Internal quality improvement and dimensional guidance (required pre-requisite)
- **creating-eval-scenarios**: Generate evaluation scenarios for Tessl tiles (used in workflow step 2)
- **tile-creator**: Create new Tessl tiles with proper structure
- **skill-review-optimizer**: Automate iterative skill improvement using Tessl review feedback

## Success Metrics

- **100% eval scenario coverage**: All public skills have 5-8 comprehensive scenarios
- **A-grade minimum**: All public skills score ≥108/120 on skill-quality-auditor
- **90%+ Tessl score**: All public skills score ≥90% on `tessl skill review`
- **Agent-agnostic compliance**: Zero harness-specific tools or instructions in public skills
- **Zero failed publications**: All publication attempts succeed on first try after validation
- **Registry acceptance rate**: 100% of submitted skills accepted without moderation flags

## References

| Task | Command |
|------|---------|
| Check publication readiness | `sh scripts/check-publication-readiness.sh skills/<domain>/<skill>` |
| Run quality audit | `sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill> --json --store` |
| View audit results | `cat .context/audits/<domain>/<skill>/latest/analysis.md` |
| Create eval scenario | `touch skills/<domain>/<skill>/evaluation-scenarios/scenario-01.md` |
| Verify private flag | `jq '.private' skills/<domain>/<skill>/tile.json` |
| Set private to false | `jq '.private = false' skills/<domain>/<skill>/tile.json > tmp.json && mv tmp.json skills/<domain>/<skill>/tile.json` |
| Run Tessl review | `tessl skill review skills/<domain>/<skill>` |
| Optimize Tessl score | `tessl skill review skills/<domain>/<skill> --optimize` |
| Publish to public registry | `tessl skill publish skills/<domain>/<skill> --public` |
| Verify publication | `tessl search <skill-name>` |
