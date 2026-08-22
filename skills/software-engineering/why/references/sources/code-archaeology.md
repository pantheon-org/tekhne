# Code Archaeology (git + in-repo)

> Adapted for this session's `tokensave` rule: wherever a `tokensave_*` graph tool covers the same ground as a raw git/`rg` command below, use it first — it's cheaper to read (semantic symbol-level summaries, not raw text) and, for blame/log, tracks the symbol across renames rather than just the file path. Raw git/`rg` stays the fallback for what tokensave doesn't index: prose docs, non-symbol line ranges, full patch text, and pickaxe-style history search (`git log -S`/`-G`, which searches *when a string entered history*, not tokensave's *current* graph).

## What this source contains

- Commit history (messages, dates, authors, diffs)
- PR descriptions, review comments, and discussion threads (via `gh`, or `mcp-gitlab` for GitLab MRs)
- Inline code comments, TODOs, FIXMEs, deprecation notes
- ADRs (architectural decision records) if the repo keeps them
- Tests. Names and assertions often encode the edge cases that motivated a change
- Related files modified in the same commits (co-change signal)
- CHANGELOG entries, release notes in the repo
- Issue/ticket IDs mentioned in commit messages and PR/MR bodies

The most trustworthy source, tied directly to the code, and the most complete. Everything that went through the repo should be here. If the repo has no remote, PR/MR discussion doesn't exist — say so as a structural gap rather than a search failure.

## How to search it

Expand the seed commit list. Lead with `tokensave` for anything symbol-shaped:

```
tokensave_log(symbol, file?, limit)      # every commit that structurally changed this symbol, oldest-first, across renames
tokensave_blame(symbol, file?)           # most recent commit that structurally changed it
tokensave_diff(from?, to?, path?)        # semantic added/removed/modified summary between two refs (or working tree vs HEAD)
tokensave_changelog(from_ref, to_ref)    # same idea, explicit ref-to-ref
```

Fall back to raw git for what those don't cover:

```bash
# Full history of the file through renames, at file granularity rather than per-symbol
git log --follow --oneline -- <file>

# Pickaxe: commits that added or removed this exact text — searches history, not tokensave's current-state graph
git log -S '<exact_string_from_code>' -- <file>

# Or for patterns:
git log -G '<regex>' -- <file>

# Line-precise blame that doesn't align to one symbol (a line inside a large function, a config block, a comment)
git blame -L <start>,<end> <file>

# The full diff of a specific commit, raw patch text
git show <hash>

# Commits between two points affecting this file, raw patches
git log <old>..<new> -p -- <file>
```

For each substantive commit, pull the PR/MR context. Where GitHub is the remote:

```bash
# Find the PR number from the merge commit or branch
git log -1 --format=%B <hash>

# Full PR context: body, review comments, linked issues
gh pr view <number> --json title,body,author,createdAt,mergedAt,labels,closingIssuesReferences,comments,reviews,files

# The --json reviews and comments fields are where the real signal is
```

Where GitLab is the remote, use the `mcp-gitlab` tools instead (`get_merge_request`, `get_merge_request_notes`, `get_merge_request_diffs`, `get_merge_request_commits`).

Look for out-of-band docs and markers. `tokensave_todos` covers the marker scan directly (structured by kind, with the enclosing symbol named); `tokensave_search` with `literal: true` covers an exact-string sweep for a specific symbol name across tests. Prose search (ADRs in Markdown) isn't a code-graph concern, so `rg` stays the right tool there:

```
tokensave_todos(path?, kinds?)                       # TODO/FIXME/HACK/XXX/WIP/NOTE markers, with enclosing symbol
tokensave_search(query='<symbol>', literal=true, path_include=['test'])   # exact-match symbol usage inside test files
```

```bash
# ADRs often live in docs/adr/ or similar — prose search, not a graph concern
rg -l -i 'architecture.decision' --glob '*.md'
```

## What good evidence looks like here

- A PR/MR description that explains the problem being solved, not just the change ("This fixes the pagination bug that caused X")
- A long review thread where alternatives were debated
- An inline comment near the target line that explains a non-obvious constraint
- A test named `test_handles_edge_case_when_X` that reveals an edge case motivating the code
- A commit message that references a ticket or incident ID
- A CHANGELOG entry that summarizes the user-visible rationale

## Common pitfalls

- **Squash-merge flatlands.** If the repo squashes PRs/MRs, individual commits in the branch history are lost. Fall back to PR/MR body and comments.
- **Misleading commit messages.** "Small refactor" sometimes hides an intentional behavior change. Look at the diff, not the message.
- **Cargo-culted patterns.** The author may have copied a pattern without understanding why. Check if the pattern originated earlier in the codebase and investigate *that* commit.
- **Bot commits and auto-merges.** Dependabot, Renovate, and automated backports usually don't carry motivation. Skip them when trying to find intent.
- **Treating code as evidence of intent.** The code itself isn't evidence for why it exists. Evidence comes from commit messages, PRs/MRs, comments, tests, docs. Don't cite "the function is named X" as evidence of intent.

## What to return

Every commit/PR/MR/comment that bears on the question, with:
- The exact text (quoted)
- The hash / PR/MR number / file:line
- Author and date
- Whether it's direct (explicitly addresses the question) or circumstantial
