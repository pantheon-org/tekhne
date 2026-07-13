# Branch protection: Plumber ISSUE-505 (decision-support)

This is decision-support for a repository-governance owner. The change it
proposes is a repo setting (a ruleset), not code, so nothing here is applied
automatically. A human must decide and document the final choice.

## The finding

The Plumber CI/CD security scan reports one open finding on `main`:

- Rule `ISSUE-505`, severity high: `Branch 'main' has non-compliant protection settings`
- Reason reported: `Code owner approval is not required`
- Docs: https://getplumber.io/docs/cli/issues/ISSUE-505

It is the only finding left after the authorized-sources tuning; the score is
otherwise **B (85/100)**. Clearing it would move the repository toward an A.

## Root cause

`main` is protected by a **ruleset** (id `13518481`, active). Its `pull_request`
rule currently sets:

| Setting | Current |
| --- | --- |
| `require_code_owner_review` | `false` |
| `required_approving_review_count` | `0` |
| `require_last_push_approval` | `false` |
| `required_review_thread_resolution` | `true` |
| `required_linear_history` | enabled |
| required status checks | Skill Audit, CodeQL, code quality |

Plumber's GitHub `branchMustBeProtected` control expects code-owner review to be
required. Two facts matter for the fix:

1. There is **no `CODEOWNERS`** file in the repository, so code-owner review
   cannot be required until one exists.
2. The `.plumber.yaml` fields `codeOwnerApprovalRequired`, `minMergeAccessLevel`,
   and `minPushAccessLevel` are GitLab access-model knobs. Testing confirmed they
   do **not** relax the GitHub check: ISSUE-505 persists with the same reason
   regardless of their values. So the finding cannot be tuned away in config the
   way the authorized-sources finding was; it is resolved either by changing the
   ruleset or by accepting the risk.

## Options

### Option 1: make `main` compliant (recommended if the team uses reviews)

Two steps, both governance decisions:

1. **Add a `CODEOWNERS` file.** Owners must be real teams or users with write
   access. Do not guess them. A starting point to adapt:

   ```
   # .github/CODEOWNERS
   # Replace with the actual owning team(s). Left as a placeholder on purpose.
   * @pantheon-org/<your-maintainers-team>
   ```

2. **Require code-owner review on the `main` ruleset.** This fetches the current
   ruleset, flips `require_code_owner_review` to `true`, and sets one required
   approval so the review actually gates, then writes it back:

   ```bash
   gh api repos/pantheon-org/tekhne/rulesets/13518481 > ruleset.json
   jq '(.rules[] | select(.type=="pull_request") | .parameters)
        |= (.require_code_owner_review = true
            | .required_approving_review_count = (if .required_approving_review_count < 1 then 1 else .required_approving_review_count end))' \
     ruleset.json > ruleset.patched.json
   gh api --method PUT repos/pantheon-org/tekhne/rulesets/13518481 --input ruleset.patched.json
   ```

   Review `ruleset.patched.json` before the `PUT`. Confirm the bypass-actor list
   still matches policy (the automation that rebases and merges PRs, e.g.
   `pantheon-ai-bot`, may need a bypass entry so it is not blocked by the new
   review requirement).

Trade-off: every PR then needs a code-owner approval. Confirm the team has the
reviewer capacity and that Dependabot/automation flows still work (they may need
a bypass actor or an auto-approval step).

### Option 2: accept the residual finding

The repository is public and already scores B. If a mandatory code-owner review
does not fit the team's workflow, the accepted path is to record the risk
acceptance (owner, date, rationale) and leave the scan advisory. The Plumber
check does not block PRs, so this has no CI impact.

## Recommendation

Route this to the governance owner. If the project intends to enforce reviews,
Option 1 is the durable fix and gets to an A; otherwise document Option 2. Either
way the decision and its owner should be recorded.
