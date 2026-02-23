---
plan_date: 2026-02-23
skill_name: github-copilot-models
source_audit: .context/audits/github-copilot-models-audit-2026-02-22.md
---

# Remediation Plan: github-copilot-models

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 87/120 (72%) | 104/120 (87%) |
| **Grade** | C | B+ |
| **Priority** | High | - |

**Verdict**: Priority improvements required. Focus on anti-pattern quality and pattern recognition.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Weak anti-pattern precision | D3 (8/15) | High | Agents may repeat mistakes |
| Poor trigger discoverability | D7 (6/10) | High | Skill not activated when needed |
| Moderate spec compliance | D4 (10/15) | Medium | Routing confusion |
| Missing deterministic workflow | D2 (10/15) | Medium | Execution ambiguity |

## Detailed Remediation Steps

### Phase 1: Pattern Recognition (D7) - HIGH PRIORITY

**File**: `skills/github-copilot-models/SKILL.md`

**Problem**: D7 score of 6/10 indicates poor trigger discoverability.

1. **Expand frontmatter description**:

```markdown
---
name: github-copilot-models
description: Query and display available GitHub Copilot AI models with their capabilities, context limits, and features. Use when: "what models are available", "show copilot models", "list github models", "check model capabilities", "switch models", "which model has largest context", "models with vision support", "GPT-5 models available".
---
```

2. **Add explicit trigger phrases section**:

```markdown
## Activation Triggers

This skill activates when users ask:

### Direct Queries
- "What models are available with GitHub Copilot?"
- "Show me the available Copilot models"
- "List all GitHub models"
- "What models can I use?"

### Capability Queries
- "Which models support vision?"
- "What model has the largest context window?"
- "Show models with [feature] support"
- "Does [model] support [capability]?"

### Model Selection
- "Which model should I use for [task]?"
- "Compare available models"
- "Recommend a model for [use case]"

### Model Family Queries
- "List all GPT-5 models"
- "Show Claude models available"
- "What Gemini models are offered?"

### Troubleshooting
- "Why is my model not available?"
- "Check model status"
```

### Phase 2: Anti-Pattern Quality (D3) - HIGH PRIORITY

**File**: `skills/github-copilot-models/SKILL.md`

1. **Add explicit anti-patterns section**:

```markdown
## Anti-Patterns

### NEVER assume model availability
- **WHY**: Model availability varies by subscription tier and region
- **BAD**: "Use GPT-4-turbo for this task"
- **GOOD**: "Check model availability first, then recommend based on current options"
- **CONSEQUENCE**: User frustration when model not available in their environment

### NEVER hardcode model names
- **WHY**: Model catalog changes frequently
- **BAD**: `const models = ['gpt-4', 'gpt-3.5-turbo'];`
- **GOOD**: Fetch current model list via API
- **CONSEQUENCE**: Outdated recommendations, broken features

### NEVER ignore context limits
- **WHY**: Exceeding context limits causes silent failures or truncation
- **BAD**: Sending 200K tokens to a 128K context model
- **GOOD**: Check context limit before recommending model
- **CONSEQUENCE**: Lost context, incomplete responses

### NEVER recommend without knowing use case
- **WHY**: Different models excel at different tasks
- **BAD**: Always defaulting to "best" model
- **GOOD**: Ask about task type, then recommend
- **CONSEQUENCE**: Suboptimal performance, wasted resources

### NEVER skip model capability verification
- **WHY**: Capabilities differ between models and versions
- **BAD**: Assuming all models support vision
- **GOOD**: Verify capability before recommending
- **CONSEQUENCE**: Feature failures, user confusion
```

2. **Add BAD/GOOD examples for each anti-pattern**:

````markdown
### Example: Model Selection Anti-Pattern

**BAD**:
```
User: "I need to analyze an image"
Agent: "Use GPT-4 for that" [No verification of vision support]
```

**GOOD**:
```
User: "I need to analyze an image"
Agent: "Let me check which models support vision..."
[Query model capabilities]
"Based on current availability, these models support vision: [list]. 
I recommend [specific model] because [reasoning]."
```
````

### Phase 3: Specification Compliance (D4) - MEDIUM PRIORITY

**File**: `skills/github-copilot-models/SKILL.md`

1. **Tighten frontmatter**:

```markdown
---
name: github-copilot-models
description: Query and display available GitHub Copilot AI models with their capabilities, context limits, and features. Use when: "what models are available", "show copilot models", "list github models", "check model capabilities", "switch models", "which model has largest context", "models with vision support", "GPT-5 models available".
version: "1.0"
author: tekhne
keywords:
  - github
  - copilot
  - models
  - ai
  - llm
  - context-window
  - vision
  - gpt
  - claude
  - gemini
---
```

2. **Add explicit output specification**:

````markdown
## Output Specification

### Model Information Structure
Each model query returns:
- Model ID (string)
- Display name (string)
- Context window size (number in tokens)
- Capabilities (array: vision, function-calling, streaming, etc.)
- Model family (string)
- Availability status (available | preview | deprecated)

### Expected Response Format

```markdown
## Available Models

| Model | Context | Capabilities | Status |
|-------|---------|--------------|--------|
| gpt-4-turbo | 128K | vision, function-calling | available |
| claude-3-opus | 200K | vision, function-calling | available |
```
````

### Phase 4: Workflow Determinism (D2) - MEDIUM PRIORITY

**File**: `skills/github-copilot-models/SKILL.md`

1. **Add explicit workflow**:

```markdown
## Workflow

### Step 1: Parse User Intent
- **Input**: User query about models
- **Action**: Identify query type (list all | filter by capability | compare | recommend)
- **Output**: Query type classification

### Step 2: Fetch Model Data
- **Action**: Query GitHub Copilot API for current model list
- **Output**: Raw model data array

### Step 3: Process Request
- **If list all**: Format all models in table
- **If filter**: Apply capability/status filter
- **If compare**: Generate comparison table
- **If recommend**: Analyze use case, recommend best match

### Step 4: Format Output
- **Output**: Formatted markdown with model information
- **Include**: Model name, context limit, capabilities, status

### Step 5: Provide Next Steps
- **Action**: Suggest model selection command if user wants to switch
- **Output**: Actionable next step
```

2. **Add scope control**:

```markdown
## Scope Control

### Use This Skill When
- User asks about available models
- User needs model capability information
- User wants to compare models
- User needs model recommendation

### Do NOT Use This Skill When
- User wants to use a model (use appropriate completion skill)
- User asks about model pricing (out of scope)
- User wants model training details (out of scope)
```

## Verification Commands

```bash
# Re-run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh github-copilot-models --json

# Validate frontmatter
yq eval '.description' skills/github-copilot-models/SKILL.md

# Check keyword coverage
grep -E "(models available|copilot models|list models)" skills/github-copilot-models/SKILL.md

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
```

## Success Criteria

| Criterion | Measurement | Target |
| --- | --- | --- |
| D7: Pattern Recognition | Score increase | >= 9/10 |
| D3: Anti-Pattern Quality | Score increase | >= 12/15 |
| D4: Specification Compliance | Score increase | >= 13/15 |
| D2: Mindset + Procedures | Score increase | >= 13/15 |
| Overall Score | Total points | >= 104/120 |
| Grade | Letter grade | >= B+ |

## Effort Estimate

- **T-shirt size**: M (4-6 hours)
- **Complexity**: Low-Medium
- **Risk**: Low (additive changes only)

## Dependencies

- None (standalone skill)

## Notes

- Consider creating `references/model-families.md` if detailed model documentation grows
- Template for model comparison output could be added to `templates/`
