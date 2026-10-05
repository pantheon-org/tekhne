# Contributing to Tekhne

## Quick start

```bash
# Install dependencies
bun install

# Run linting
bunx @biomejs/biome check .

# Run markdown lint
mise run lint   # rumdl check . (config: .rumdl.toml)
```

Pre-commit hooks (via [`hk`](https://hk.jdx.dev), configured in `hk.pkl`) run
Biome, rumdl, YAML validation, skill artifact checks, and skill
structure validation automatically on staged files.

## Adding or editing skills

1. Read `AGENTS.md` for domain organisation and authoring rules.
2. Read the existing `SKILL.md` in the skill directory before editing it.
3. Run the quality audit before publishing:

```bash
sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh <domain>/<skill-name> --json --store
```

Target B-grade (112/140) minimum; A-grade (126/140) for publication.

The pull request Skill Audit grades every skill that has any changed file, not only `SKILL.md`. A skill graded below B blocks the pull request only if it is new or its score is lower than on `main`, so a change that does not lower it can still merge. When a skill is already below B and not lowered, a tracking issue is opened for it automatically and linked to your pull request. If your change lowers a skill's score, raise it back before merging.

## Tooling

Skill management, auditing, and validation are provided by Rust crates under
`crates/` (`skill-auditor`, `skill-validator-rs`, `adr`, `journal`) plus the
`scripts/catalog` catalog generator. Everything builds from the Cargo workspace
or runs with Bun, so there is no separate install step.

```bash
# Audit a single skill (builds the auditor from source, then evaluates)
mise run audit:skill <domain>/<skill-name>

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
