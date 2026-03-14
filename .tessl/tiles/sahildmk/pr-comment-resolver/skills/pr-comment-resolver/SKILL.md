---
name: pr-comment-resolver
description: |
  Resolve pull request review comments by critically assessing each one, fixing source files (not generated files), regenerating derived artifacts, running lint/format, committing, pushing, and replying to comment threads. Use when the user asks to address PR feedback, respond to reviewer suggestions, fix issues from code review, or resolve GitHub review comments.
---

# PR Comment Resolver

Resolve unaddressed review comments on a GitHub PR by fixing the underlying source code, verifying the fix, committing, pushing, and replying to the comment thread.

## Inputs

The user will provide one of:
- A PR number (e.g. `#689`)
- A PR URL (e.g. `https://github.com/org/repo/pull/689`)
- A request to look at "the PR" (infer from the current branch)

If no specific comment is mentioned, address all unresolved review comments.

## Workflow

### 1. Identify the PR and fetch review comments

```bash
# If on a feature branch, find the associated PR
gh pr view --json number,url

# Get the authenticated user's login (used to detect our own replies)
gh api user --jq '.login'

# Fetch review comments (not issue comments)
gh api repos/{owner}/{repo}/pulls/{pr_number}/comments
```

#### Filtering out already-handled comments

1. **Exclude comments we already replied to.** Skip any top-level comment (no `in_reply_to_id`) that has a child reply with `user.login` matching the authenticated user's login.
2. **Exclude old comments from prior runs.** Find the last agent commit timestamp, re-fetch with `?since={timestamp}`, then re-apply filter 1:
   ```bash
   git log --author="$(git config user.name)" --grep="Co-Authored-By: Claude" -1 --format=%cI
   gh api "repos/{owner}/{repo}/pulls/{pr_number}/comments?since={timestamp}"
   ```
   If no prior commit exists (first run), process all comments.

Focus on comments that are not marked resolved/outdated and contain actionable feedback from reviewers or bots.

**If no unaddressed comments remain after filtering, report this to the user and stop.**

### 2. Assess each comment

**Do not blindly action every comment.** For each comment:

1. Read the comment body and understand the concern
2. Assess: Is it correct? Does it apply? Should it be fixed now or deferred?
3. Decide: **address**, **defer**, or **disagree**
4. If addressing: identify the **source file** (never generated files like `openapi.json`, `types.d.ts`, build artifacts)
5. Read the source file for context and determine the minimal fix

### 3. Confirm plan with the user

Once you have assessed all comments, **present your plan to the user before making any changes**. Use the ask question tool (or equivalent structured prompt) to show:

- For each comment: a brief summary of the reviewer's concern and your proposed action (**address**, **defer**, or **disagree**) with a one-line rationale
- If addressing: which source file you intend to edit and a short description of the fix

Ask the user to confirm the plan or adjust individual items. Only proceed with fixes after receiving confirmation. This prevents wasted effort on misunderstood feedback and keeps the user in control.

### 4. Apply the fix

- Edit the **source file** only. If the issue is in a generated file, find and fix the source that generates it.
- Keep changes minimal and focused — fix exactly what the comment asks for, nothing more.

### 5. Regenerate derived artifacts

If the project has generated files affected by the change (e.g. OpenAPI specs, type definitions, compiled output), run the appropriate regeneration command. Check CLAUDE.md or project docs for the correct command (e.g. `bun run api:sync`).

### 6. Verify the fix

- Confirm the generated output no longer has the reported issue
- Run the project's formatter and linter (e.g. `bun run format && bun run lint`)
- Run relevant tests if applicable

### 7. Commit and push

Stage only the relevant files (source + regenerated artifacts). Write a clear commit message:

```
fix: <concise description of what was fixed>

<brief explanation of root cause and what changed>

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
```

Push to the current branch.

### 8. Reply to the PR comment

Reply **directly on the review comment thread** so the reviewer sees it inline.

**Replies must be very short, concise, and clear.** No fluff, no preamble. A few words to a single sentence is ideal. Examples:

- If addressed: `"Fixed in <sha> — updated the return type to match the schema."`
- If deferred: `"Valid point — deferring to a follow-up since it requires a broader refactor."`
- If disagreed: `"Keeping as-is — the explicit check is needed here because <brief reason>."`

When you've made a decision that differs from what the reviewer suggested, briefly explain **why**. Don't just say "won't fix" — give the reasoning in one sentence.

#### Posting the reply

```bash
gh api repos/{owner}/{repo}/pulls/comments/{comment_id}/replies \
  -f body="<short reply>"
```

Use the `comment_id` from step 1 (the top-level review comment's `id`, not a child reply's ID).

**If the `/replies` endpoint fails, do not immediately fall back to a top-level comment.** Diagnose first:
- Verify `owner`, `repo`, and `comment_id` — common mistakes: using a reply's ID instead of the top-level ID, wrong owner for forks, or swapped PR number and comment ID
- Re-fetch comments and confirm the correct top-level `id`, then retry
- Try alternate form: `gh api repos/{owner}/{repo}/pulls/{pr_number}/comments/{comment_id}/replies`
- **Only after all retries fail**, fall back to a top-level issue comment:
  ```bash
  gh api repos/{owner}/{repo}/issues/{pr_number}/comments \
    -f body="<short reply referencing the comment>"
  ```

## Important rules

- **Confirm with the user** before making changes (step 3). If a comment is unclear, ask before proceeding.
- **Source files only.** Never edit generated files — fix the source, then regenerate.
- **Minimal fixes.** No unrelated refactors or improvements.
- **Short replies.** One sentence max. Include a brief reason when disagreeing or deferring.
- **One commit per comment** unless issues share a root cause.
- **Deferring is valid.** Say so and move on.
