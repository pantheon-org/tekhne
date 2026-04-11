# Scenario 01: Create AGENTS.md for Simple Node.js Project

## User Prompt

Your team has been working on a new internal API service that handles user authentication and profile management. The service is a straightforward Node.js application with Express, TypeScript, and PostgreSQL. Several team members have been struggling to understand how to run tests, build the project, and follow coding conventions when making contributions.

The engineering lead has asked you to create an AGENTS.md file in the repository root to help AI assistants and new developers understand how to work with this codebase. This documentation should be practical and enable someone to start contributing quickly.

## Output Specification

Create an `AGENTS.md` file in the repository root that provides:

- How to set up the development environment
- How to run tests
- How to build the project
- Coding conventions and patterns used
- Any useful commands for development

The documentation should be concise but complete enough for someone to start contributing without asking questions.

## Expected Behavior

1. Run discovery commands (e.g., `rg` or equivalent) to inspect the repository structure and config files before writing anything
2. Detect the actual technology stack (package manager, testing framework, TypeScript config) from existing project files rather than assuming
3. Choose a single-file AGENTS.md structure appropriate for a simple project — no unnecessary hierarchy
4. Write concise, actionable content: setup, test, build, lint commands with project-specific paths
5. Include useful discovery commands so future agents can orient themselves
6. Validate that every command matches the detected stack before finalising

## Success Criteria

- **Discovery performed**: Agent ran discovery commands (rg or similar) to understand the repository structure before creating documentation
- **Technology detection**: Agent detected actual technology stack (package manager, testing framework, etc.) from config files rather than assuming
- **Single file structure**: Created single AGENTS.md at root (not unnecessary hierarchical structure for simple project)
- **Concise content**: Content is concise and not encyclopedic (no full framework manuals embedded)
- **Path-specific commands**: Commands include specific paths relevant to this project, not generic placeholders
- **Verified commands**: Commands appear to be valid for the detected project (e.g., correct package manager commands)
- **No assumptions**: Documentation reflects discovered technologies, not assumed ones
- **Actionable instructions**: Instructions are concrete and actionable, not vague
- **Discovery commands included**: Includes useful discovery commands for future agents (e.g., file listing, config detection)
- **Quality check**: Documentation appears validated (no obvious broken links or invalid commands)

## Failure Conditions

- Writes AGENTS.md without first running any discovery commands
- Assumes technologies not present in the project (e.g., references npm when the project uses a different package manager)
- Creates a multi-level AGENTS.md hierarchy for a simple single-package project
- Produces encyclopedic or bloated content duplicating full framework documentation
- Uses generic placeholder commands not tied to this project's actual scripts or paths
- Includes commands that would fail when copy-pasted into the project
