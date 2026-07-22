# adr CLI Usage

The `adr` binary creates and manages Architecture Decision Records from the
house template, and installs this companion skill into agent directories. It is
offline and deterministic: every command is a pure function of the ADR
directory's contents plus the current date.

## Directory resolution

Commands resolve the ADR directory in this order of precedence:

1. An explicit `--dir <path>` flag.
2. The `ADR_DIR` environment variable.
3. The default, `docs/adr` (relative to the current working directory).

Set `ADR_DIR` once per repository to avoid passing `--dir` on every call:

```bash
export ADR_DIR=architecture/decisions
```

## Commands

### `adr new <title>`

Creates the next-numbered ADR. The number is `max(existing NNNN-*.md) + 1`,
starting at `0001`. The title is slugged into the file name and rendered into
the heading. The record is stamped with today's date and status `Proposed`.

```bash
adr new "Adopt OpenTelemetry for tracing"
# Created docs/adr/0001-adopt-opentelemetry-for-tracing.md
```

### `adr list`

Prints every record, sorted by number, as `ADR-NNNN  <status>  <title>`. Files
that do not match the `NNNN-*.md` pattern (such as `README.md`) are ignored.

```bash
adr list
# ADR-0001  Accepted   Adopt OpenTelemetry for tracing
# ADR-0002  Proposed   Split the ingestion service
```

### `adr supersede <number> <new-title>`

Marks ADR `<number>` as `Superseded by ADR-NNNN` and creates a new Accepted
record titled `<new-title>` that references the old one. Errors if `<number>`
does not exist.

```bash
adr supersede 1 "Adopt Grafana Tempo for tracing"
# Superseded docs/adr/0001-adopt-opentelemetry-for-tracing.md
# Created docs/adr/0003-adopt-grafana-tempo-for-tracing.md
```

### `adr skill install`

Installs this `adr-creator` skill into agent skills directories. The skill is
embedded in the binary, so no repository checkout is needed.

| Flag | Effect |
| --- | --- |
| `--agent <name>` | Target a specific agent by slug (repeatable). |
| `--all` | Install into every agent in the universal list. |
| `--local` | Install into project-local directories instead of global. |
| `--mode copy\|symlink` | Copy (default) or symlink the skill files. |
| `--dry-run` | Report what would happen without writing. |
| `--list-agents` | List targetable agents and exit. |

```bash
adr skill install --all --dry-run
```

## Exit behaviour

Every command exits `0` on success and `1` on error, printing `Error: <message>`
to standard error. This makes the tool safe to chain in scripts and CI.

```bash
adr new "Record the decision" && git add docs/adr && git commit -m "docs(adr): record decision"
```
