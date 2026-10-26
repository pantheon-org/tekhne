# Scenario 5: Write the Per-Type Split Index

## User Prompt

"The single index.yaml is too big to read. I want one index file per type, with a readable view next to each. Do it, and tell me how to check it stays correct."

## Input

A repository whose `.context/` holds plans, follow-ups and goals, with a
generated `.context/index.yaml`.

## Expected Behavior

1. Run `regenerate-context-index.sh --split` (the split is opt-in; plain runs are unchanged).
2. Confirm one `index-<type>.yaml` and one `index-<type>.md` exist per non-empty type, and none for an empty type.
3. Explain that the per-type files carry no date, and that the md files are generated views to be left alone.
4. Explain `--split --check` fails on a stale, missing or orphan file, and `--only <type> --stdout` renders one file without writing.
5. Not hand-edit any generated file, and not create an `index-*` context file by hand.

## Success Criteria

- `--split` used, not a hand-written or scripted copy of index.yaml.
- Pairs exist only for non-empty types.
- `--split --check` named as the verification.
- The `index-` prefix is treated as reserved for generated files.

## Failure Conditions

- Per-type files written by hand or by splitting index.yaml with ad hoc tooling.
- A pair kept for a type that has no entries.
- `index-*.md` indexed as a context entry or edited manually.
