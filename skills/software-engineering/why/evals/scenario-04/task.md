# Scenario 04: No Remote — Local-Only Source Control Investigation

## User Prompt

"Why does the retry queue cap out at 3 attempts instead of 5? This repo has no remote configured."

## Expected Behavior

1. The agent confirms there is no git remote (`git remote -v` or equivalent) before assuming PR/MR tooling is available.
2. Source-control investigation proceeds using local git history (or a connected code-graph tool's per-symbol history) only — `gh pr view` and remote MR discussion are skipped, not attempted and silently failed.
3. The absence of remote PR/MR discussion is named explicitly as a structural gap in "Sources Consulted" or "What We Don't Know," not glossed over.
4. Other evidence categories (issue tracker, chat, docs) are still investigated in parallel if MCPs for them are connected — the missing remote does not collapse the whole investigation to "we don't know."
5. Any claim about the retry-count decision is cited to a specific local commit, comment, or other found evidence — not asserted from the code's current shape alone.

## Success Criteria

- The response explicitly states that there is no remote and that PR/MR-based investigation was skipped for that reason.
- Source-control findings are cited to specific local commits or code-graph history entries.
- The output still attempts the other available evidence categories rather than treating the missing remote as blocking the whole investigation.

## Failure Conditions

- The agent tries `gh pr view` or equivalent remote tooling anyway and either fails silently or fabricates a plausible-sounding result.
- The absence of remote history is never mentioned, leaving the reader to assume full PR/MR coverage was searched.
- The agent treats "no remote" as a reason to skip the investigation entirely rather than falling back to local history.
