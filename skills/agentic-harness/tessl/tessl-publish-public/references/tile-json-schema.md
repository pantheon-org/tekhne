# Tile.json Schema Reference

This document defines all required and optional fields for Tessl tile.json files, specifically for public registry publishing. Based on analysis of 66+ actual production tiles.

## Required Fields

All public tiles MUST include these fields:

### name (string, required)

The globally unique package identifier for the tile.

**Format**: `workspace/tile-name`

- **workspace**: Your Tessl workspace name (e.g., `pantheon-ai`, `my-org`)
- **tile-name**: Kebab-case skill name (e.g., `markdown-authoring`, `terraform-validator`)

**Examples**:

```json
"name": "pantheon-ai/markdown-authoring"
"name": "my-workspace/docker-best-practices"
```

**Validation**:

- Must contain exactly one forward slash (/)
- Workspace and tile-name must use lowercase letters, numbers, and hyphens only
- No spaces, underscores, or special characters
- Must be globally unique across Tessl registry

### version (string, required)

Semantic version number following SemVer 2.0.0 specification.

**Format**: `MAJOR.MINOR.PATCH`

- **MAJOR**: Incompatible changes (breaking changes)
- **MINOR**: Backward-compatible new features
- **PATCH**: Backward-compatible bug fixes

**Examples**:

```json
"version": "1.0.0"
"version": "2.3.1"
"version": "0.1.0"
```

**Validation**:

- Must match regex: `^\d+\.\d+\.\d+$`
- Start at `0.1.0` for initial development
- Use `1.0.0` for first stable public release
- When republishing, version must be higher than previous published version

**Version bumping rules**:

- Breaking change → increment MAJOR (2.0.0 → 3.0.0)
- New feature → increment MINOR (2.0.0 → 2.1.0)
- Bug fix/docs → increment PATCH (2.0.0 → 2.0.1)

### summary (string, required)

A comprehensive one-line description of the skill's purpose, capabilities, and usage.

**Characteristics**:

- Single sentence (can be long, 150-300 characters typical)
- Describes WHAT the skill does and WHEN to use it
- Includes primary keywords INLINE (not in separate keywords array)
- Written for human readers, not just search engines
- Must not end with a period (Tessl convention)

**Structure pattern**:

```text
[Action verb] [what it does] [key capabilities]. Use when [trigger phrases]. Keywords: [3-8 relevant terms].
```

**Examples** (from actual production tiles):

```json
"summary": "Author high-quality Markdown documentation with deterministic structure, lint compliance, and CI integration. Use when writing README files, creating docs pages, fixing markdownlint failures, defining style rules, or wiring markdown checks into pre-commit and pipelines. Keywords: markdown, markdownlint, readme, docs, headings, lists, code fences, links, images, lint config, ci, documentation style"
```

```json
"summary": "Validate, lint, test, and automate Terraform configurations with comprehensive HCL analysis, security scanning, and CI/CD integration. Use when working with .tf files, validating infrastructure-as-code, debugging configurations, performing dry-run testing with terraform plan, or implementing custom providers and modules. Keywords: terraform, hcl, validation, infrastructure-as-code, iac, terraform-validate, tflint, security"
```

**Validation**:

- Length: 100-500 characters (recommended 150-300)
- Must not be empty or generic ("A useful skill")
- Should include specific domain terminology
- Must not contain markdown formatting or line breaks
- Should include concrete use cases ("Use when...")
- **IMPORTANT**: Embed keywords inline with "Keywords: term1, term2, term3" rather than separate keywords array

### private (boolean, required for public publishing)

Controls tile visibility in the Tessl registry.

**Values**:

- `false`: Tile is publicly discoverable and installable by anyone (required for public publishing)
- `true`: Tile is only accessible within your workspace (private/development)
- If omitted: Defaults to `true` (private)

**Examples**:

```json
"private": false  // For public publishing - REQUIRED
"private": true   // For private/workspace-only tiles
```

**Publishing rules**:

- MUST be set to `false` before running `tessl skill publish --public`
- Private tiles cannot be published to public registry
- Once published publicly, tile remains discoverable (cannot be unpublished, only deprecated)
- For republishing: keep as `false`, bump version number instead

### skills (object, required)

Defines the skills contained within this tile, their file paths, and optional bundled resources.

**Structure**:

```json
{
  "skills": {
    "skill-identifier": {
      "path": "relative/path/to/SKILL.md",
      "references": ["ref1.md", "ref2.md"],
      "resources": ["resource1.md", "resource2.md"]
    }
  }
}
```

**Required per skill**:

- `path` (string): Relative path from tile root to SKILL.md file

**Optional per skill**:

- `references` (array): Markdown files referenced by this skill (bundled with tile)
- `resources` (array): Markdown resource files for this skill (alternative name for references)

**Rules**:

- At least one skill must be defined
- Skill identifier must be kebab-case
- Path is relative to tile root directory
- Path must point to a valid SKILL.md file
- SKILL.md must have proper YAML frontmatter

**Single skill example**:

```json
{
  "skills": {
    "markdown-authoring": {
      "path": "SKILL.md"
    }
  }
}
```

**Skill with references example** (from k8s-debug):

```json
{
  "skills": {
    "k8s-debug": {
      "path": "SKILL.md",
      "references": [
        "references/troubleshooting_workflow.md",
        "references/common_issues.md",
        "references/kubectl_commands.md"
      ]
    }
  }
}
```

**Skill with resources example** (from typescript-advanced):

```json
{
  "skills": {
    "typescript-advanced": {
      "path": "SKILL.md",
      "resources": [
        "references/compiler-tsconfig.md",
        "references/types-generics.md",
        "references/utility-types.md"
      ]
    }
  }
}
```

**Multiple skills example**:

```json
{
  "skills": {
    "terraform-generator": {
      "path": "generator/SKILL.md",
      "references": ["generator/references/templates.md"]
    },
    "terraform-validator": {
      "path": "validator/SKILL.md",
      "references": ["validator/references/rules.md"]
    }
  }
}
```

**Validation**:

- Skills object must not be empty
- All paths must exist and be valid markdown files
- Each SKILL.md must contain YAML frontmatter with `name` and `description`
- Skill identifiers should match directory names (convention)
- All `references` and `resources` paths must exist

## Optional Root-Level Fields

These fields enhance discoverability and organization but are not required:

### docs (string, optional)

Path to the primary documentation file for this tile.

**Example**:

```json
"docs": "references/overview.md"
```

**Usage**:

- Points to detailed tile documentation
- Separate from skill documentation (SKILL.md)
- Typically explains tile structure, installation, configuration
- Path relative to tile root

**Used by**: 2 tiles (minimal usage, most tiles use SKILL.md for all docs)

### files (array of strings, optional)

Additional files to bundle with the tile (templates, assets, configuration examples).

**Example** (from gitlab-ci-generator):

```json
{
  "files": [
    "references/best-practices.md",
    "references/security-guidelines.md",
    "templates/basic-pipeline.yml",
    "templates/docker-build.yml"
  ]
}
```

**Usage**:

- Include reference documentation not directly tied to a specific skill
- Include template files (YAML, JSON, config examples)
- Include asset files (diagrams, examples)
- All paths relative to tile root
- All files will be bundled when tile is installed

**Used by**: 7 tiles (gitlab-ci-generator, ansible-generator, helm-generator, etc.)

**Note**: The difference between `files` (root-level) and `skills.<skill>.references` (skill-level):

- `files`: Shared across all skills in the tile, general tile resources
- `references`/`resources`: Specific to one skill, skill-specific documentation

## Complete Examples

### Minimal Public Tile

```json
{
  "name": "my-workspace/awesome-skill",
  "version": "1.0.0",
  "private": false,
  "summary": "Create amazing things with powerful automation and best practices. Use when building features, fixing bugs, or optimizing workflows. Keywords: automation, best-practices, productivity",
  "skills": {
    "awesome-skill": {
      "path": "SKILL.md"
    }
  }
}
```

### Comprehensive Single-Skill Tile with References

```json
{
  "name": "pantheon-ai/k8s-debug",
  "version": "1.2.0",
  "private": false,
  "summary": "Debug Kubernetes clusters with systematic troubleshooting workflows, pod inspection, and log analysis. Use when pods crash, services are unreachable, or resources are misconfigured. Keywords: kubernetes, k8s, debug, troubleshoot, kubectl, pods, logs",
  "skills": {
    "k8s-debug": {
      "path": "SKILL.md",
      "references": [
        "references/troubleshooting_workflow.md",
        "references/common_issues.md",
        "references/kubectl_commands.md"
      ]
    }
  }
}
```

### Multi-Skill Tile with Root-Level Files

```json
{
  "name": "pantheon-ai/gitlab-ci-complete",
  "version": "2.0.0",
  "private": false,
  "summary": "Generate, validate, and optimize GitLab CI/CD pipelines with best practices, security scanning, and comprehensive testing. Use when creating pipelines, validating YAML syntax, implementing CI/CD workflows, or debugging pipeline failures. Keywords: gitlab-ci, ci-cd, pipelines, yaml, gitlab, automation, devops",
  "docs": "references/overview.md",
  "files": [
    "references/best-practices.md",
    "references/security-guidelines.md",
    "templates/basic-pipeline.yml",
    "templates/docker-build.yml"
  ],
  "skills": {
    "gitlab-ci-generator": {
      "path": "generator/SKILL.md",
      "references": ["generator/references/pipeline-patterns.md"]
    },
    "gitlab-ci-validator": {
      "path": "validator/SKILL.md",
      "references": ["validator/references/validation-rules.md"]
    }
  }
}
```

## Validation Checklist

Before publishing publicly, verify:

- [ ] `name` field exists and follows `workspace/tile-name` format (lowercase, kebab-case)
- [ ] `version` field exists and follows semantic versioning (x.y.z)
- [ ] `private` field is explicitly set to boolean `false` (not string `"false"`)
- [ ] `summary` field exists and is descriptive (150-300 chars recommended)
- [ ] `summary` includes "Use when..." trigger phrases
- [ ] `summary` includes relevant keywords inline ("Keywords: term1, term2, term3")
- [ ] `skills` object exists with at least one skill defined
- [ ] All skill paths point to existing SKILL.md files
- [ ] All SKILL.md files have valid YAML frontmatter (name, description)
- [ ] All `references` and `resources` paths exist (if specified)
- [ ] All `files` paths exist (if specified)
- [ ] JSON syntax is valid (no trailing commas, proper quotes)
- [ ] Version is bumped if republishing existing public tile

## Common Errors

### Missing required fields

```json
{
  "name": "my-workspace/skill",
  "version": "1.0.0"
  // ERROR: Missing "private", "summary", and "skills"
}
```

### Invalid private value (string instead of boolean)

```json
{
  "private": "false"  // ERROR: Must be boolean false, not string "false"
}
```

```json
{
  "private": true  // ERROR: Must be false for public publishing
}
```

### Generic or empty summary (no keywords)

```json
{
  "summary": "A useful skill"  // ERROR: Too generic, no keywords, no "Use when"
}
```

**FIX**:

```json
{
  "summary": "Validate JSON schemas with comprehensive error reporting and automatic fixing. Use when validating API responses, config files, or data structures. Keywords: json, schema, validation, json-schema"
}
```

### Invalid name format

```json
{
  "name": "MyWorkspace/SkillName"  // ERROR: Must be lowercase kebab-case
}
```

```json
{
  "name": "workspace_skill_name"  // ERROR: Use forward slash, not underscores
}
```

### Invalid version format

```json
{
  "version": "1.0"  // ERROR: Must be x.y.z format
}
```

```json
{
  "version": "v1.0.0"  // ERROR: No 'v' prefix
}
```

### Empty skills object

```json
{
  "skills": {}  // ERROR: Must contain at least one skill
}
```

### Non-existent skill path

```json
{
  "skills": {
    "my-skill": {
      "path": "does-not-exist.md"  // ERROR: File must exist
    }
  }
}
```

### Using deprecated keywords array (anti-pattern)

```json
{
  "summary": "Validate Terraform configurations",
  "keywords": ["terraform", "validation"]  // ANTI-PATTERN: Embed in summary instead
}
```

**FIX**: Embed keywords inline in summary

```json
{
  "summary": "Validate Terraform configurations with HCL syntax checking and security scanning. Use when working with .tf files or debugging infrastructure code. Keywords: terraform, validation, hcl, iac, infrastructure-as-code"
}
```

**Reasoning**: Only 1 out of 66 production tiles uses separate `keywords` array. Standard pattern is to embed keywords inline in the summary for better discoverability and readability.

## Property Usage Statistics

Based on analysis of 66 production tiles in this repository:

| Property | Usage | Type | Level |
|----------|-------|------|-------|
| `name` | 66/66 (100%) | Required | Root |
| `version` | 66/66 (100%) | Required | Root |
| `private` | 66/66 (100%) | Required | Root |
| `summary` | 66/66 (100%) | Required | Root |
| `skills` | 66/66 (100%) | Required | Root |
| `files` | 7/66 (11%) | Optional | Root |
| `docs` | 2/66 (3%) | Optional | Root |
| `keywords` | 1/66 (2%) | Deprecated | Root |
| `references` | ~30/66 (45%) | Optional | Skill-level |
| `resources` | 1/66 (2%) | Optional | Skill-level |

**Key insight**: The `keywords` property is effectively deprecated. Embed keywords inline in the summary instead.

## References

- Tessl CLI: `tessl skill lint` for automated validation
- Tessl CLI: `tessl skill import` to generate initial tile.json
- Repository: AGENTS.md for publishing workflow
- Skill: skill-quality-auditor for quality validation
- Semantic Versioning: <https://semver.org/>
- Agent Skills Specification: <https://agentskills.io>
