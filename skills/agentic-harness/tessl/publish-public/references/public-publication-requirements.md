# Public Publication Requirements for Tessl Plugins

This document defines the complete set of requirements a Tessl plugin must meet before publishing to the public registry.

## Critical Requirements (Must Pass)

### 1. Quality Threshold: A-Grade Minimum (>=108/120)

**Source**: skill-quality-auditor eight-dimension scoring system

**Rationale**: Between 2025-2026, 63 plugins were published using only `tessl review run`, resulting in average score of 98.3/120 (82%) with 37% in C+/C range. This required 40-60 hours of remediation work to lift to acceptable levels.

**Validation**: Run quality audit and check total score

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store
cat .context/audits/<domain>/<skill-name>/latest/analysis.md | grep "Total Score"
```

**Remediation**: If score < 108/120, review remediation plan

```bash
cat .context/audits/<domain>/<skill-name>/latest/remediation-plan.md
```

**Common weak dimensions** (repository-wide audit 2026-03):

- **D3 Anti-Pattern Quality (68% avg)**: Add "Common Mistakes" section with 3-5 anti-patterns
- **D5 Progressive Disclosure (73% avg)**: Structure info as Quick Start -> Detailed Guide -> Advanced
- **D2 Mindset & Procedures (74% avg)**: Establish mental models before procedures

### 2. Evaluation Scenarios: 5-8 Comprehensive Test Cases

**Rationale**: Public plugins represent effectiveness standards. Without eval scenarios, there's no proof the plugin delivers value.

**Structure**: Each scenario requires four sections:

1. **User Prompt**: Exact input the agent receives
2. **Expected Behavior**: Step-by-step actions agent should take
3. **Success Criteria**: Measurable outcomes (files created, commands run, outputs)
4. **Failure Conditions**: What indicates plugin was not used or failed

**Coverage**: Scenarios should cover:

- Basic use case (happy path)
- Intermediate complexity
- Edge cases (missing files, incorrect config)
- Error handling
- Integration with other skills/workflows
- Agent-agnostic execution

**Example**:

```markdown
# Scenario 01: Basic Publication Readiness Check

## User Prompt
"Is the nx-vite-integration skill ready to publish publicly?"

## Expected Behavior
1. Agent locates skill directory
2. Checks for evals/ directory
3. Verifies .tessl-plugin/plugin.json has private: false
4. Reviews quality audit results
5. Reports readiness status

## Success Criteria
- Agent identifies all missing requirements
- Provides specific remediation steps
- Does NOT publish if requirements unmet

## Failure Conditions
- Agent skips eval scenario check
- Agent ignores quality threshold
- Agent provides generic "looks good" response
```

### 3. .tessl-plugin/plugin.json Configuration

**Required fields**:

```json
{
  "name": "workspace/plugin-name",
  "version": "1.0.0",
  "private": false,
  "description": "Clear value proposition with use cases and keywords",
  "skills": ["SKILL.md"]
}
```

**Critical settings**:

- `private: false` - MUST be explicitly set for public publishing
- `version` - Semantic versioning; bump if republishing
- `description` - Clear, concise (shown in registry search); replaces legacy `summary`
- `skills` - Array of path strings (NOT the old tile.json object format)

**Common mistakes**:

- Omitting `private` field (defaults to true)
- Leaving `private: true` unchanged
- Generic description ("A plugin for X")
- Using old tile.json format with `skills` as object instead of array
- Using `summary` field (legacy tile.json) instead of `description`

### 4. Agent-Agnostic Compliance

**Prohibited**:

- Agent-specific tools (Claude Code, Cursor, Gemini-specific)
- Harness-specific instructions ("use Claude Code's X feature")
- Platform-specific behaviors (VS Code commands, IDE integrations)

**Required**:

- Universal tools only (Read, Write, Edit, Bash, Grep, Glob)
- Conditional capability checks for advanced features
- Cross-platform compatibility (CLI agents, IDE agents)

**Validation**: Scan SKILL.md for violations

```bash
grep -i "claude code\|cursor\|vs code\|opencode" skills/domain/skill-name/SKILL.md
```

### 5. Tessl Optimization: >=90% Quality Score

**Source**: `tessl review run` scoring system (independent of skill-quality-auditor)

**Rationale**: Tessl's optimization engine can dramatically improve scores (observed 85% -> 99% improvements).

**Workflow**:

```bash
# Initial review
tessl review run skills/domain/skill-name

# If < 90%, optimize
tessl review run skills/domain/skill-name --optimize

# Re-review to verify
tessl review run skills/domain/skill-name
```

**Key insight**: Always use `--optimize` flag for plugins scoring below 90%. Manual improvements may miss optimization opportunities.

## Recommended Requirements (Best Practices)

### 6. SKILL.md Structure and Sections

**Required sections**:

- YAML frontmatter (name, description)
- When to Use This Skill
- When NOT to Use This Skill
- Mindset
- Scope (table format)
- Workflow (numbered steps)
- Anti-Patterns (NEVER pattern with WHY/BAD/GOOD)
- Success Metrics
- Quick Reference (table format)

**Recommended sections**:

- Quick Commands (bash blocks)
- Configuration Examples
- Gotchas
- Production Caveats
- Related Skills

**Progressive disclosure**: Structure as Quick Start -> Detailed Guide -> Advanced Topics

### 7. Documentation Quality

**Clarity**:

- Short paragraphs (2-4 sentences)
- Active voice
- Clear headings hierarchy (H2 -> H3 -> H4)
- Concrete examples over abstract explanations

**Completeness**:

- Cover 80% use cases (not 100%)
- Address common mistakes explicitly
- Provide troubleshooting guidance
- Link to related skills/references

**Actionability**:

- Deterministic instructions (not "you might want to")
- Copy-paste code blocks with syntax highlighting
- Specific commands with expected outputs

### 8. Cross-Platform Testing

**Recommended validation**:

- Test skill instructions with multiple agents (Claude Code, Cursor, Gemini CLI)
- Verify commands work on macOS, Linux, Windows (if applicable)
- Check for shell-specific syntax (prefer POSIX sh over bash)
- Validate all referenced tools are commonly available

## Publication Workflow

### Phase 1: Pre-Publication Audit

1. Run skill-quality-auditor -> verify >=108/120
2. Review remediation plan if needed
3. Fix critical dimensions (D1, D2, D3, D5)
4. Re-audit until A-grade achieved

### Phase 2: Evaluation Scenario Creation

1. Create `evals/` directory
2. Generate 5-8 comprehensive scenario files
3. Ensure each has all four required sections
4. Cover breadth of skill capabilities

### Phase 3: Plugin Configuration

1. Ensure `.tessl-plugin/plugin.json` exists
2. Set `private: false` in plugin.json
3. Verify required fields (name, version, description, skills)
4. Bump version if republishing

### Phase 4: Tessl Optimization

1. Run `tessl review run skills/domain/skill-name`
2. If < 90%, run `tessl review run --optimize`
3. Re-review to verify improvements
4. Target >=90% score

### Phase 5: Agent-Agnostic Validation

1. Scan SKILL.md for agent-specific references
2. Verify only universal tools used
3. Check for platform-specific behaviors
4. Confirm cross-platform compatibility

### Phase 6: Publication

1. Run publication readiness script (optional but recommended)
2. Execute `tessl plugin publish --workspace pantheon-ai skills/domain/skill-name --bump patch`
3. Verify publication with `tessl search skill-name`

## Quality Gates Summary

| Gate | Requirement | Validation Command | Blocker |
|------|-------------|-------------------|---------|
| Quality Audit | >=108/120 (A-grade) | `sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill> --json --store` | YES |
| Eval Scenarios | 5-8 comprehensive scenarios | `ls skills/<domain>/<skill>/evals/` | YES |
| Plugin Config | `private: false` | `jq '.private' skills/<domain>/<skill>/.tessl-plugin/plugin.json` | YES |
| Agent-Agnostic | No harness-specific tools | Manual SKILL.md scan | YES |
| Tessl Score | >=90% | `tessl review run skills/<domain>/<skill>` | RECOMMENDED |
| SKILL.md Sections | Required sections present | Manual review | RECOMMENDED |
| Cross-Platform | Works on all agents | Multi-agent testing | RECOMMENDED |

## Common Publication Failures

### Failure 1: Skipped Evaluation Scenarios

**Symptom**: Publish command succeeds, but plugin flagged during registry review

**Cause**: `evals/` directory missing or has < 5 scenarios

**Fix**: Create comprehensive eval scenarios before publishing

### Failure 2: Private: True Blocks Publishing

**Symptom**: `tessl plugin publish --public` publishes to private workspace only

**Cause**: plugin.json missing `private: false` or has `private: true`

**Fix**: Explicitly set `private: false` in `.tessl-plugin/plugin.json`

### Failure 3: Quality Score Too Low

**Symptom**: Plugin published but performs poorly in agent evaluations

**Cause**: Skipped skill-quality-auditor or published below A-grade threshold

**Fix**: Run audit, address remediation plan, re-audit until >=108/120

### Failure 4: Agent-Specific Tools Break Cross-Platform

**Symptom**: Plugin works in one agent but fails in others

**Cause**: Used harness-specific tools or instructions

**Fix**: Rewrite using universal tools only, add capability checks

### Failure 5: Skipped Tessl Optimization

**Symptom**: Tessl review score stuck at 85-89%

**Cause**: Didn't use `--optimize` flag

**Fix**: Run `tessl review run --optimize` to unlock automatic improvements

### Failure 6: Legacy tile.json Format Used

**Symptom**: `tessl review run` fails or ignores manifest

**Cause**: Plugin still uses old `tile.json` format with `summary` and `skills` object

**Fix**: Migrate to `.tessl-plugin/plugin.json` with `description` field and `skills` array

## Post-Publication Considerations

### Version Management

- Published versions are immutable
- Use `--bump patch|minor|major` for updates
- Breaking changes require major version bump

### Quality Monitoring

- Registry may flag plugins with declining quality scores
- Plugins < 80% Tessl review may be delisted
- Monitor community feedback and issue reports

### Updates and Maintenance

- Address reported issues promptly
- Update evaluation scenarios as skill evolves
- Re-run audits after significant changes

### Community Standards

- Respond to public comments constructively
- Share improvements back to community
- Maintain high quality bar for all published plugins

## References

- **skill-quality-auditor**: Internal quality improvement tool
- **tessl review run**: Registry publication preparation tool
- **tessl plugin publish**: Registry publication command
- **Repository AGENTS.md**: Tessl publishing workflow and quality gates
