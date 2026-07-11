---
name: sop-structure
description: Use when structuring Standard Operating Procedures with proper sections, organization, and markdown formatting. Covers SOP anatomy and section organization.
allowed-tools:
  - Read
  - Write
  - Edit
  - Bash
  - Grep
  - Glob
---

# SOP Structure

Well-structured SOPs follow consistent patterns that make them easy to understand, maintain, and execute. This skill covers the anatomy of effective SOPs and how to organize sections.

## Key Concepts

### Standard SOP Anatomy

Every SOP should include these core sections:

1. **Title**: Clear, action-oriented description
2. **Overview**: Brief summary of purpose and use cases
3. **Parameters**: Configurable inputs for reusability
4. **Prerequisites**: Required tools, knowledge, or setup
5. **Steps**: Sequential instructions for execution
6. **Success Criteria**: How to verify completion
7. **Error Handling**: What to do when things go wrong
8. **Related SOPs**: Links to related workflows

### File Naming Convention

SOP files MUST use the `.sop.md` extension:

```
✅ deployment-checklist.sop.md
✅ code-review-security.sop.md
✅ database-migration.sop.md

❌ deployment.md (missing .sop)
❌ checklist.sop.txt (wrong file type)
❌ SOP-Deployment.md (incorrect format)
```

## Mindset

An SOP is executed by someone who was not in the room when it was written. Optimise for unambiguous, repeatable execution: every step one concrete action, every prerequisite stated up front. Know **when not to** write an SOP: a one-off task nobody will repeat does not need one.

## When to Use This Skill

Use when writing or restructuring a Standard Operating Procedure, or when a procedure is being followed inconsistently and needs a repeatable structure.

## Best Practices

### Title Section

```markdown
# {Action Verb} {Specific Outcome}

Short form: Use kebab-case filename
Long form: Use Title Case heading
```

**Examples:**

```markdown
# Generate API Documentation

# Implement Feature Using TDD

# Review Pull Request for Security
```

### Overview Section

The overview should answer three questions:

1. **What**: What does this SOP accomplish?
2. **When**: When should you use this SOP?
3. **Why**: Why use this approach?

```markdown
## Overview

This SOP guides you through implementing new features using Test-Driven
Development (TDD). Use this when adding functionality that requires high
confidence in correctness. TDD ensures comprehensive test coverage and
reduces regression risk.
```

### Parameters Section

Define all configurable inputs at the beginning:

```markdown
## Parameters

- **Input Variable**: {variable_name} - Description and example values
- **Configuration**: {config_option} - Available options (option1, option2, option3)
- **Path**: {file_path} - Expected format and constraints
```

**Example:**

```markdown
## Parameters

- **Repository Path**: {repository_path} - Absolute path to git repository
- **Output Format**: {output_format} - Documentation format (markdown, html, pdf)
- **Verbosity**: {verbosity} - Detail level (concise, standard, comprehensive)
- **Include Tests**: {include_tests} - Whether to include test examples (yes, no)
```

### Prerequisites Section

List required tools, knowledge, and setup:

```markdown
## Prerequisites

### Required Tools
- Tool name (version X.X or higher)
- Another tool (version Y.Y or higher)

### Required Knowledge
- Understanding of concept A
- Familiarity with technology B

### Required Setup
- Environment variable {VAR_NAME} must be set
- Configuration file {config.json} must exist
```

**Example:**

```markdown
## Prerequisites

### Required Tools
- Node.js (v18 or higher)
- npm (v8 or higher)
- Git (v2.30 or higher)

### Required Knowledge
- Understanding of JavaScript/TypeScript
- Familiarity with testing frameworks
- Git workflow basics

### Required Setup
- Package.json exists in project root
- Test framework is installed (Jest, Vitest, or Mocha)
- Git repository is initialized
```

### Steps Section

Structure steps hierarchically:

```markdown
## Steps

1. First major step
   - Sub-step or detail
   - Another sub-step
   - Additional context

2. Second major step
   - Implementation detail
   - Expected outcome

3. Third major step
   - Specific action
   - Validation step
```

**With Validation:**

```markdown
## Steps

1. Analyze codebase structure
   - Identify main entry points
   - Map directory organization
   - List dependencies
   - **Validation**: Confirm all entry points are documented

2. Extract patterns
   - Identify design patterns
   - Document data flow
   - Note architectural decisions
   - **Validation**: Verify patterns are correctly identified

3. Generate documentation
   - Create overview section
   - Document public APIs
   - Add usage examples
   - **Validation**: Ensure documentation builds without errors
```

### Success Criteria Section

Define measurable outcomes:

```markdown
## Success Criteria

- [ ] Specific measurable outcome 1
- [ ] Specific measurable outcome 2
- [ ] Specific measurable outcome 3
- [ ] All tests pass
- [ ] Documentation is complete
```

**Example:**

```markdown
## Success Criteria

- [ ] All new code has test coverage ≥ 90%
- [ ] All tests pass without warnings
- [ ] Code passes linter with zero errors
- [ ] Documentation includes usage examples
- [ ] Changes follow existing code patterns
```

### Error Handling Section

Provide guidance for common failures:

```markdown
## Error Handling

### Error: {Error Name or Code}

**Symptoms**: How this error manifests

**Cause**: Why this error occurs

**Resolution**:
1. First troubleshooting step
2. Second troubleshooting step
3. Alternative approach if steps fail
```

**Example:**

```markdown
## Error Handling

### Error: Tests Fail to Run

**Symptoms**: Test runner exits with error code, tests don't execute

**Cause**: Missing dependencies, incorrect test framework configuration, or environment issues

**Resolution**:
1. Verify test framework is installed: `npm list {test-framework}`
2. Check test configuration file exists and is valid
3. Ensure NODE_ENV is set correctly
4. If issue persists, reinstall dependencies: `rm -rf node_modules && npm install`

### Error: Type Errors During Build

**Symptoms**: TypeScript compiler reports type mismatches

**Cause**: Incorrect type annotations or missing type definitions

**Resolution**:
1. Run type checker: `npx -y --package typescript tsc`
2. Review error messages for specific type issues
3. Add necessary type annotations
4. Install missing @types packages if needed
```

### Related SOPs Section

Link to related workflows:

```markdown
## Related SOPs

- **{sop-name}**: Brief description of when to use this instead
- **{another-sop}**: How this complements the current SOP
```

**Example:**

```markdown
## Related SOPs

- **code-review**: Use after completing feature implementation to get peer review
- **deployment-checklist**: Use after code review passes to deploy changes
- **rollback-procedure**: Use if deployment fails or issues are discovered
```

## Anti-Patterns

**Avoid These Structure Mistakes:**

1. **Missing Parameters**
   - ❌ Hard-coding values in steps
   - ✅ Define parameters at the beginning

2. **Unclear Prerequisites**
   - ❌ Assuming tools are installed
   - ✅ Explicitly list required tools and versions

3. **Vague Success Criteria**
   - ❌ "Code should be good quality"
   - ✅ "Code passes linter with 0 errors and has test coverage ≥ 80%"

4. **No Error Handling**
   - ❌ Only describing happy path
   - ✅ Including common failures and resolutions

5. **Poor Section Organization**
   - ❌ Steps before parameters, success criteria mixed with steps
   - ✅ Consistent section order: Overview → Parameters → Prerequisites → Steps → Success Criteria → Error Handling

## References

- [SOP Templates and Examples](references/sop-templates.md) - complete worked SOP example plus reusable code-analysis and implementation templates.

## Related Skills

- **sop-authoring**: Learn to write clear, actionable instructions
- **sop-rfc2119**: Use RFC 2119 keywords for precise requirements
- **sop-maintenance**: Keep SOPs current and relevant
