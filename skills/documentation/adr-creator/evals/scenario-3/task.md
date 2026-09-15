# Task: Audit and Summarise an ADR Log

A new engineer has joined and asks for a summary of the architectural decisions
recorded so far, including which are still in force and which have been retired.

This repository keeps its records in `architecture/decisions` rather than the
default location. The directory contains:

```text
architecture/decisions/use-postgresql-for-primary-store.md
architecture/decisions/use-rest-for-internal-services.md
architecture/decisions/adopt-structured-logging.md
architecture/decisions/adopt-grpc-for-internal-services.md
architecture/decisions/index.md
architecture/decisions/README.md
```

`use-rest-for-internal-services` is superseded by
`adopt-grpc-for-internal-services`; the other three are accepted.

## What to do

List the records from the correct directory and produce a short summary that
distinguishes decisions in force from retired ones. Also give the command a CI
gate would use for the counts.

## Output Specification

Produce:

1. The command used to list the records from this repository's directory.
2. A summary grouping records into "In force" and "Retired", each named by slug.
3. The command that would give a CI gate the per-status counts, and its output
   shape.
4. A note on why `index.md` and `README.md` do not appear in the listing.
