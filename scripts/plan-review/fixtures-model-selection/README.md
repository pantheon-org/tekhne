# Fixtures for validate-model-selection.sh (plan-review)

These are the golden cases for `skills/project-mgmt/plan-review/scripts/validate-model-selection.sh`. Each file in `cases/` is a synthetic
input (placeholder text, no real document). The extension is `.fixture`, so the yaml hooks do not reject the deliberately
invalid ones. `invocations.txt` lists the runs, one per line as `name|arguments`. `expected/` holds what the python
version printed for each run: `<name>.stdout`, `<name>.stderr` and `<name>.exit`.

## How the expected output was recorded

`scripts/validator-recording/record.sh plan-review validate-model-selection scripts/plan-review/fixtures-model-selection` runs the python version of the script from tekhne
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
| `root-list` | D4: python crashes with a traceback, the port reports it |
| `root-scalar` | D4: python crashes with a traceback, the port reports it |
| `empty-file` | D4: python crashes with a traceback, the port reports it |
| `selection-string` | D4: python crashes with a traceback, the port reports it |
| `selection-list` | D4: python crashes with a traceback, the port reports it |
| `non-utf8-comment` | D4: python crashes with a traceback, the port reports it |
| `two-documents` | D5: bun returns an array for several documents, python reports invalid YAML |
| `invalid-yaml` | D5: the parser text differs |
| `tab-indent` | D5: the parser text differs |

A byte order mark is stripped before parsing, as python does. Bytes that are not UTF-8 are reported, not accepted.

The python version loaded the schema file and never used it. The port does not read it, so a missing or damaged schema file no longer makes it fail.

The minimum bun is 1.3.10, the version tekhne's CI pins. Bun 1.2.21 reads `yes` as a boolean, so its verdicts differ.

## What protects these tests

Tekhne's CI does not run `bun test scripts/`. The `hk` pre-push hook does (`unit-tests`), so the protection is local to
whoever pushes.
