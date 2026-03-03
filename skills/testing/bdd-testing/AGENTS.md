# bdd-testing

> **Note:** `CLAUDE.md` is a symlink to this file.

## Overview

Master Behavior-Driven Development (BDD) testing with comprehensive guidance on principles, patterns, Gherkin syntax, collaboration practices, and Cucumber.js implementation.

## Structure

```
bdd-testing/
  SKILL.md       # Main navigation hub - read this first
  AGENTS.md      # This file - complete reference guide
  CLAUDE.md      # Symlink to AGENTS.md
  references/    # Detailed reference files by category
```

## Usage

1. Read `SKILL.md` to understand when and how to apply BDD practices
2. Navigate to specific categories based on your current task
3. Load reference files on-demand - only read what you need for the task
4. Start with principles, then move to patterns, then implementation

## Reference Categories

| Priority | Category | Impact | Prefix |
|----------|----------|--------|--------|
| 1 | Principles & Philosophy | CRITICAL | `principles-` |
| 2 | Gherkin Syntax & Structure | CRITICAL | `gherkin-` |
| 3 | Collaboration Practices | HIGH | `collaboration-` |
| 4 | Scenario Writing | HIGH | `scenarios-` |
| 5 | Patterns & Anti-patterns | MEDIUM | `patterns-` |
| 6 | Cucumber.js Implementation | MEDIUM | `cucumber-` |

Reference files are named `{prefix}-{topic}.md`.

## Available References

**Principles & Philosophy** (`principles-`):
- `references/principles-core-philosophy.md` - What is BDD, discovery-development-delivery cycle
- `references/principles-three-amigos.md` - Three perspectives collaboration practice
- `references/principles-living-documentation.md` - Executable specifications as documentation
- `references/principles-ubiquitous-language.md` - Shared vocabulary between business and tech
- `references/principles-example-mapping.md` - Workshop technique with colored cards
- `references/principles-specification-by-example.md` - Concrete examples drive development
- `references/principles-outside-in.md` - Outside-in development workflow
- `references/principles-bdd-vs-tdd.md` - Differences and how they complement each other

**Gherkin Syntax & Structure** (`gherkin-`):
- `references/gherkin-syntax.md` - Feature, Scenario, Given/When/Then keywords
- `references/gherkin-given-when-then.md` - Arrange-Act-Assert pattern explained
- `references/gherkin-scenario-outlines.md` - Data-driven tests with Examples tables
- `references/gherkin-background.md` - Common setup for all scenarios in a feature
- `references/gherkin-tags.md` - Organizing and filtering scenarios with tags
- `references/gherkin-data-tables.md` - Passing structured data to steps

**Collaboration Practices** (`collaboration-`):
- `references/collaboration-three-amigos-sessions.md` - Running effective discovery sessions
- `references/collaboration-example-mapping-workshop.md` - Yellow/blue/green/red card technique
- `references/collaboration-discovery-workshop.md` - Structured 50-minute workshop format
- `references/collaboration-building-ubiquitous-language.md` - Creating shared domain vocabulary
- `references/collaboration-stakeholder-involvement.md` - Engaging business stakeholders

**Scenario Writing** (`scenarios-`):
- `references/scenarios-writing-good-scenarios.md` - Specific, declarative, independent scenarios
- `references/scenarios-acceptance-criteria.md` - Rule-based acceptance criteria format
- `references/scenarios-edge-cases.md` - Systematically covering edge cases
- `references/scenarios-organization.md` - Feature file structure and organization
- `references/scenarios-declarative-vs-imperative.md` - Business-focused vs implementation-focused

**Patterns & Anti-patterns** (`patterns-`):
- `references/patterns-one-scenario-one-behavior.md` - Single responsibility for scenarios
- `references/patterns-independent-scenarios.md` - Avoiding scenario dependencies
- `references/patterns-background-usage.md` - When and how to use Background
- `references/patterns-reusable-steps.md` - Creating maintainable step definitions
- `references/patterns-domain-language.md` - Using business terms not technical jargon
- `references/patterns-common-antipatterns.md` - What to avoid in BDD tests

**Cucumber.js Implementation** (`cucumber-`):
- `references/cucumber-setup.md` - Installation and project structure
- `references/cucumber-step-definitions.md` - Writing step definitions in TypeScript
- `references/cucumber-cucumber-expressions.md` - Parameter types and custom patterns
- `references/cucumber-world.md` - Sharing state between steps
- `references/cucumber-hooks.md` - Before/After hooks and setup/teardown
- `references/cucumber-configuration.md` - cucumber.js config, parallel execution, profiles
- `references/cucumber-running-tests.md` - CLI commands, tags, dry-run, snippets

---

*Consolidates 6 original skills: bdd-collaboration, bdd-gherkin, bdd-patterns, bdd-principles, bdd-scenarios, cucumber-best-practices*

*42 reference files across 6 categories*
