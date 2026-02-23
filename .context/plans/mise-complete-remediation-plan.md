---
plan_date: 2026-02-23
skill_name: mise-complete
source_audit: .context/audits/mise-complete-audit-2026-02-22.md
---

# Remediation Plan: mise-complete

## Executive Summary

| Metric | Current | Target |
| --- | --- | --- |
| **Score** | 82/120 (68%) | 100/120 (83%) |
| **Grade** | D | B |
| **Priority** | Critical | - |

**Verdict**: Major rewrite recommended. Low usability and weak anti-patterns.

## Critical Issues to Address

| Issue | Dimension | Severity | Impact |
| --- | --- | --- | --- |
| Weak anti-pattern precision | D3 (8/15) | High | Common mise mistakes repeated |
| Low practical usability | D8 (8/15) | **Critical** | Commands/examples missing |
| Poor trigger discoverability | D7 (6/10) | High | Skill not activated |
| Moderate spec compliance | D4 (10/15) | Medium | Routing confusion |
| Missing deterministic workflow | D2 (10/15) | Medium | Execution ambiguity |
| Over/under constraint | D6 (10/15) | Medium | Flexibility issues |

## Detailed Remediation Steps

### Phase 1: Practical Usability (D8) - CRITICAL PRIORITY

**File**: `skills/mise-complete/SKILL.md`

**Problem**: D8 score of 8/15 indicates missing commands and examples.

1. **Add Quick Commands section**:

````markdown
## Quick Commands

### Tool Version Management
```bash
# Install a specific tool version
mise use node@20

# Install multiple versions
mise use node@20 python@3.11

# Set global default
mise use -g node@20

# List installed tools
mise ls

# List available versions
mise ls-remote node
```

### Task Management
```bash
# List available tasks
mise tasks

# Run a task
mise run build

# Run task with arguments
mise run test --filter=unit

# Add a new task
mise task add build -- npm run build
```

### Environment Variables
```bash
# Set environment variable
mise set NODE_ENV=development

# Set in .mise.toml
mise set --file .mise.toml API_KEY=secret

# View current environment
mise env
```

### Configuration
```bash
# View current config
mise config

# Validate config
mise config --validate

# Show config file locations
mise config --paths
```

### Profiles
```bash
# List profiles
mise profiles

# Activate profile
mise profile activate production

# Set profile-specific tools
mise use --profile production node@18
```
````

2. **Add expected output examples**:

````markdown
## Expected Outputs

### mise ls Output
```
Plugin  Version  Config Source
node    20.11.0  ~/.config/mise/config.toml
python  3.11.7   ~/.config/mise/config.toml
go      1.22.0   .mise.toml
```

### mise tasks Output
```
Task    Description   Source
build   Build project .mise.toml
test    Run tests     .mise.toml
lint    Lint code     .mise.toml
```

### .mise.toml Structure
```toml
[tools]
node = "20"
python = "3.11"

[env]
NODE_ENV = "development"

[tasks.build]
run = "npm run build"
description = "Build the project"

[tasks.test]
run = "npm test"
description = "Run tests"
```
````

### Phase 2: Anti-Pattern Quality (D3) - HIGH PRIORITY

**File**: `skills/mise-complete/SKILL.md`

1. **Add explicit anti-patterns section**:

```markdown
## Anti-Patterns

### NEVER commit .mise.local.toml with secrets
- **WHY**: Local config may contain sensitive environment variables
- **BAD**: Committing `.mise.local.toml` with API keys
- **GOOD**: Use `.env` files or secret managers, add to .gitignore
- **CONSEQUENCE**: Secret exposure in version control

### NEVER use global config for project-specific tools
- **WHY**: Breaks project portability and reproducibility
- **BAD**: `mise use -g node@20` for project that needs node@18
- **GOOD**: `mise use node@18` (project-level)
- **CONSEQUENCE**: Team members have different versions, build failures

### NEVER hardcode absolute paths in tasks
- **WHY**: Paths differ across environments
- **BAD**: `run = "/Users/me/project/build.sh"`
- **GOOD**: `run = "./build.sh"` or `run = "bash $MISE_PROJECT_ROOT/scripts/build.sh"`
- **CONSEQUENCE**: Tasks fail on other machines

### NEVER skip version pinning
- **WHY**: Floating versions cause non-deterministic builds
- **BAD**: `node = "latest"` or `node = "20"` (minor version floats)
- **GOOD**: `node = "20.11.0"` (pinned)
- **CONSEQUENCE**: Different versions across builds, subtle bugs

### NEVER mix asdf and mise in same project
- **WHY**: Tool version conflicts, environment pollution
- **BAD**: Both `.tool-versions` and `.mise.toml` present
- **GOOD**: Migrate completely to mise, remove asdf config
- **CONSEQUENCE**: Tool version conflicts, unpredictable behavior

### NEVER ignore mise trust prompt
- **WHY**: Security mechanism for untrusted config files
- **BAD**: Automatically trusting all configs
- **GOOD**: Review config before trusting
- **CONSEQUENCE**: Potential execution of malicious tasks

### Repository-Specific Rules
- This repository uses `.mise.toml` (not `mise.toml`)
- All tools pinned to exact versions
- Environment variables via `env._.file` pattern
```

### Phase 3: Pattern Recognition (D7) - HIGH PRIORITY

**File**: `skills/mise-complete/SKILL.md`

1. **Expand frontmatter description**:

```markdown
---
name: mise-complete
description: Complete mise (formerly rtx) tool management guidance covering version management, task automation, environment variables, and configuration. Use when: "install tool with mise", "set up mise tasks", "manage environment variables", "configure mise.toml", "mise use", "mise install", "mise tasks", "mise env", "mise set", "tool versions".
---
```

2. **Add explicit trigger phrases**:

```markdown
## Activation Triggers

This skill activates when users ask:

### Tool Installation
- "Install [tool] with mise"
- "Set up [tool] version"
- "Use mise to manage [tool]"
- "What tools are available in mise?"

### Task Automation
- "Create a mise task"
- "Run [task] with mise"
- "Set up build task"
- "Add npm script to mise"

### Environment Management
- "Set environment variable in mise"
- "Configure mise environment"
- "Load .env with mise"
- "Profile-specific environment"

### Configuration
- "Set up mise.toml"
- "Configure mise for project"
- "Mise best practices"
- "Migrate from asdf to mise"

### Troubleshooting
- "Why is mise not finding [tool]?"
- "Fix mise version conflict"
- "Mise task not running"
```

### Phase 4: Specification Compliance (D4) - MEDIUM PRIORITY

**File**: `skills/mise-complete/SKILL.md`

1. **Add output specification**:

````markdown
## Output Specification

### .mise.toml Structure
```toml
[tools]
tool-name = "version"     # Required: at least one tool

[env]                     # Optional: environment variables
VAR_NAME = "value"

[tasks.task-name]         # Optional: task definitions
run = "command"
description = "Task description"
```

### Config File Locations (Priority Order)
1. `.mise.local.toml` (git-ignored, local overrides)
2. `.mise.toml` (project-level, committed)
3. `~/.config/mise/config.toml` (global)

### Required Frontmatter Fields
- `name`: Skill identifier
- `description`: Trigger phrases and scope
````

### Phase 5: Workflow Determinism (D2) - MEDIUM PRIORITY

**File**: `skills/mise-complete/SKILL.md`

1. **Add explicit workflow**:

````markdown
## Workflow

### Step 1: Determine Scope
- **Input**: User request
- **Decision**: Global | Project | Local config
- **Output**: Config scope confirmed

### Step 2: Choose Operation
| Operation | Command |
|-----------|---------|
| Add tool | `mise use [tool]@[version]` |
| Remove tool | `mise uninstall [tool]` |
| Add task | Edit `.mise.toml` [tasks] section |
| Set env var | `mise set [VAR]=[value]` |

### Step 3: Apply Configuration
- **Action**: Execute mise command or edit config file
- **Output**: Updated configuration

### Step 4: Verify
```bash
mise install          # Install any missing tools
mise ls               # Verify tool versions
mise tasks            # Verify tasks available
```
- **Exit condition**: All checks pass

### Step 5: Document
- **Action**: Add comments in .mise.toml for complex tasks
- **Output**: Documented configuration
````

### Phase 6: Freedom Calibration (D6) - MEDIUM PRIORITY

**File**: `skills/mise-complete/SKILL.md`

1. **Add flexibility guidelines**:

```markdown
## Flexibility Guidelines

### Hard Constraints (NEVER violate)
- Tools must be pinned to exact versions
- Never commit secrets to .mise.toml
- Use kebab-case for task names
- Validate config before committing

### Flexible Decisions (adapt to context)
- Global vs project-level config (project-specific needs)
- Task complexity (simple run vs shell script)
- Environment variable approach (env section vs .env file)

### Fallback Paths
- If tool not in registry: Use asdf plugin or custom install
- If task complex: Extract to shell script in scripts/
- If version unavailable: Use closest matching version
```

## Verification Commands

```bash
# Re-run skill evaluation
sh skills/skill-quality-auditor/scripts/evaluate.sh mise-complete --json

# Verify mise works
mise --version
mise ls

# Check skill structure
ls -la skills/mise-complete/

# Run full audit
skills/skill-quality-auditor/scripts/audit-skills.sh --skills-dir skills
```

## Success Criteria

| Criterion | Measurement | Target |
| --- | --- | --- |
| D8: Practical Usability | Score increase | >= 13/15 |
| D3: Anti-Pattern Quality | Score increase | >= 12/15 |
| D7: Pattern Recognition | Score increase | >= 9/10 |
| D4: Specification Compliance | Score increase | >= 13/15 |
| D2: Mindset + Procedures | Score increase | >= 13/15 |
| D6: Freedom Calibration | Score increase | >= 12/15 |
| Overall Score | Total points | >= 100/120 |
| Grade | Letter grade | >= B |

## Effort Estimate

- **T-shirt size**: M (4-6 hours)
- **Complexity**: Medium
- **Risk**: Low (additive changes)

## Dependencies

- mise installed (check with `mise --version`)

## Notes

- D5 (Progressive Disclosure) is already strong at 15/15 - maintain current structure
- Consider moving detailed configuration examples to references/
- 16 reference files already exist - leverage them in SKILL.md navigation
