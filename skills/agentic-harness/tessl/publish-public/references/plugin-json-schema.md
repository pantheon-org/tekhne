# .tessl-plugin/plugin.json Schema Reference

This document defines all required and optional fields for Tessl `.tessl-plugin/plugin.json` files, specifically for public registry publishing.

## Required Fields

All public plugins MUST include these fields:

### name (string, required)

The globally unique package identifier for the plugin.

**Format**: `workspace/plugin-name`

- **workspace**: Your Tessl workspace name (e.g., `pantheon-ai`, `my-org`)
- **plugin-name**: Kebab-case skill name (e.g., `markdown-authoring`, `terraform-validator`)

**Examples**:

```json
"name": "pantheon-ai/markdown-authoring"
"name": "my-workspace/docker-best-practices"
```

**Validation**:

- Must contain exactly one forward slash (/)
- Workspace and plugin-name must use lowercase letters, numbers, and hyphens only
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

- Breaking change -> increment MAJOR (2.0.0 -> 3.0.0)
- New feature -> increment MINOR (2.0.0 -> 2.1.0)
- Bug fix/docs -> increment PATCH (2.0.0 -> 2.0.1)

### description (string, required)

A comprehensive one-line description of the plugin's purpose, capabilities, and usage. Replaces the legacy `summary` field from tile.json.

**Characteristics**:

- Single sentence (can be long, 150-300 characters typical)
- Describes WHAT the plugin does and WHEN to use it
- Includes primary keywords INLINE (not in separate keywords array)
- Written for human readers, not just search engines
- Must not end with a period (Tessl convention)

**Structure pattern**:

```text
[Action verb] [what it does] [key capabilities]. Use when [trigger phrases]. Keywords: [3-8 relevant terms].
```

**Examples**:

```json
"description": "Author high-quality Markdown documentation with deterministic structure, lint compliance, and CI integration. Use when writing README files, creating docs pages, fixing markdownlint failures, defining style rules, or wiring markdown checks into pre-commit and pipelines. Keywords: markdown, markdownlint, readme, docs, headings, lists, code fences, links, images, lint config, ci, documentation style"
```

```json
"description": "Validate, lint, test, and automate Terraform configurations with comprehensive HCL analysis, security scanning, and CI/CD integration. Use when working with .tf files, validating infrastructure-as-code, debugging configurations, performing dry-run testing with terraform plan, or implementing custom providers and modules. Keywords: terraform, hcl, validation, infrastructure-as-code, iac, terraform-validate, tflint, security"
```

**Validation**:

- Length: 100-500 characters (recommended 150-300)
- Must not be empty or generic ("A useful plugin")
- Should include specific domain terminology
- Must not contain markdown formatting or line breaks
- Should include concrete use cases ("Use when...")
- **IMPORTANT**: Embed keywords inline with "Keywords: term1, term2, term3"

### private (boolean, required for public publishing)

Controls plugin visibility in the Tessl registry.

**Values**:

- `false`: Plugin is publicly discoverable and installable by anyone (required for public publishing)
- `true`: Plugin is only accessible within your workspace (private/development)
- If omitted: Defaults to `true` (private)

**Examples**:

```json
"private": false  // For public publishing - REQUIRED
"private": true   // For private/workspace-only plugins
```

**Publishing rules**:

- MUST be set to `false` before running `tessl plugin publish --public`
- Private plugins cannot be published to public registry
- Once published publicly, plugin remains discoverable (cannot be unpublished, only deprecated)
- For republishing: keep as `false`, bump version number instead

### skills (array of strings, required)

Defines the skill files contained within this plugin. Each entry is a path string relative to the plugin root.

**Structure**:

```json
{
  "skills": ["SKILL.md"]
}
```

**Rules**:

- Must be an array of strings (NOT an object with keys - this is the key difference from legacy tile.json)
- At least one skill path must be defined
- Paths are relative to plugin root directory
- Each path must point to a valid SKILL.md file
- SKILL.md must have proper YAML frontmatter

**Single skill example**:

```json
{
  "skills": ["SKILL.md"]
}
```

**Multi-skill example**:

```json
{
  "skills": ["generator/SKILL.md", "validator/SKILL.md"]
}
```

**Validation**:

- Skills array must not be empty
- All paths must exist and be valid markdown files
- Each SKILL.md must contain YAML frontmatter with `name` and `description`

## Optional Root-Level Fields

These fields enhance discoverability and organization but are not required:

### files (array of strings, optional)

Additional files to bundle with the plugin (templates, assets, configuration examples).

**Example**:

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
- All paths relative to plugin root
- All files will be bundled when plugin is installed

## Complete Examples

### Minimal Public Plugin

```json
{
  "name": "my-workspace/awesome-skill",
  "version": "1.0.0",
  "private": false,
  "description": "Create amazing things with powerful automation and best practices. Use when building features, fixing bugs, or optimizing workflows. Keywords: automation, best-practices, productivity",
  "skills": ["SKILL.md"]
}
```

### Plugin with Bundled Files

```json
{
  "name": "pantheon-ai/gitlab-ci-complete",
  "version": "2.0.0",
  "private": false,
  "description": "Generate, validate, and optimize GitLab CI/CD pipelines with best practices, security scanning, and comprehensive testing. Use when creating pipelines, validating YAML syntax, implementing CI/CD workflows, or debugging pipeline failures. Keywords: gitlab-ci, ci-cd, pipelines, yaml, gitlab, automation, devops",
  "files": [
    "references/best-practices.md",
    "references/security-guidelines.md",
    "templates/basic-pipeline.yml",
    "templates/docker-build.yml"
  ],
  "skills": ["generator/SKILL.md", "validator/SKILL.md"]
}
```

## Validation Checklist

Before publishing publicly, verify:

- [ ] `name` field exists and follows `workspace/plugin-name` format (lowercase, kebab-case)
- [ ] `version` field exists and follows semantic versioning (x.y.z)
- [ ] `private` field is explicitly set to boolean `false` (not string `"false"`)
- [ ] `description` field exists and is descriptive (150-300 chars recommended)
- [ ] `description` includes "Use when..." trigger phrases
- [ ] `description` includes relevant keywords inline ("Keywords: term1, term2, term3")
- [ ] `skills` is an array with at least one skill path string
- [ ] All skill paths point to existing SKILL.md files
- [ ] All SKILL.md files have valid YAML frontmatter (name, description)
- [ ] All `files` paths exist (if specified)
- [ ] JSON syntax is valid (no trailing commas, proper quotes)
- [ ] Version is bumped if republishing existing public plugin

## Common Errors

### Missing required fields

```json
{
  "name": "my-workspace/plugin",
  "version": "1.0.0"
  // ERROR: Missing "private", "description", and "skills"
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

### Generic or empty description (no keywords)

```json
{
  "description": "A useful plugin"  // ERROR: Too generic, no keywords, no "Use when"
}
```

**FIX**:

```json
{
  "description": "Validate JSON schemas with comprehensive error reporting and automatic fixing. Use when validating API responses, config files, or data structures. Keywords: json, schema, validation, json-schema"
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

### Empty skills array

```json
{
  "skills": []  // ERROR: Must contain at least one skill path
}
```

### Skills as object (legacy tile.json format)

```json
{
  "skills": {
    "my-skill": { "path": "SKILL.md" }
  }
  // ERROR: skills must be an array of strings in plugin.json
}
```

### Non-existent skill path

```json
{
  "skills": ["does-not-exist.md"]  // ERROR: File must exist
}
```

## References

- Tessl CLI: `tessl review run` for automated validation
- Repository: AGENTS.md for publishing workflow
- Skill: skill-quality-auditor for quality validation
- Semantic Versioning: <https://semver.org/>
