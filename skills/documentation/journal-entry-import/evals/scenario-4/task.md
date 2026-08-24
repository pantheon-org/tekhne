# Scenario 4: Annotate migrated external links (Mode C)

## User Prompt

"The old Bitbucket repos moved to gitlab.com/acme/platform. Point the links in these entries at GitLab."

## Input

Two entries contain Bitbucket links:

```markdown
- back-and-forth with the PR:
  <https://bitbucket.example.com/projects/AP/repos/order-service/pull-requests/166/overview>
- ran the [sql-file-executor](https://bitbucket.org/acme-tools/sql-file-executor/src/master/) against prod
```

Resolution facts (already looked up):

- `order-service` was renamed and migrated to `gitlab.com/acme/platform/commerce/order-service` (confidence high).
- `sql-file-executor` has no GitLab project; the only reference is a repo-mapping manifest entry pointing it at
  `git@bitbucket.org:acme-tools/sql-file-executor.git` (i.e. still on Bitbucket, not migrated).

## Expected Behavior

1. **Annotate, do not replace**: keep the original Bitbucket URL and append the GitLab pointer.
2. For `order-service`: append
   `([now on GitLab: acme/platform/commerce/order-service](https://gitlab.com/acme/platform/commerce/order-service))`.
   The Bitbucket PR number does not map to a GitLab MR number, so the pointer is the **repo root**, and the
   original PR link stays as the archived record.
3. For `sql-file-executor`: leave it as-is. A manifest entry pointing at `git@bitbucket.org:` means it was NOT
   migrated - do not invent a GitLab link for it.
4. Handle the markdown forms correctly: the autolink `<...>` gets the annotation after the `>`; the inline link
   `[text](url)` gets it after the closing `)`. Never insert inside the link display text.
5. Touched entries still pass their validators and markdownlint.

## Success Criteria

- `order-service` link annotated with the GitLab repo-root pointer; original Bitbucket URL retained.
- `sql-file-executor` left unchanged (correctly identified as not migrated).
- Annotation placed outside the link/autolink syntax, not inside display text.
- Entries validate; markdownlint clean.

## Failure Conditions

- Bitbucket URL replaced/removed instead of annotated.
- A GitLab link invented for `sql-file-executor` (the unmigrated repo).
- A fabricated GitLab MR number for PR 166 instead of a repo-root pointer.
- Annotation inserted inside `[...]` display text, breaking the link.
