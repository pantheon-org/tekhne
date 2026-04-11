# Scenario 03: Create AGENTS.md for Python FastAPI Project

## User Prompt

Your team maintains a Python-based microservice that provides REST API endpoints for a billing system. The service uses FastAPI, PostgreSQL, and Redis. Several AI assistants you've tried have been generating incorrect code because they assume Node.js/TypeScript stack based on common patterns they've seen.

The project lead wants you to create AGENTS.md documentation that will help AI assistants understand this is a Python project and provide accurate, working guidance. The documentation should be based on the actual technology stack, not assumptions.

## Output Specification

Create an `AGENTS.md` file in the repository root that provides:

- Correct Python/Poetry/Pip environment setup instructions
- How to run the FastAPI application
- How to run tests (pytest)
- Database migration commands
- Coding conventions

The key requirement: the documentation must accurately reflect the Python ecosystem, not assume Node.js or other frameworks.

## Expected Behavior

1. Inspect config files (e.g., `pyproject.toml`, `requirements.txt`, `Makefile`) to detect the Python stack before writing anything
2. Write commands that match the Python toolchain: Poetry/pip for dependencies, pytest for tests, uvicorn/fastapi for running the service
3. Avoid importing or referencing Node.js patterns (no `npm`, `jest`, `tsconfig`, etc.)
4. Produce a root AGENTS.md that is concise — no full framework manuals embedded
5. Provide project-specific paths and commands, not generic Python boilerplate
6. Avoid duplicating content that can be referenced via links to existing docs

## Success Criteria

- **No technology assumptions**: Documentation does NOT assume technologies that weren't present in the input (e.g., no React mention for Python project)
- **Discovery-based content**: Content is based on discovered technologies from provided config files
- **Concise root file**: Root AGENTS.md is NOT encyclopedic (no full framework manual embedded)
- **No duplication**: Instructions are NOT duplicated across multiple files unnecessarily
- **Verified commands**: Commands included are valid for the detected stack (e.g., correct package manager, testing framework)
- **References over duplication**: Uses references/links to external docs rather than embedding full documentation
- **Path-specific content**: Commands are specific to this project, not generic placeholders

## Failure Conditions

- Documentation references npm, jest, tsc, or other Node.js tools not present in this project
- Content is generated from assumptions about a typical web project rather than inspection of provided config files
- Root AGENTS.md embeds full Python tutorial content or generic FastAPI documentation
- Identical instructions appear in multiple places without referencing a single source
- Commands would fail if copy-pasted (e.g., wrong package manager, incorrect test runner flags)
- Uses generic placeholder paths like `<your-project>` instead of project-specific values
