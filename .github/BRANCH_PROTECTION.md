# Branch protection: Plumber ISSUE-505 (risk acceptance)

Decision (13-07-2026), owner **@thoroc**: the finding is **accepted, not
remediated**. Rationale below. Revisit if the project gains more maintainers.

## The finding

The Plumber CI/CD security scan reports one open finding on `main`:

- Rule `ISSUE-505`, severity high: `Branch 'main' has non-compliant protection settings`
- Reason reported: `Code owner approval is not required`
- Docs: <https://getplumber.io/docs/cli/issues/ISSUE-505>

It is the only finding left after the authorized-sources tuning; the score is
otherwise **B (85/100)**.

## Why it is accepted rather than fixed

The only way to clear this finding is to require code-owner review on `main`
(`require_code_owner_review: true` plus a `CODEOWNERS` file). This project is a
**single-maintainer effort**, so mandatory review is impractical and harmful:

- GitHub does not let an author approve their own PR, so every change would need
  a second reviewer who does not exist, or a bypass that defeats the control.
- It would stall automation: Dependabot and the auto-rebase/merge workflow would
  block on a code-owner approval that never comes.

Verified while investigating: the `.plumber.yaml` fields
`codeOwnerApprovalRequired`, `minMergeAccessLevel`, and `minPushAccessLevel` are
GitLab access-model knobs and do **not** affect the GitHub check, so the finding
cannot be tuned away in config either.

## What `main` still enforces

Accepting this finding does not leave `main` unprotected. The active ruleset
(id `13518481`) still enforces:

- No force-push (non-fast-forward) and no branch deletion
- Linear history
- Pull request required before merge, with review-thread resolution
- Required status checks: see the table below
- Code-scanning gate (CodeQL, high-or-higher)

The single residual gap is code-owner review, which carries no security value
for a solo maintainer.

## Required status checks

Verified against ruleset `13518481` on 13-09-2026. The ruleset requires
exactly one check today:

| Check | Required now | Should be required |
| --- | --- | --- |
| `Skill Audit` | yes | yes |
| `Build, test, lint` (Rust CI) | no | yes |
| `zizmor` | no | yes |
| `conventional commit title` | no | yes |

An earlier revision of this document listed "Skill Audit, CodeQL, code
quality (errors)" as required. That was not accurate: the ruleset names only
`Skill Audit`. CodeQL runs and reports, and the separate code-scanning gate
is configured independently of the status-check list, but neither appears as
a required status check.

The three unrequired rows matter because a check that runs but is not
required blocks nothing. A pull request can go red on Rust CI, on the
workflow security scan, or on a malformed title that will produce a wrong
version bump, and still be merged.

Adding them is a repository settings change rather than a file in this
repository, so it is not applied by the commit that added this table. To
apply:

```sh
gh api repos/pantheon-org/tekhne/rulesets/13518481 --method PUT \
  --input ruleset.json   # with the contexts added to required_status_checks
```

Then verify by opening a pull request that deliberately fails one of them
and confirming it cannot be merged. A required check that was never tested
against a real failure has not been shown to work.

Note that `actionlint` is deliberately absent: it runs as a pre-commit hook
via `hk`, not as a workflow, so there is no status check to require. If it
should also gate merges it needs a CI workflow first.

## Impact

None on CI. The Plumber check is advisory (`soft-fail` + `continue-on-error`),
so ISSUE-505 does not block PRs. The repository is public and the SARIF alert
remains visible in the Security tab for transparency.

## Revisit trigger

If additional maintainers join, reconsider requiring code-owner review: add a
`CODEOWNERS` file and set `require_code_owner_review: true` on the ruleset (keep
a bypass actor for the `pantheon-ai-bot` automation), which clears ISSUE-505 and
moves the score toward an A.
