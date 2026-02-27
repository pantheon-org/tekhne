---
category: framework
priority: HIGH
source: tessl registry requirements + session experience
---

# Tessl Registry Compliance Framework

Supplementary validation checks for skills intended for [Tessl](https://tessl.io/) registry submission. These checks extend the core 8-dimension framework with agent-agnostic and performance-focused evaluations.

**Use after:** Core 8-dimension evaluation (≥108 points required)
**Target:** 100% tessl compliance for registry acceptance

## Overview

Tessl focuses on performance-evaluated, agent-agnostic skills that provide measurable effectiveness improvements. This framework adds 3 supplementary validation areas:

1. **Agent-Agnostic Validation** - Ensure cross-platform compatibility
2. **Performance Metrics Integration** - Quantify effectiveness improvements  
3. **Cross-Platform Compatibility** - Validate tool/feature support

## Agent-Agnostic Validation (Pass/Fail)

**Purpose:** Ensure skills work across different AI assistant platforms without agent-specific dependencies.

### Validation Checks

#### ❌ NEVER: Agent-Specific Tool Dependencies

Check `allowed-tools` frontmatter for agent-specific tools:

```bash
# Bad - Claude Code specific
allowed-tools: [claude-artifact, claude-codebase]

# Good - Universal tools
allowed-tools: [bash, edit, read, write]
```

**Auto-check pattern:** Flag tools containing agent names (claude-, cursor-, openai-, etc.)

#### ❌ NEVER: Platform-Specific Instructions

Scan content for agent-specific references:

```markdown
❌ BAD: "Tell Claude to run the command"
❌ BAD: "Use Cursor's autocomplete feature"  
❌ BAD: "In OpenAI's interface, click..."

✅ GOOD: "Run the command using your bash tool"
✅ GOOD: "Use your code completion capabilities"
✅ GOOD: "Execute the following workflow"
```

**Auto-check pattern:** `/\b(claude|cursor|openai|copilot|gemini|chatgpt)\b/i`

#### ❌ NEVER: Hardcoded Agent Behaviors

Avoid assuming specific agent capabilities:

```markdown
❌ BAD: "Since you can't execute code directly..."
❌ BAD: "Use your web browsing to..."
❌ BAD: "Your image generation will..."

✅ GOOD: "If code execution tools are available..."
✅ GOOD: "When web access is supported..."
✅ GOOD: "For agents with image capabilities..."
```

### Scoring

- **PASS:** No agent-specific dependencies detected
- **FAIL:** Any agent-specific references found

## Performance Metrics Integration (Pass/Fail)

**Purpose:** Ensure skills define measurable effectiveness improvements that can be evaluated.

### Required Components

#### 1. Success Metrics Definition

Skills must include quantifiable outcomes:

```markdown
## Success Metrics

This skill provides:
- ✅ 85% reduction in configuration errors
- ✅ 3x faster setup time (5 minutes vs 15 minutes)
- ✅ 100% compliance with security standards
```

#### 2. Before/After Scenarios

Show clear improvement examples:

```markdown
## Effectiveness Examples

### Before Using This Skill
- Manual setup takes 30+ commands
- 40% failure rate on first attempt
- Inconsistent configuration across environments

### After Using This Skill  
- One-command deployment
- <5% failure rate
- Standardized, reproducible environments
```

#### 3. Measurable Outcomes

Define what "effective use" looks like:

```markdown
## Expected Outcomes

When applied correctly, this skill delivers:
- Time savings: 60-90% reduction in task duration
- Quality improvement: 95%+ adherence to best practices
- Error reduction: <10% incident rate vs 30% baseline
```

### Validation Checks

- **Required sections:** "Success Metrics", "Expected Outcomes", or equivalent
- **Quantified claims:** Must include specific numbers (percentages, time, error rates)
- **Comparative data:** Before/after or baseline comparisons

### Scoring

- **PASS:** Contains measurable effectiveness claims with quantification
- **FAIL:** No performance metrics or only qualitative claims

## Cross-Platform Compatibility (Pass/Fail)

**Purpose:** Validate that skill instructions work across different development environments and agent platforms.

### Tool Compatibility Checks

#### Universal Tools Only

Verify all referenced tools are widely supported:

```markdown
✅ GOOD: bash, read, write, edit, glob, grep
✅ GOOD: Standard CLI tools (git, npm, docker)
✅ GOOD: Common development commands

❌ BAD: Agent-specific tools
❌ BAD: Proprietary extensions
❌ BAD: Platform-locked features
```

#### Command Portability

Ensure shell commands work across operating systems:

```bash
# Bad - macOS specific
brew install package

# Good - Cross-platform with options
# Install using your package manager:
# - macOS: brew install package  
# - Ubuntu: apt install package
# - Windows: choco install package
```

#### Path References

Use portable path conventions:

```bash
❌ BAD: /usr/local/bin/tool (Unix-specific)
❌ BAD: C:\Program Files\tool (Windows-specific)

✅ GOOD: Add tool to your PATH
✅ GOOD: $(which tool) or equivalent
```

### Agent Feature Assumptions

Avoid assuming specific agent capabilities:

```markdown
❌ BAD: "Use your built-in web scraping"
❌ BAD: "Generate an image with DALL-E"
❌ BAD: "Create a diagram with your drawing tools"

✅ GOOD: "If web scraping tools are available..."
✅ GOOD: "Using image generation capabilities..."
✅ GOOD: "With diagram creation tools..."
```

### Validation Process

1. **Tool Audit:** Check all tool references for universality
2. **Command Review:** Verify cross-platform shell commands  
3. **Feature Check:** Flag agent-specific capability assumptions
4. **Documentation Review:** Ensure instructions work for any agent

### Scoring

- **PASS:** No platform-specific dependencies detected
- **FAIL:** Contains non-portable tools or commands

## Implementation Guide

### For Skill Authors

When preparing skills for Tessl submission:

1. **Run core evaluation first:** Achieve A-grade (≥108 points)
2. **Apply tessl validations:** Use checks in this framework
3. **Fix compatibility issues:** Remove agent-specific dependencies
4. **Add performance metrics:** Quantify effectiveness improvements
5. **Verify portability:** Test commands across platforms

### For Auditors

Integration with existing skill-quality-auditor workflow:

```bash
# Standard evaluation first
sh skills/skill-quality-auditor/scripts/evaluate.sh <skill-name> --json

# Then apply tessl compliance checks
sh skills/skill-quality-auditor/scripts/tessl-compliance-check.sh <skill-name>
```

### Automated Validation

**Agent-Agnostic Check:**

```bash
# Check for agent-specific terms
grep -ri "claude\|cursor\|openai\|copilot\|gemini" skills/<skill>/
```

**Tool Compatibility Check:**

```bash
# Extract and validate allowed-tools
yq '.allowed-tools[]?' skills/<skill>/SKILL.md | grep -E "(claude|cursor|openai)-"
```

**Performance Metrics Check:**

```bash  
# Look for quantified outcomes
grep -E "[0-9]+(%|x|times|\s(seconds|minutes|hours)|reduction|improvement)" skills/<skill>/
```

## Integration with Core Framework

This framework supplements, not replaces, the 8-dimension evaluation:

| Check Type | When to Apply | Pass Criteria |
|------------|---------------|---------------|
| **Core 8-Dimension** | Always | ≥108 points (A-grade) |
| **Agent-Agnostic** | Tessl submission | No agent-specific deps |
| **Performance Metrics** | Tessl submission | Quantified effectiveness |
| **Cross-Platform** | Tessl submission | Universal compatibility |

## See Also

- `framework-skill-judge-dimensions.md` - Core 8-dimension framework
- `framework-quality-standards.md` - A-grade requirements
- [Tessl Registry](https://tessl.io/registry) - Performance-evaluated skills
