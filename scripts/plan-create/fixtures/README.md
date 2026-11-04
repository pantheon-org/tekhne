# Fixtures for validate-plan-scaffold.sh (plan-create)

These are the golden cases for `skills/project-mgmt/plan-create/scripts/validate-plan-scaffold.sh`. Each file in `cases/` is a synthetic
input (placeholder text, no real document). The extension is `.fixture`, so the yaml hooks do not reject the deliberately
invalid ones. `invocations.txt` lists the runs, one per line as `name|arguments`. `expected/` holds what the python
version printed for each run: `<name>.stdout`, `<name>.stderr` and `<name>.exit`.

## How the expected output was recorded

`scripts/validator-recording/record.sh plan-create validate-plan-scaffold scripts/plan-create/fixtures` runs the python version of the script from tekhne
commit `9fdd876c` (before the port) in the container of `scripts/validator-recording/Dockerfile`: python 3.12 and
PyYAML 6.0.2, the image pinned by digest, with `PYTHONUTF8=1` and `LC_ALL=C.UTF-8`. It runs from this folder with
relative paths, so the output holds no machine path. It needs docker and git.

## Where the TypeScript differs from the python version

The port gives the same exit status and the same lines on every invocation except the cases below. `expectations.json`
holds the expected output of each one, and the tests read it. A line that starts with `__PREFIX__` there is compared by
its start only, because bun's parser text differs from PyYAML's.

| Case | Why it differs |
| ---- | -------------- |
| `missing-file` | D1: python skips it |
| `missing-and-valid` | D1: python skips the missing file and passes the other |
| `phase-name-no-unquoted` | D5: YAML 1.2 reads no as a string, YAML 1.1 as false |
| `root-list` | D4: python crashes with a traceback, the port reports it |
| `root-scalar` | D4: python crashes with a traceback, the port reports it |
| `empty-file` | D4: python crashes with a traceback, the port reports it |
| `plan-list` | D4: python crashes with a traceback, the port reports it |
| `frontmatter-null` | D4: python crashes with a traceback, the port reports it |
| `phases-string` | D4: python crashes with a traceback, the port reports it |
| `phase-scalar` | D4: python crashes with a traceback, the port reports it |
| `tasks-number` | D4: python crashes with a traceback, the port reports it |
| `tasks-string` | D4: python takes the length of a string as the task count and passes it, the port reports it |
| `invalid-yaml` | D5: the parser text differs, only the start of the line is compared |

A byte order mark is stripped before parsing, as python does. Bytes that are not UTF-8 are reported, not accepted.

The python version loaded the schema file and never used it. The port does not read it, so a missing or damaged schema file no longer makes it fail.

The minimum bun is 1.3.10, the version tekhne's CI pins. Bun 1.2.21 reads `yes` as a boolean, so its verdicts differ.

## What protects these tests

Tekhne's CI does not run `bun test scripts/`. The `hk` pre-push hook does (`unit-tests`), so the protection is local to
whoever pushes.
