# Task: Audit and Summarise an ADR Log

A new engineer has joined and asks for a summary of the architectural decisions
recorded so far, including which ones are still in force and which have been
retired. The repository uses a non-default ADR directory configured via the
`ADR_DIR` environment variable set to `architecture/decisions`.

The directory contains:

```text
architecture/decisions/0001-use-postgresql-for-primary-store.md   (Accepted)
architecture/decisions/0002-use-rest-for-internal-services.md      (Superseded by ADR-0004)
architecture/decisions/0003-adopt-structured-logging.md            (Accepted)
architecture/decisions/0004-adopt-grpc-for-internal-services.md    (Accepted)
architecture/decisions/README.md
```

## What to do

List the ADR log from the correct directory and produce a short summary that
distinguishes decisions in force from retired ones.

## Output Specification

Produce:

1. The command used to list the records from the configured directory.
2. A summary grouping records into "In force" and "Retired", each with its
   number and title.
3. A note that `README.md` is not an ADR and is excluded.
