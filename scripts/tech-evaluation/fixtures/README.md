# Fixtures for validate-tech-evaluation.sh (tech-evaluation)

These are the golden cases for `skills/project-mgmt/tech-evaluation/scripts/validate-tech-evaluation.sh`. Each file in `cases/` is a synthetic
input (placeholder text, no real document). The extension is `.fixture`, so the yaml hooks do not reject the deliberately
invalid ones. `invocations.txt` lists the runs, one per line as `name|arguments`. `expected/` holds what the python
version printed for each run: `<name>.stdout`, `<name>.stderr` and `<name>.exit`.

## How the expected output was recorded

`scripts/validator-recording/record.sh tech-evaluation validate-tech-evaluation scripts/tech-evaluation/fixtures` runs the python version of the script from tekhne
commit `9fdd876c` (before the port) in the container of `scripts/validator-recording/Dockerfile`: python 3.12 and
PyYAML 6.0.2, the image pinned by digest, with `PYTHONUTF8=1` and `LC_ALL=C.UTF-8`. It runs from this folder with
relative paths, so the output holds no machine path. It needs docker and git.

## Where the TypeScript differs from the python version

The port gives the same exit status and the same lines on every invocation except the cases below. `expectations.json`
holds the expected output of each one, and the tests read it. A line that starts with `__PREFIX__` there is compared by
its start only, because bun's parser text differs from PyYAML's. `__HEADER__` means only the header line and the exit status are compared.

| Case | Why it differs |
| ---- | -------------- |
| `invalid-yaml` | D5: the parser text differs |
| `rationale-number` | D4: python crashes with a traceback, the port reports it |
| `evaluation-string` | D4: python crashes with a traceback, the port reports it |
| `questions-string` | D4: python crashes with a traceback, the port reports it |
| `question-scalar` | D4: python crashes with a traceback, the port reports it |
| `evidence-string` | D4: python reads a string as a list of characters and reports missing fields, the port reports the shape |
| `evidence-scalar` | D4: python looks for the field names inside the string, the port reports the shape |
| `recommendation-string` | D4: python crashes with a traceback, the port reports it |

A byte order mark is stripped before parsing, as python does. Bytes that are not UTF-8 are reported, not accepted.

The minimum bun is 1.3.10, the version tekhne's CI pins. Bun 1.2.21 reads `yes` as a boolean, so its verdicts differ.

## What protects these tests

Tekhne's CI does not run `bun test scripts/`. The `hk` pre-push hook does (`unit-tests`), so the protection is local to
whoever pushes.
