---
plan_date: 2026-02-23
skill_name: opencode-config
source_audit: .context/audits/opencode-config-audit-2026-02-22.md
---

# Remediation Plan: opencode-config

## Executive Summary

| Metric | Value |
| --- | --- |
| **Current Score** | 88/120 (73%) |
| **Current Grade** | C |
| **Target Score** | 102/120 (85%) |
| **Target Grade** | B |
| **Priority** | High |
| **Effort Estimate** | Medium (S) |

## Critical Issues to Address

| # | Issue | Severity | Dimension |
|---|-------|----------|-----------|
| 1 | Pattern recognition low (6/10) | High | D7 |
| 2 | Mindset + procedures unclear (10/15) | Medium | D2 |
| 3 | Specification compliance moderate (10/15) | Medium | D4 |
| 4 | Knowledge delta moderate (15/20) | Medium | D1 |
| 5 | Anti-pattern quality moderate (10/15) | Medium | D3 |
| 6 | Progressive disclosure moderate (10/15) | Low | D5 |

## Detailed Remediation Steps

### Phase 1: Pattern Recognition (D7: 6/10 -> 10/10)

**File**: `skills/opencode-config/SKILL.md`

1. Expand frontmatter description with trigger keywords:

```yaml
---
description: |
  Edit opencode.json, AGENTS.md, and config files for OpenCode agent configuration.
  Use proactively for provider setup, permission changes, model config, 
  formatter rules, or environment variables.
  
  Triggers: "edit opencode.json", "configure opencode", "add provider", 
  "set model", "opencode permissions", "agent config", "OpenCode setup",
  "provider API key", "model preference", "formatter config"
---
```

1. Add comprehensive "Use When" section:

```markdown
## Use When

- Adding or configuring AI model providers (Anthropic, OpenAI, etc.)
- Setting up agent permissions and tool access
- Configuring default model preferences
- Managing formatter rules (prettier, biome, gofmt)
- Setting environment variables for API keys
- Creating or modifying AGENTS.md documentation
- Troubleshooting agent behavior issues

## NOT For

- General coding tasks (use appropriate language skill)
- Creating new agents (use agent-architect skill)
- Plugin development (use opencode-sdk-development skill)
```

### Phase 2: Mindset + Procedures (D2: 10/15 -> 13/15)

**File**: `skills/opencode-config/SKILL.md`

Add deterministic workflow with explicit steps:

```markdown
## Workflow

### Step 1: Identify Configuration Target

Preconditions: User request received
Actions:
- Determine which config file to modify
- Check current state with `opencode run test`

Exit: Target config file identified

### Step 2: Read Current Configuration

Preconditions: Target identified
Actions:
- Read opencode.json
- Read AGENTS.md (if applicable)
- Read any referenced config files

Exit: Current state understood

### Step 3: Apply Changes

Preconditions: Changes planned
Actions:
- Edit configuration file(s)
- Validate JSON syntax if opencode.json
- Verify markdown format if AGENTS.md

Exit: Files modified correctly

### Step 4: Verify Configuration

Preconditions: Changes applied
Actions:
- Run `opencode run test` to verify
- Check for configuration errors
- Confirm expected behavior

Exit: Configuration validated
```

### Phase 3: Specification Compliance (D4: 10/15 -> 13/15)

**File**: `skills/opencode-config/SKILL.md`

1. Ensure frontmatter follows OpenCode skill spec
2. Add explicit scope definition:

```markdown
## Scope

This skill covers configuration of:

| File | Purpose |
|------|---------|
| opencode.json | Main agent configuration |
| AGENTS.md | Project-specific agent instructions |
| .env files | Environment variables (via dotenvx) |
| providers/ | Custom provider configurations |

Out of scope:
- Plugin development
- Custom tool creation
- Agent personality design
```

### Phase 4: Knowledge Delta (D1: 15/20 -> 17/20)

**File**: `skills/opencode-config/SKILL.md`

Add non-obvious guidance:

```markdown
## Gotchas

### Provider Priority

When multiple providers support the same model, OpenCode uses the first 
matching provider in the `providers` array. Order matters.

### Permission Inheritance

Agent-level permissions override global permissions. Empty `allow` arrays 
mean "allow nothing" (deny by default), not "allow everything".

### Environment Variables

Base environment variables in opencode.json are overridden by:
1. Agent-specific env vars
2. .env files loaded by dotenvx
3. Shell environment variables

The precedence is: shell > dotenvx > agent env > base env

### Model Name Format

Model names must match the provider's expected format exactly. For example:
- Anthropic: `claude-3-5-sonnet-20241022`
- OpenAI: `gpt-4-turbo`
- OpenRouter: `openai/gpt-4-turbo` (includes provider prefix)

## Production Caveats

- API keys in opencode.json are visible in git - use baseEnv pattern
- Agent permissions should be least-privilege by default
- Always test configuration changes with `opencode run test`
```

### Phase 5: Anti-Pattern Quality (D3: 10/15 -> 13/15)

**File**: `skills/opencode-config/SKILL.md`

Add explicit anti-patterns:

````markdown
## Anti-Patterns

### NEVER: Commit API keys directly in opencode.json

**WHY**: Keys are visible in git history and can be leaked.

**BAD**:
```json
{
  "providers": [{
    "name": "anthropic",
    "apiKey": "sk-ant-api03-..."
  }]
}
```

**GOOD**:
```json
{
  "providers": [{
    "name": "anthropic",
    "baseEnv": "ANTHROPIC_API_KEY"
  }]
}
```

### NEVER: Grant broad file access without restriction

**WHY**: Violates least-privilege principle, increases risk.

**BAD**:
```json
{
  "permissions": {
    "fileAccess": [{ "path": "/", "allow": ["read", "write"] }]
  }
}
```

**GOOD**:
```json
{
  "permissions": {
    "fileAccess": [{ "path": "src/", "allow": ["read", "write"] }]
  }
}
```

### NEVER: Use ambiguous model names

**WHY**: Different providers may resolve differently.

**BAD**:
```json
{ "model": "gpt-4" }
```

**GOOD**:
```json
{ "model": "openai/gpt-4-turbo-2024-04-09" }
```

### NEVER: Skip permission testing after config changes

**WHY**: Incorrect permissions cause silent failures or security issues.

**BAD**:
- Edit permissions, commit, assume it works

**GOOD**:
```bash
opencode run test --agent <agent-name>
```
````

### Phase 6: Progressive Disclosure (D5: 10/15 -> 12/15)

**Files to create/modify**:

1. **Create** `skills/opencode-config/references/provider-configuration.md`:
   - Move detailed provider setup for each provider
   - Add provider-specific options
   - Document model availability

2. **Create** `skills/opencode-config/references/permission-schema.md`:
   - Move permission schema documentation
   - Add examples for common permission patterns
   - Document tool-specific permissions

**File to modify**: `skills/opencode-config/SKILL.md`

Add reference links and reduce redundant content.

## Verification Commands

```bash
sh skills/skill-quality-auditor/scripts/evaluate.sh opencode-config --json
bunx @biomejs/biome check skills/opencode-config/
bunx markdownlint-cli2 "skills/opencode-config/**/*.md"
ls -la skills/opencode-config/references/  # Should have 3+ files (1 existing)
```

## Success Criteria

| Criterion | Measurement |
| --- | --- |
| D7 Pattern Recognition | Score >= 9/10 |
| D2 Mindset + Procedures | Score >= 12/15 |
| D4 Specification Compliance | Score >= 12/15 |
| D3 Anti-Pattern Quality | Score >= 12/15 |
| References created | >= 2 new files |
| Overall Score | >= 102/120 (B+) |

## Effort Estimate

| Phase | Effort | Time |
| --- | --- | --- |
| Phase 1: Pattern Recognition | S | 30 min |
| Phase 2: Procedures | S | 30 min |
| Phase 3: Spec Compliance | S | 30 min |
| Phase 4: Knowledge Delta | S | 30 min |
| Phase 5: Anti-patterns | S | 30 min |
| Phase 6: Progressive Disclosure | S | 20 min |
| **Total** | **S** | **3 hours** |

## Dependencies

- None (standalone skill)

## Rollback Plan

```bash
git checkout HEAD~1 -- skills/opencode-config/SKILL.md
```

## Notes

- Rating: **8/10** - Already follows Format B template very well with detailed phases and code examples
- Strong structure with 6 phases covering all critical dimensions
- Has Estimated Effort table, Dependencies, Rollback Plan
- Code examples in remediation steps are specific and actionable
