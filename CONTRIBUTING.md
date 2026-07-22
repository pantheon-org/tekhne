# Contributing to Tekhne

## Quick start

```bash
# Install dependencies
bun install

# Run linting
bunx @biomejs/biome check .

# Run markdown lint
bunx markdownlint-cli2 "**/*.md"
```

Pre-commit hooks (via [`hk`](https://hk.jdx.dev), configured in `hk.pkl`) run
Biome, markdownlint, YAML validation, skill artifact checks, and skill
structure validation automatically on staged files.

## Adding or editing skills

1. Read `AGENTS.md` for domain organisation and authoring rules.
2. Read the existing `SKILL.md` in the skill directory before editing it.
3. Run the quality audit before publishing:

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store
```

Target B-grade (112/140) minimum; A-grade (126/140) for publication.

## Tooling

Skill management, auditing, and validation are provided by Rust crates under
`crates/` (`skill-auditor`, `skill-validator-rs`, `adr`, `journal`) plus the
`scripts/catalog` catalog generator. Everything builds from the Cargo workspace
or runs with Bun, so there is no separate install step.

```bash
# Audit a single skill (builds the auditor from source, then evaluates)
bun run audit:skill <domain>/<skill-name>

# Validate a skill's structure
cargo run -p skill-validator-rs -- validate structure skills/<domain>/<skill-name>

# Regenerate the skill catalog in README.md and the docs tiles page
bun run readme:update
```

### Running tests

```bash
# TypeScript catalog tests
bun test scripts/

# Rust workspace tests
cargo test --workspace

# Integration (cucumber) tests
bun run test:integration
```

## Commit style

Follow [Conventional Commits](https://www.conventionalcommits.org/):
`feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`.
