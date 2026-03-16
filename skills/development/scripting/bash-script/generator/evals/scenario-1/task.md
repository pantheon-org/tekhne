# Service Health-Check CLI

## Problem/Feature Description

A platform team maintains a fleet of Linux microservices and needs a CLI tool that an on-call engineer can run to quickly check whether a named service is healthy. The tool should hit a configurable HTTP health endpoint, check that a systemd service is active, and optionally tail the last N lines of the service's log file. It must be easy to integrate into runbooks: new engineers should be able to run it with `--help` and immediately understand all options without reading any other documentation.

The tool also needs to be safe to hand to contractors who may have limited bash experience — any invalid option or missing required argument should produce a clear error message pointing back to the help text rather than a cryptic shell error.

## Output Specification

Produce a single file named `healthcheck.sh`. The script should:

- Accept at minimum: a service name (required positional or flag argument), optional URL override, optional log-tail line count, and a verbose/debug flag.
- Print a human-readable usage/help block when invoked with the help flag.
- Exit with a non-zero code and a clear message when a required argument is missing.
- Exit with a non-zero code and a clear message when an unrecognised option is passed.
