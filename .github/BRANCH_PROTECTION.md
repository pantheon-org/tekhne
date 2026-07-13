# Branch protection: Plumber ISSUE-505 (risk acceptance)

Decision (13-07-2026), owner **@thoroc**: the finding is **accepted, not
remediated**. Rationale below. Revisit if the project gains more maintainers.

## The finding

The Plumber CI/CD security scan reports one open finding on `main`:

- Rule `ISSUE-505`, severity high: `Branch 'main' has non-compliant protection settings`
- Reason reported: `Code owner approval is not required`
- Docs: https://getplumber.io/docs/cli/issues/ISSUE-505

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
- Required status checks: Skill Audit, CodeQL, code quality (errors)
- Code-scanning gate (CodeQL, high-or-higher)

The single residual gap is code-owner review, which carries no security value
for a solo maintainer.

## Impact

None on CI. The Plumber check is advisory (`soft-fail` + `continue-on-error`),
so ISSUE-505 does not block PRs. The repository is public and the SARIF alert
remains visible in the Security tab for transparency.

## Revisit trigger

If additional maintainers join, reconsider requiring code-owner review: add a
`CODEOWNERS` file and set `require_code_owner_review: true` on the ruleset (keep
a bypass actor for the `pantheon-ai-bot` automation), which clears ISSUE-505 and
moves the score toward an A.
