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

## When to Use This Skill

- **Publishing skills to public Tessl registry**: Ensuring all requirements are met before `tessl skill publish --public`
- **Pre-publication validation**: Verifying a skill has proper eval scenarios and quality scores
- **Quality gate enforcement**: Blocking publication of skills below A-grade threshold (≥108/120)
- **Eval scenario creation**: Generating comprehensive evaluation scenarios for skill effectiveness measurement
- **Tile configuration audit**: Checking tile.json for public publication settings (private: false)
- **Registry submission preparation**: Final checklist before submitting to Tessl public registry

Examples:

- user: "Publish this skill to Tessl" → Check eval scenarios, quality score, tile.json settings
- user: "Is this skill ready for public publishing?" → Run readiness validation workflow
- user: "Create eval scenarios for this skill" → Generate comprehensive test cases with success criteria
- user: "Prepare skill for Tessl registry" → Execute full publication readiness workflow

## When NOT to Use This Skill

- **Private skill publishing**: Use standard `tessl skill publish` without public flag
- **Internal skill development**: Publishing to private workspace only
- **Quick prototyping**: Skills not intended for public consumption
- **Pre-alpha testing**: Skills still in early development phase

## Mindset

Public skill publication to the Tessl registry requires discipline and quality enforcement. A public skill represents the community standard and must demonstrate measurable effectiveness through evaluation scenarios.

**Think quality gate, not publishing shortcut.** Every public skill must:

1. **Prove effectiveness**: Eval scenarios demonstrate real-world value
2. **Meet quality bar**: A-grade minimum (≥108/120) from skill-quality-auditor
3. **Be agent-agnostic**: No harness-specific features or tools
4. **Show clear ROI**: Measurable improvements in agent performance

The Tessl registry is curated for quality, not quantity. Respect the threshold.

## Scope

| What's Included | What's NOT Included |
|----------------|---------------------|
| Evaluation scenario creation and validation | Writing the skill content itself |
| Quality threshold enforcement (A-grade ≥108/120) | General skill authoring guidance |
| Tile.json configuration for public publishing | Private workspace skill publishing |
| Publication readiness validation workflow | Skill content optimization |
| Agent-agnostic compliance checking | Tessl installation/authentication |
| Cross-platform compatibility validation | Skill-quality-auditor execution (separate tool) |
| Registry submission preparation | Post-publication monitoring |

## Workflow

### 1. Pre-Publication Quality Audit

Run skill-quality-auditor to ensure A-grade minimum:

```bash
# Run quality audit and store results
sh skills/testing/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store

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
sh skills/agentic-harness/tessl/tessl-publish-public/scripts/check-publication-readiness.sh skills/<domain>/<skill-name>

# Publish to public registry
tessl skill publish skills/<domain>/<skill-name> --public

# Verify publication
tessl search <skill-name>
```

## Quick Commands

```bash
# Full publication readiness check
sh skills/agentic-harness/tessl/tessl-publish-public/scripts/check-publication-readiness.sh skills/<domain>/<skill-name>

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

## Configuration Examples

### Minimal tile.json for Public Publishing

```json
{
  "name": "workspace/skill-name",
  "version": "1.0.0",
  "private": false,
  "summary": "Brief description of skill value",
  "keywords": ["tessl", "skill", "domain"],
  "license": "MIT"
}
```

### Comprehensive tile.json

```json
{
  "name": "workspace/skill-name",
  "version": "1.2.0",
  "private": false,
  "summary": "Clear value proposition for skill effectiveness",
  "description": "Detailed explanation of what the skill does and how it helps agents perform tasks more effectively",
  "keywords": ["tessl", "ci-cd", "automation", "quality"],
  "author": "Your Name <email@example.com>",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/org/repo"
  }
}
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

### NEVER: Skip Evaluation Scenarios

Public skills require eval scenarios to prove effectiveness. Minimum 5 scenarios with specific success criteria.

```bash
# BAD: Direct publish without evals
tessl skill publish skills/domain/skill --public

# GOOD: Verify evals exist first
ls skills/domain/skill/evaluation-scenarios/ | wc -l  # Must be ≥5
```

### NEVER: Publish Below A-Grade (108/120)

Quality threshold exists for a reason. B+ and below skills have significant gaps.

```bash
# BAD: Publish at 85/120 (C+ grade)
tessl skill publish skills/domain/skill --public

# GOOD: Audit, remediate, re-audit until ≥108/120
sh skills/testing/skill-quality-auditor/scripts/evaluate.sh domain/skill --json --store
cat .context/audits/domain/skill/latest/remediation-plan.md
```

### NEVER: Forget private: false

tile.json defaults to private:true. Must explicitly set to false.

```bash
# BAD: Publish without checking
tessl skill publish skills/domain/skill --public

# GOOD: Verify and set
jq '.private = false' skills/domain/skill/tile.json > tmp.json && mv tmp.json skills/domain/skill/tile.json
```

### NEVER: Skip --optimize When <90%

Tessl optimization can boost scores 85%→99%. Always use it when below 90%.

```bash
# BAD: Publish at 85% without optimization
tessl skill publish skills/domain/skill --public

# GOOD: Optimize first
tessl skill review skills/domain/skill --optimize
tessl skill review skills/domain/skill  # Verify improvement
```

### NEVER: Use Agent-Specific Tools

Public skills must work across all harnesses (Claude Code, Cursor, Gemini CLI, OpenCode). Use universal tools only (Bash, Edit, Read, Write, Grep, Glob).

```markdown
<!-- BAD: Claude Code specific -->
Use Claude Code's terminal integration to run tests

<!-- GOOD: Universal -->
Use Bash tool to run tests
```

## Quick Reference

| Task | Command |
|------|---------|
| Check publication readiness | `sh scripts/check-publication-readiness.sh skills/<domain>/<skill>` |
| Run quality audit | `sh skills/testing/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill> --json --store` |
| View audit results | `cat .context/audits/<domain>/<skill>/latest/analysis.md` |
| Create eval scenario | `touch skills/<domain>/<skill>/evaluation-scenarios/scenario-01.md` |
| Verify private flag | `jq '.private' skills/<domain>/<skill>/tile.json` |
| Set private to false | `jq '.private = false' skills/<domain>/<skill>/tile.json > tmp.json && mv tmp.json skills/<domain>/<skill>/tile.json` |
| Run Tessl review | `tessl skill review skills/<domain>/<skill>` |
| Optimize Tessl score | `tessl skill review skills/<domain>/<skill> --optimize` |
| Publish to public registry | `tessl skill publish skills/<domain>/<skill> --public` |
| Verify publication | `tessl search <skill-name>` |

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
