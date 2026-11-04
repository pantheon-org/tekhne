# Fixtures for the plan-review report validator

These are the golden cases for `skills/project-mgmt/plan-review/scripts/validate-review-report.sh`. Each case in
`cases/` is a synthetic report (placeholder text, no real review). The extension is `.fixture`, so the yaml hooks do
not reject the deliberately invalid ones. `invocations.txt` lists the runs, one per line as `name|arguments`.
`expected/` holds what the python version printed for each run: `<name>.stdout`, `<name>.stderr` and `<name>.exit`.

## How the expected output was recorded

`record.sh` runs the python version of the script at commit `562cd973` (before the port) inside the container in
`Dockerfile`: python 3.12.16 and PyYAML 6.0.2. It runs from this folder with relative paths, so the output holds no
machine path. Run `./record.sh` from anywhere in a checkout that has that commit, with docker and git.

## Where the TypeScript differs from the python version

The port gives the same exit status and the same lines on every invocation except these named differences:

| Case | Python | TypeScript on bun | Why |
| ---- | ------ | ----------------- | --- |
| `missing-file`, `missing-and-valid`, `missing-and-bad` | Skips a file that does not exist, and counts it in `OK: N file(s)` | Fails with `<file>: file not found`, exit 1 (D1) | A typo in a path passed. |
| `plan-yes` | `yes` is a boolean, so it is rejected as not a string | `yes` is the string `yes`, so it is accepted (D6) | PyYAML reads YAML 1.1, bun 1.3.10 reads YAML 1.2. |
| `plan-date` | A date is a date object, rejected as not a string | An unquoted date is a string, accepted (D6) | Same reason. |
| `hash-e-notation-unquoted` | An unquoted `12345678e999…` is a string of hex characters, accepted | It is a number, rejected as not a string (D6) | A YAML 1.2 float needs no dot. |
| `invalid-yaml`, `two-documents`, `tab-indent` | The parser's own multi-line text after the first line | The first line has the same form (`<file>: invalid YAML`); the parser text differs, and a tab or a second document may be read rather than rejected (D7) | Bun cannot reproduce PyYAML's text. The exit status is compared. |
| `non-utf8-comment` | An uncaught traceback on stderr, exit 1 | `<file>: not valid UTF-8`, exit 1 | Only the exit status is compared. |

A byte order mark is stripped before parsing, so `with-bom` matches python.

The minimum bun is 1.3.10, the version tekhne's CI pins. Bun 1.2.21 has `Bun.YAML.parse` but reads `yes` as a boolean, so
its verdict on `plan-yes` differs from 1.3.10.

## What protects these tests

Tekhne's CI does not run `bun test scripts/`. The `hk` pre-push hook does (`unit-tests`), so the protection is local to
whoever pushes.
