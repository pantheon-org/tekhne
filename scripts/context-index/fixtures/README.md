# Baseline generator

`regenerate-context-index.baseline.sh` is a frozen copy of
`skills/project-mgmt/context-index/scripts/regenerate-context-index.sh` as it was on
`origin/main` at 268add67 (26-10-2026), before the `--split` flag. The tests run it
beside the current generator to prove the default output is byte-identical. Do not
edit it and do not update it when the generator changes on purpose: update the test
that says why the default moved.
