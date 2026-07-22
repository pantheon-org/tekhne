# Task: Supersede an Earlier Decision

Eighteen months ago the team recorded a decision to use REST for all internal
service communication. They have now agreed to move internal traffic to gRPC
for performance and streaming, while leaving public APIs on REST.

The ADR log currently contains:

```text
docs/adr/0001-use-postgresql-for-primary-store.md   (Accepted)
docs/adr/0002-use-rest-for-internal-services.md      (Accepted)
docs/adr/0003-adopt-structured-logging.md            (Accepted)
```

## What to do

Retire ADR-0002 in favour of a new decision "Adopt gRPC for internal services",
preserving the history. Do not edit the decision text of ADR-0002.

## Output Specification

Produce:

1. The single command used to perform the transition.
2. What changed in ADR-0002.
3. The number, file name, status, and cross-reference of the new record.
