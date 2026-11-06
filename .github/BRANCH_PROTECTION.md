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
- Required status checks: `Skill Audit`, `zizmor` and `Conventional Commit Title` (the three entries in the ruleset's status-check list, verified against the API on 06-11-2026)
- Code-scanning gate (CodeQL, high-or-higher), a separate ruleset rule
- Code-quality gate (errors), a separate ruleset rule

The single residual gap is code-owner review, which carries no security value
for a solo maintainer.

## Requiring the stored-audit checks (proposed 02-10-2026, not yet applied)

Two new checks stop a skill's stored audit, and the rating the catalogue shows,
from going stale. They exist as workflows but are **not required yet**: a
required check is a ruleset setting that only a repository administrator can
change, so this section records the request. Once the ruleset is updated, move
the two names into the list under "What `main` still enforces" and delete the
"not yet applied" wording above.

### Checks to require

| Check name (exact) | Workflow | What it enforces |
| --- | --- | --- |
| `Stored Audits` | `stored-audits.yml` | A pull request that changes a skill must leave that skill's stored audit current, and comments with the fix command if not. |
| `Catalogue Up To Date` | `stored-audits.yml` | The generated skills catalogue must match the stored audits, so a commit that skipped the local hook cannot leave it stale. |

Both run on every pull request with no path filter, so they always report. The
`Stored Audits` job is skipped for pull requests from forks and for pushes; a
skipped job counts as passing for a required check, so this does not block
anyone, but it means fork pull requests are not covered (the Skill Audit check
has the same limit).

### Check deliberately left out

`Stored Audits Full Tree` (`stored-audits-full-tree.yml`) is **not** proposed as
required. On pull requests it only runs when the scorer changes (a path filter),
and GitHub waits indefinitely on a required check that never runs, which would
block every other pull request. It already runs on each push to `main` as a
safety net and fails on a scorer pull request that invalidates stored audits. To
make it required later, move the path test inside the workflow (a first step
that exits early) so it always reports, and treat that as a separate change.

### How to apply

In the repository settings, edit the ruleset named `main` (id `13518481`), open
"Require status checks to pass", and add `Stored Audits` and
`Catalogue Up To Date`, source GitHub Actions. The existing "Require branches to
be up to date before merging" setting is already on, so no other change is
needed. The same change through the API adds two entries next to the existing
`Skill Audit` one:

```json
{ "context": "Stored Audits", "integration_id": 15368 }
{ "context": "Catalogue Up To Date", "integration_id": 15368 }
```

### Why this is safe to require now

The checks were proven in both directions on throwaway pull requests on
02-10-2026: a stale audit fails with a comment and passes once fixed, and a
wrong catalogue fails with the differing line printed. They start from a clean
`main` (all 164 stored audits current). Manifest-only and changelog-only bot
changes inside a skill leave its score unchanged, so they pass.

### Revisit trigger

There is no skip label, so the only escape from a misfiring required check is
reverting the workflow. If either check is reverted or disabled more than once
within 30 days of becoming required, add a documented skip label for
maintainers.

## Requiring the workflow security and title checks (applied 06-11-2026)

Two checks stop a workflow security problem, or a pull request title that would
produce a wrong version bump, from merging. They have been required since
06-11-2026: the ruleset's status-check list holds `Skill Audit`, `zizmor` and
`Conventional Commit Title`. Before that a pull request could go red on either of
the new two and still merge. (The stored-audit checks proposed above are not
applied.)

### Checks required

| Check name (exact) | Workflow | What it enforces |
| --- | --- | --- |
| `zizmor` | `zizmor.yml` | No high-severity workflow security finding: credential persistence, template injection, over-broad permissions, unpinned actions. |
| `Conventional Commit Title` | `conventional-commits.yml` | The pull request title is `type(scope): lowercase summary` with a type release-please recognises, since the squash-merge title is what it reads. |

Both run on every pull request with no path filter, so they always report. A
check that is skipped would block the pull request forever once required.

### Checks deliberately left out

- `Commit Types` (`commit-types.yml`) guards the title gate's copy of the type
  list. It is path-filtered, so it is **not** required, for the same
  reason as `Stored Audits Full Tree` above. It still fails the pull request that
  breaks the copy.
- `actionlint` runs only as a pre-commit step (`hk`), not as a workflow, so there
  is no status check to require. If it should gate merges it needs a workflow
  first.

### Known limit: a pull request can change the check that judges it

A required check runs the workflow as the pull request defines it. A pull
request that edits `zizmor.yml` or `conventional-commits.yml`, for example by
adding `if: false` to the job, runs its own altered copy, and a skipped job
counts as passing. The ruleset requires 0 approving reviews, so nothing but the
author's own judgement stops that merge.

This is true of every status check that runs on `pull_request`, `Skill Audit`
and `Stored Audits` included, so these two do not add a new kind of gap.
Running a trusted copy from the base branch against the pull request's code
would need `pull_request_target` plus a checkout of the pull request, the unsafe
combination zizmor itself warns about, so it is not used.

The fix is a repository setting, not a workflow change: require at least one
approving review, or add a `CODEOWNERS` entry for `.github/workflows/**` and
require code-owner review. The final "Revisit trigger" section covers the second
for the day more maintainers join.

### How it was applied

On 06-11-2026, through the API, to the ruleset named `main` (id `13518481`): two
entries were added next to the existing `Skill Audit` one, and every other rule,
the bypass list and the strict-policy setting were left identical. The result
was read back to confirm that.

```json
{ "context": "zizmor", "integration_id": 15368 }
{ "context": "Conventional Commit Title", "integration_id": 15368 }
```

To undo it, remove those two entries from "Require status checks to pass" in the
ruleset settings. The check name is the job's `name:`, and the ruleset matches on
it. Renaming a job silently drops the requirement, which is why each job carries
a comment saying so.

### Proof before requiring them (06-11-2026)

Each check was shown to fail and to pass on a throwaway draft pull request, both
closed unmerged afterwards. A required check that was never shown to fail has
not been shown to work.

- `Conventional Commit Title`, pull request 473: failed on the title "Add a
  throwaway proof of the title gate", with the received title in the log, then
  passed after the title was changed to "ci: throwaway proof of the title gate".
  Editing a title re-runs the check.
- `zizmor`, pull request 474: failed with one high-severity `template-injection`
  finding on a planted manual-only workflow, then passed once that file was
  removed.

The title gate rejects titles that merge freely today (for example "Add
blocks/blocked-by dependency links"). AGENTS.md states the allowed types.

### Revisit trigger

There is no skip label, so the only escape from a misfiring required check is
reverting the workflow. If either check is reverted or disabled more than once
within 30 days of becoming required on 06-11-2026, add a documented skip label
for maintainers.

## Impact

None on CI. The Plumber check is advisory (`soft-fail` + `continue-on-error`),
so ISSUE-505 does not block PRs. The repository is public and the SARIF alert
remains visible in the Security tab for transparency.

## Revisit trigger

If additional maintainers join, reconsider requiring code-owner review: add a
`CODEOWNERS` file and set `require_code_owner_review: true` on the ruleset (keep
a bypass actor for the `pantheon-ai-bot` automation), which clears ISSUE-505 and
moves the score toward an A.
