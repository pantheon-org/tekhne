# Task: Supersede an Earlier Decision

Eighteen months ago the team recorded a decision to use REST for all internal
service communication. They have now agreed to move internal traffic to gRPC for
performance and streaming, while leaving public APIs on REST. The work is on
branch `feat/adopt-grpc-for-internal-services`.

`docs/adr/` currently contains:

```text
use-postgresql-for-primary-store.md   accepted
use-rest-for-internal-services.md     accepted
adopt-structured-logging.md           accepted
```

## What to do

Retire `use-rest-for-internal-services` in favour of the new decision,
preserving the history. Do not edit the decision text of the old record.

## Output Specification

Produce:

1. The exact commands used, in order.
2. What changed in the old record's frontmatter, and what changed in the
   replacement's.
3. What was deliberately left untouched, and why that matters.
