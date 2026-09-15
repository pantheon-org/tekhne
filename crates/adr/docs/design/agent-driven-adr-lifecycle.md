<!-- Preserved verbatim from the retired standalone Go adr CLI
(~/Projects/github/pantheon-org/adr, no git remote, last commit 2026-05-14).
This is the design rationale for the check/draft/review lifecycle that
pantheon-adr now implements. Kept as history: the code it describes is Go and
no longer exists, and the shapes it proposes were reworked during the port.
See crates/adr/src/ for what was actually built. -->

# Plan: Agent-Driven ADR Lifecycle

**Status**: proposed  
**Created**: 2026-05-13  
**Author**: Claude  

---

## Problem

The current tool creates an ADR skeleton and stops. The agent is expected to fill it in, but has no structured pull to do so, no way to know when it is "done", and no mechanism to get human eyes on it. The result is ADRs that stay as empty templates or never get created at all.

The goal of this plan is to make `adr` an active participant in the decision-making process — helping agents gather context, assess risk, validate completeness, and route decisions to humans for review.

---

## Lifecycle (target)

```
proposed
  │
  ├─ adr draft    → gathers git + graph context, outputs to stdout (agent fills the file)
  │
  ├─ adr check    → validates completeness, gives stopping signal
  │  (loop: agent refines until check passes)
  │
  ├─ adr review   → status = review-requested (ADR visible in PR diff)
  │
  └─ human reads, comments on PR diff
       │
       ├─ approved   → adr update --status accepted
       └─ feedback   → adr update --status proposed → edit → check → review
```

Existing statuses `deprecated` and `superseded` remain unchanged and apply at any point after `proposed`.

---

## New status: `review-requested`

Add `review-requested` to `types.ADRStatuses` between `proposed` and `accepted`.

| Status | Who sets it | Meaning |
|--------|-------------|---------|
| `proposed` | `adr create` | ADR exists, content in progress |
| `review-requested` | `adr review` | Agent considers it complete, awaiting human approval |
| `accepted` | `adr update` or human | Decision confirmed and implemented |
| `deprecated` | `adr update` | No longer relevant |
| `superseded` | `adr update --superseded-by` | Replaced by a newer ADR |

---

## Command: `adr draft [slug]`

**Purpose**: Aggregate all context the agent needs to write a complete ADR.  
**When**: Called by the agent after some initial editing of the ADR file (not immediately after `adr create` — see usage note below).  **Design**: Output-only. `adr draft` writes nothing to the ADR file — the agent reads the output (or `--json`) and edits the file directly. This keeps the command stateless and composable.

### Slug resolution

If `[slug]` is omitted:

1. Load all `ADREntry` records from the index sorted by `Created` descending.
2. Zero records → error: `no ADRs found in index; run 'adr create' first`.
3. One or more records → use the most recently created.

If `[slug]` is provided and not found in the index → error: `ADR '<slug>' not found`.  
If the resolved file path does not exist on disk → error: `ADR file not found: <path>`.

### Output (stdout, also `--json`)

```
## Branch Context
Branch:   feat/add-grpc
Type:     feat
Commits:  4
  • "add proto definitions for user service" (2026-05-10)
  • "implement grpc server with interceptors" (2026-05-11)
  • "add grpc client with retry logic"        (2026-05-12)
  • "add roundtrip tests"                     (2026-05-12)

## Changes
Files changed: 23  (+1,847 / -312)
  internal/grpc/          12 files  +1,200
  internal/server/         4 files    +400
  internal/auth/           3 files    +147

## Impact Analysis  [go list]
...

## Related ADRs
  • add-authentication   [proposed]  — shared tags: grpc, api
  • api-design           [accepted]  — shared tags: api

## Suggested sections to address
  • Rationale:      Why gRPC over REST/GraphQL?
  ...
```

### JSON schema (`--json`)

```json
{
  "branch": "feat/add-grpc",
  "type": "feat",
  "base": "main",
  "commits": [
    { "hash": "abc1234", "message": "add proto definitions for user service", "date": "2026-05-10" }
  ],
  "diff_stat": {
    "files_changed": 23,
    "insertions": 1847,
    "deletions": 312,
    "by_dir": [
      { "path": "internal/grpc/", "files": 12, "insertions": 1200, "deletions": 0 }
    ]
  },
  "impact_analysis": null,
  "related_adrs": [
    { "slug": "add-authentication", "status": "proposed", "shared_tags": ["grpc", "api"] }
  ],
  "suggested_sections": ["Rationale", "Non-Goals"],
  "adr_file": ".context/decisions/add-grpc.md"
}
```

`impact_analysis` is `null` when `--with-analysis` is absent. When present, it has the structure:

```json
"impact_analysis": {
  "changed_packages": ["github.com/acme/app/internal/grpc"],
  "direct_importers": ["github.com/acme/app/internal/api"],
  "transitive_importers": ["github.com/acme/app/cmd/server"],
  "high_fan_in": [
    { "package": "github.com/acme/app/cmd/server", "imports_changed": 3 }
  ]
}
```

### Flags

| Flag | Description |
|------|-------------|
| `--base string` | Base branch for git log/diff. Resolution order: this flag → `IndexMeta.DefaultBase` → `"main"`. |
| `--bootstrap` | When the ADR has ≥ 5 unfilled placeholders, output a structured question list derived from branch context instead of a missing-sections report. Designed for the first-pass gap: agent answers the questions and uses the output as initial content. |
| `--with-analysis` | Run Go import graph analysis (opt-in; requires `go list` in PATH) |
| `--pr` | Fetch PR comments for the current branch via `gh pr view --json comments`; surface comments on the ADR file as a `## Reviewer Feedback` section. Enables the feedback loop between review rounds. Skip silently if no PR exists or `gh` is unavailable. |
| `--json` | Machine-readable output |

### Edge cases

| Condition | Behaviour |
|-----------|-----------|
| No diverging commits (`git log <base>..HEAD` is empty) | Commits section shows "0 commits"; diff stat shows 0 files changed. Output is still valid — agent can draft context from the ADR description alone. |
| Slug not found in index | Error: `ADR '<slug>' not found in index`. |
| ADR file missing from disk | Error: `ADR file not found: <path>`. |
| `go.mod` absent when `--with-analysis` passed | Skip impact section silently (already covered by implementation notes). |
| `go list` exceeds 30s timeout | Skip impact section silently (same as failure). |
| `go list` fails | Skip impact section silently (already covered). |

### Implementation notes

- Git context: `git log <base>..HEAD --oneline`, `git diff <base>...HEAD --stat` (where `<base>` is resolved via `--base` flag → `IndexMeta.DefaultBase` → `"main"`)
- Impact analysis: **opt-in via `--with-analysis`**, only runs when flag is set AND `go.mod` exists
  - Shell out to `go list -json ./...` via `os/exec` to load the full package graph
  - Build a reverse-import map (`map[string][]string`) in memory — no external library needed
  - Determine changed packages from `git diff <base>...HEAD --name-only` → strip to package import paths
  - BFS outward from changed packages (depth ≤ 3) to compute direct and transitive importers
  - Flag packages that import ≥ 2 changed packages as high fan-in
  - Skip gracefully (omit the section) if `go list` fails
- `adr draft` refreshes `ADREntry.ChangedFiles` for the current ADR before any other computation: run `git diff <base>...HEAD --name-only`, update the in-memory index entry, and write it back atomically via the standard temp+rename path. This keeps the snapshot current as the branch evolves, so other ADRs' file-overlap detection remains accurate regardless of how many commits have landed since `adr create`.
- Related ADRs: two signals, both optional; omit section if neither matches
  - Primary: **shared tags** — scan `idx.Decisions` for tag overlap
  - Secondary: **overlapping changed files** — compare the refreshed `ADREntry.ChangedFiles` for the current ADR against `ChangedFiles` stored on every other `ADREntry`; any ADR whose stored files intersect is surfaced
- Suggested sections: derived from unfilled placeholders in the ADR file + high-fan-in packages (if `--with-analysis`)
- Bootstrap mode (`--bootstrap`): when ≥ 5 unfilled placeholders detected, suppress the missing-sections list and instead emit a question prompt per unfilled section, pre-populated with branch context (e.g. "You added 23 files to `internal/grpc/`. Why gRPC over REST?"). Each question maps to a template heading so the agent can fill sections directly.
- PR comments (`--pr`): shell out to `gh pr view --json comments,reviewComments`; filter to comments whose `path` matches the ADR file; render as `## Reviewer Feedback` block. Skip silently if no PR or `gh` fails.

### Usage note: when to call `adr draft`

`adr draft --bootstrap` is designed for the **immediately-after-create** step — when all placeholders are still unfilled, it asks structured questions instead of listing missing sections. After a first pass is written, drop `--bootstrap` and use plain `adr draft` to surface what's still missing and the blast-radius context. The right workflow is:

```
adr create                          # creates skeleton
adr draft --bootstrap               # structured question list → agent fills first pass
<agent edits file>
adr draft                           # surfaces what's still missing + blast radius context
<agent refines>
adr check                           # validates completeness
```

---

## Command: `adr check [slug]`

**Purpose**: Validate that an ADR is complete enough for human review.  
**When**: Called by the agent after drafting, and by the `session-end` hook.

### Completeness rules

| Rule | Weight | Pass condition | Mandatory cap |
|------|--------|----------------|---------------|
| No unfilled `<!-- … -->` placeholders | 20% | 0 placeholders remaining | ≥ 3 remaining → score capped at 70 |
| Problem Statement / Context non-empty | 25% | > 30 words | < 30 words (including absent) → score capped at 60 |
| Chosen Solution non-empty | 25% | > 30 words | < 30 words (including absent) → score capped at 60 |
| Rationale non-empty | 20% | > 20 words | — |
| Impact Assessment has at least one non-`none` entry | 10% | ≥ 1 filled | — |

> **Boilerplate filter**: applied after word-count rules. If a section passes the word-count threshold but > 60% of its tokens match a deny-list (`describe`, `decision`, `context`, `placeholder`, `example`, `tbd`, `todo`), that section is treated as failing and its word-count rule reverts to a fail. This prevents trivially padding sections to hit the threshold. The same deny-list words also count toward placeholder detection.

> **Risks & Pitfalls**: this section (added in Phase 1) is validated exclusively through the placeholder rule above — its `<!-- describe -->` marker must be removed, but no minimum word count is required. No separate scored rule is added; the weights above already sum to 100.

Score 0–100. Exit 0 if ≥ 80, exit 1 otherwise.

The cap for Problem Statement and Chosen Solution triggers whenever the word count is **below the threshold**, whether the section is absent or present-but-thin. A section with 1 word is treated the same as a missing section for cap purposes; the rule also fails in both cases. This prevents trivially bypassing the cap by inserting a single placeholder word.

The effective cap is the **minimum of all triggered caps** (not "stacking" — just take the lowest). For example, if both Chosen Solution is thin (cap 60) and ≥ 3 placeholders remain (cap 70), the effective cap is 60.

### Output

```
ADR: add-grpc
Score: 65/100  [incomplete]

Missing:
  ✗ Chosen Solution is empty                         (−25, caps score at 60)
  ✗ 3 unfilled placeholders remain in Rationale      (−20)

Filled:
  ✓ Problem Statement / Context
  ✓ Options Considered
  ✓ Impact Assessment
```

### Flags

| Flag | Description |
|------|-------------|
| `--json` | Machine-readable score + missing list |
| `--strict` | Require score ≥ 95 (all sections filled) |

---

## Command: `adr review <slug>`

**Purpose**: Mark an ADR as ready for human review.  
**When**: Called by the agent once `adr check` passes.

### Behaviour

1. Validates `adr check --json` score ≥ 80 (blocks if incomplete)
2. Updates index: `status = review-requested`
3. Prints confirmation:
   ```
   ADR marked for review: add-grpc
   File: .context/decisions/add-grpc.md
   Score: 85/100
   ```

The ADR file lives in the repository and appears in the PR diff. Reviewers comment on it there through the normal PR review process — no external routing needed.

### Feedback loop

If a reviewer requests changes:

1. `adr update <slug> --status proposed` — reverts to `proposed`
2. Agent edits the ADR file
3. `adr check` — validates completeness
4. `adr review` — re-marks as `review-requested`

> **Note on bypass**: `adr update --status accepted` still works directly and intentionally bypasses `review`. The `review-requested` status is a visibility signal for humans, not an enforcement gate. Teams that want strict review gates should enforce this at the PR level (e.g. a CI check on `adr status --json`).

---

## Hook updates

### `session-end` (updated)

Current behaviour: lists proposed ADRs.  
New behaviour:

1. For each `proposed` ADR in the index: call `RunCheck` **as an in-process library call** (not a subprocess). `RunCheck` must return a `CheckResult` struct — never call it via `exec.Command`. The call is wrapped in a `recover()` so that a panic (e.g. malformed ADR file, nil pointer) is caught, a warning is printed, and the hook continues. The hook must not propagate a non-zero exit code from check failures or panics; hook exit must always be 0 (otherwise it breaks the shell session-end flow).
   - If score < 80: print missing items, prompt agent to complete before ending session
2. For each `review-requested` ADR: print prominently — human review is pending

### `session-start` (updated)

Surface all `review-requested` ADRs with a distinct label — they need no agent action, just visibility.

For each `proposed` ADR, call `RunCheck` in-process. If score < 80, surface the score and the single highest-weight missing item, and append: `Run 'adr draft' for full context.` This gives the agent a proactive signal at session start without requiring an explicit invocation — `adr draft` remains pull, but the agent is nudged rather than left to remember. Advisory only (exit 0); no block. `IndexMeta.DefaultBase` (i.e. the session is on main/develop), also list all `accepted` ADRs and print an advisory:

```
Accepted ADRs on record — verify none are now stale:
  • add-grpc          [accepted]  feat/add-grpc
  • auth-session      [accepted]  feat/auth-overhaul
Run 'adr update <slug> --status deprecated' for any that are no longer active.
```

This is advisory only (exit 0). It fires the prompt that post-merge cleanup lacks: after branches merge to main, the next session on main is the natural moment to sweep stale accepted decisions.

Add to Phase 5 smoke tests: `session-start` on the default base branch with `accepted` ADRs in the index → lists them with the deprecation advisory.

### `post-tool-use` (no change)

Still notifies once per branch when no ADR exists.

### `pre-push` (new)

Runs `adr check --json` on every `proposed` ADR in the index; exits non-zero if any score < 80, printing the missing items. Exits 0 if no `proposed` ADRs exist or all pass. This is the actual enforcement gate — unlike session-end (which is advisory), pre-push blocks the push until ADRs are complete.

Installed explicitly via `adr init --install-hooks` (or `adr sync --install-hooks`). Must not silently install on plain `adr init`.

**Hooks manager detection** (run at install time, in priority order):

| Detected signal | Action |
|-----------------|--------|
| `lefthook.yml` / `lefthook.yaml` present | Append an `adr-check` command under `pre-push.commands` in `lefthook.yml` using `yq` or direct TOML/YAML edit. Print guidance if `lefthook` binary is absent. |
| `.husky/` directory present | Write / append `adr check --json \|\| exit 1` to `.husky/pre-push` (create file if absent; append if exists). |
| `.overcommit.yml` present | Print instructions to add a `ShellCommand` entry under `PrePush` — do not auto-edit (overcommit config is opinionated about structure). The printed instructions must include the exact YAML block to paste:<br><br>```yaml<br>PrePush:<br>  ShellCommand:<br>    adr-check:<br>      command: "adr check --json \|\| exit 1"<br>      description: "ADR completeness check"<br>``` |
| `.pre-commit-config.yaml` present | pre-commit does not manage `pre-push` by default; fall through to direct install. |
| None of the above | Write to `.git/hooks/pre-push`. If the file already exists and is non-empty, **append** the adr check call rather than overwriting. Mark file executable (`chmod +x`). |

In all cases: print what was done and how to undo it (`adr init --uninstall-hooks`).

---

## New ADR template section: Risks & Pitfalls

Add a dedicated section to the markdown template (between Impact Assessment and Outcome):

```markdown
## Risks & Pitfalls
<!-- Known risks accepted going into this decision -->
- **Risk**: <!-- describe -->  **Mitigation**: <!-- describe or "accepted" -->
```

This gives `adr check` a concrete section to validate and gives reviewers a clear place to look.

---

## Agent/automation usage note

`adr draft` and `adr check` support human-readable output by default (CLI convention). In hooks and agent workflows, **always pass `--json`** — the structured output is more reliable to parse and will not change format with terminal width or locale.

```sh
# In hooks and scripts — always --json
adr draft --json
adr check --json
adr status --json
```

---

## Implementation phases

Phases are grouped into **waves**. All phases within a wave have no dependencies on each other and can be implemented in parallel. Each wave must be complete before the next begins.

```
Wave 1: P0 ──┐
             ├──► Wave 2: P2 ──┐
Wave 1: P0.5─┘                 │
                               ├──► Wave 3: P4 ──┐
Wave 1: P1 ────► Wave 2: P3 ──┤                  ├──► Wave 4: P5
                               └──► Wave 3: PCI ──┘
```

### Execution model: subagents + worktrees

Each phase within a wave is implemented by a **dedicated subagent** working in its own **git worktree**. This keeps agents isolated, prevents file-level conflicts, and allows all phases in a wave to proceed simultaneously.

**Worktree setup per phase:**

```sh
# One worktree per phase, branched from the wave's merge base
rtk git worktree add ../adr-p0   feat/p0-git-helpers
rtk git worktree add ../adr-p0.5 feat/p0.5-schema
rtk git worktree add ../adr-p1   feat/p1-template
```

**Wave orchestration protocol:**

1. **Orchestrator** (this session) launches one subagent per phase in the wave, each pointed at its own worktree and branch.
2. Each **subagent** receives: its phase spec (files, tasks, acceptance criteria from this plan), the worktree path, and the branch name. It works autonomously: writes tests first (TDD), implements, runs `mise run test` and `mise run lint`, then signals completion.
3. Orchestrator **waits** for all subagents in the wave to complete and pass CI before merging their branches to `main` and starting the next wave.
4. **Merge order within a wave**: rebase each phase branch onto `main` in sequence (order within wave doesn't matter), resolve any minor conflicts, then proceed to the next wave.

**Subagent handoff template:**

> You are implementing **[Phase N — Name]** of the agent-driven ADR lifecycle plan.
> Worktree: `../adr-pN` on branch `feat/pN-<slug>`.
> Spec: [paste phase section from this plan].
> Rules: TDD (tests before implementation), `mise run test` must pass, `mise run lint` must be clean, no commits to `main`.
> Signal completion by returning: branch name, test count delta, coverage delta, lint status.

**Conflict surface is minimal by design**: each wave's phases touch disjoint files. The one exception is `internal/types/adr.go`, which is modified by both P0.5 (Wave 1) and P4 (Wave 3) — these are in different waves so no concurrent conflict arises.

---

### Wave 1 — No dependencies (implement in parallel)

#### Phase 0 — Refactor: extract git helpers
Files: `internal/git/git.go` (new), `internal/cli/helpers.go` (shrinks)

- Extract `currentBranch()` and `resolveAuthor()` from `internal/cli/helpers.go` into `internal/git/git.go`.
- `internal/git/git.go` owns **all stateless git primitives** (branch detection, author resolution, and any future low-level git calls).
- No behaviour change; existing callers updated to import the new package.
- `internal/git/context.go` (Phase 2) will live alongside `git.go` in the same package and adds `BranchContext` / `GetBranchContext`. The two files are intentionally separate: `git.go` = primitives, `context.go` = higher-level branch context assembly.

#### Phase 0.5 — Schema: add `DefaultBase` to `IndexMeta`
Files: `internal/types/adr.go`, `internal/cli/init.go`

Add `DefaultBase` to `IndexMeta`:

```go
type IndexMeta struct {
    // ... existing fields ...
    DefaultBase string `toml:"default_base,omitempty"` // base branch for git log/diff
}
```

- `adr init` detects the default base branch via `git symbolic-ref refs/remotes/origin/HEAD --short` (strips `origin/` prefix), falls back to `"main"` if detection fails or the repo has no remote, and stores the result in `IndexMeta.DefaultBase`.
- Existing indexes with no `DefaultBase` field fall back to `"main"` at runtime — no migration needed.
- `adr create` stores the list of changed files at create time: run `git diff <base>...HEAD --name-only` and write the result to a new `ChangedFiles []string` field on `ADREntry`. Used by `adr draft` for file-path-based related ADR detection. Empty slice if no diverging commits.

#### Phase 1 — Template update
Files: `internal/adr/template.go`

- Add Risks & Pitfalls section to the markdown template
- Update any snapshot tests that assert template output

---

### Wave 2 — After Wave 1 (implement in parallel)

#### Phase 2 — `adr draft` *(depends on P0, P0.5)*
Files: `internal/cli/draft.go`, `internal/git/context.go`, `internal/graph/imports.go`

- New `internal/git/context.go`: `BranchContext` struct, `GetBranchContext(base string)` function (commits, diff stat, changed file list). `base` is passed through from the `--base` flag.
- `RunDraft`: assembles `BranchContext`, formats output (human + `--json`)
- Related ADRs: tag-overlap + file-path overlap (against `ADREntry.ChangedFiles`); omit section if neither matches
- Suggested sections: derived from unfilled `<!-- … -->` placeholders in ADR file
- Bootstrap mode (`--bootstrap`): when ≥ 5 unfilled placeholders, emit structured questions per section instead of missing-sections report
- PR comments (`--pr`): shell out to `gh pr view --json comments,reviewComments`; filter to ADR file path; render as `## Reviewer Feedback`; skip silently on failure
- Impact analysis (`--with-analysis`): `ImportGraph` struct, `LoadImportGraph()` via `go list -json ./...`, `BlastRadius(pkgs []string, depth int)` — run via `exec.CommandContext` with 30-second timeout; skip section silently on timeout or non-zero exit; auto-skip if `go.mod` absent

#### Phase 3 — `adr check` *(depends on P1)*
Files: `internal/cli/check.go`, `internal/adr/completeness.go`

- `CheckResult` struct with score, caps applied, passing rules, failing rules
- Markdown parser: counts `<!-- … -->` placeholders, measures section word counts
- Boilerplate filter: if a section passes word-count threshold but > 60% of tokens match the deny-list (`describe`, `decision`, `context`, `placeholder`, `example`, `tbd`, `todo`), revert that section to failing
- `RunCheck`: loads ADR file, runs rules, applies caps and boilerplate filter, formats output (human + `--json`)

---

### Wave 3 — After Wave 2 (implement in parallel)

#### Phase 4 — `review-requested` status + `adr review` *(depends on P3)*
Files: `internal/types/adr.go`, `internal/cli/review.go`

- Add `review-requested` to `ADRStatuses`
- Add `ChangedFiles []string` to `ADREntry` (used by Phase 2 for file-path relatedness; populated at `adr create` time)
- Add `History []StatusTransition` to `ADREntry`, where `StatusTransition` is `{ From, To Status; At time.Time; Note string }`. `adr update` appends an entry on every status change.
- `RunReview`: validates check score ≥ 80, updates status, appends history entry, prints confirmation
- Feedback loop: `adr update --status proposed` is the documented revert path

#### Phase CI — CI tasks *(depends on P3)*
Files: `mise.toml`

- Add `check-adrs` mise task (exits 1 if any `proposed` ADRs exist)
- Add `check-adrs-strict` mise task (exits 1 if any `proposed` or `review-requested` ADRs exist)

---

### Wave 4 — After Waves 2 + 3 (sequential)

#### Phase 5 — Hook updates + pre-push installer *(depends on P3, P4)*
Files: `internal/cli/hooks.go`, `.claude/settings.json`, `internal/cli/init.go`

- Update `RunSessionEnd` to call `RunCheck` **in-process** (returning `CheckResult`, never via subprocess) on all `proposed` ADRs in the index; wrap each call with `recover()` so a panic prints a warning and the hook continues with exit 0
- Update `RunSessionStart` to surface all `review-requested` ADRs
- Add `pre-push` hook installer: `RunInstallHooks` in `internal/cli/init.go`
  - Detect hooks manager in priority order: lefthook → husky → overcommit → direct `.git/hooks/`
  - lefthook: append `adr-check` command to `pre-push.commands` in `lefthook.yml`
  - husky: append `adr check --json || exit 1` to `.husky/pre-push` (create if absent)
  - overcommit: print manual instructions, do not auto-edit
  - direct: append to existing `.git/hooks/pre-push` if non-empty; create if absent; `chmod +x`
  - In all cases: print what was done and mention `adr init --uninstall-hooks` to undo
- `RunUninstallHooks`: reverse of install — removes or reverts the adr-check entry per manager type

---

## Out of scope

- MCP server mode (`adr mcp serve`) — right end state but a separate project
- LLM-assisted content generation inside the CLI — the agent IS the LLM; the CLI feeds it context
- Automated acceptance without human review
- Multi-reviewer workflows or approval quorum
- Non-Go import graph analysis — `--with-analysis` shells out to `go list` and is Go-only; TypeScript, Python, and other language blast-radius analysis requires separate tooling outside this project
- CI/CD workflow templates beyond `mise run check-adrs` / `check-adrs-strict` — GitHub Actions, GitLab CI, CircleCI configs are out of scope; teams wire the mise tasks into their own pipelines

## CI integration

Add a `mise` task so teams can gate PRs on ADR completeness without writing their own scripts:

```toml
[tasks.check-adrs]
run = "adr status --json | jq -e 'if .by_status.proposed then .by_status.proposed == 0 else true end'"
```

Returns exit 0 if no `proposed` ADRs exist (index empty or all advanced). Returns exit 1 if any ADR is still `proposed`. Wire to CI as a required status check; the pre-push hook provides the same gate locally.

For teams that want to enforce the full review gate (blocking `adr update --status accepted` bypasses), add a strict task that also rejects `review-requested` ADRs:

```toml
[tasks.check-adrs-strict]
run = "adr status --json | jq -e '(.by_status.proposed // 0) == 0 and (.by_status[\"review-requested\"] // 0) == 0'"
```

`check-adrs-strict` blocks merges until every ADR has passed through `adr review` and been explicitly `accepted`. Use `check-adrs` for advisory enforcement; use `check-adrs-strict` when the review-requested handoff must be enforced at the PR level.

---

## Verification

Each phase ships independently and is testable. Phases within the same wave can be verified concurrently:

| Phase | Smoke test |
|-------|------------|
| 0 | Existing tests pass unchanged after refactor |
| 0.5 | `adr init` on repo with remote → `DefaultBase` stored in index; on repo with no remote → `DefaultBase` is `"main"` |
| 0.5 | `adr draft` without `--base` on a repo with `DefaultBase = "develop"` → uses `develop` as base |
| 0.5 | `adr create` on branch with diverging commits → `ADREntry.ChangedFiles` non-empty; on branch with no commits → empty slice |
| 1 | `adr create` produces template with Risks & Pitfalls section |
| 2 | `adr draft` on a repo with commits → non-empty output; `--json` parses cleanly |
| 2 | `adr draft` on a branch with new commits added after `adr create` → `ADREntry.ChangedFiles` updated in index to reflect full branch diff |
| 2 | `adr draft` when a second ADR shares changed files → surfaces it in Related ADRs even with no shared tags |
| 2 | `adr draft --bootstrap` on fresh skeleton (≥ 5 placeholders) → outputs question list, not missing-sections report |
| 2 | `adr draft --pr` when PR exists with ADR file comments → surfaces them as Reviewer Feedback; when no PR → no crash |
| 2 | `adr draft --with-analysis` on a Go repo → includes blast radius; without flag → no impact section |
| 2 | `adr draft --with-analysis` on non-Go repo → no crash, no impact section |
| 2 | `adr draft --with-analysis` when `go list` times out → impact section omitted, rest of output intact |
| 3 | `adr check` on empty template → score < 80, exit 1; on filled ADR → score ≥ 80, exit 0 |
| 3 | `adr check` with empty Chosen Solution → score capped at 60 regardless of other sections |
| 3 | `adr check` on ADR with Problem Statement filled with deny-list words only → section treated as failing |
| 4 | `adr review` on incomplete ADR → blocked with score; on complete ADR → status updated, file path printed |
| 4 | `adr update --status accepted` on an ADR → `History` array gains one entry with correct From/To/At |
| 5 | `session-end` with incomplete `proposed` ADR in index → lists missing items |
| 5 | `session-end` with `review-requested` ADR in index → surfaces it prominently |
| 5 | `session-end` when `RunCheck` panics on a malformed ADR → warning printed, exit 0 |
| 5 | `session-start` with `review-requested` ADR in index → shown with distinct label |
| 5 | `session-start` on default base branch with `accepted` ADRs in index → lists them with deprecation advisory; on feature branch → no advisory shown |
| 5 | `session-start` with `proposed` ADR scoring < 80 → score and top missing item printed with `adr draft` suggestion; with `proposed` ADR scoring ≥ 80 → no nudge shown |
| 5 | `adr init --install-hooks` with no hooks manager → `.git/hooks/pre-push` written and executable |
| 5 | `adr init --install-hooks` with existing non-empty `.git/hooks/pre-push` → adr check appended, original content preserved |
| 5 | `adr init --install-hooks` with `lefthook.yml` present → entry added to `pre-push.commands`, file not clobbered |
| 5 | `adr init --install-hooks` with `.husky/` present → `.husky/pre-push` created or appended |
| 5 | `adr init --uninstall-hooks` → removes or reverts the adr-check entry without touching other hook content |
| 5 | pre-push hook with incomplete `proposed` ADR → exits non-zero, lists missing items |
| 5 | pre-push hook with no `proposed` ADRs → exits 0 |
| Phase CI | `mise run check-adrs` with all ADRs accepted → exit 0; with any `proposed` → exit 1 |
| Phase CI | `mise run check-adrs-strict` with all ADRs accepted → exit 0; with any `review-requested` → exit 1; with any `proposed` → exit 1 |

### Unit test cases for `internal/adr/completeness.go`

The completeness parser must have explicit unit tests for:

| Scenario | Expected |
|----------|----------|
| Empty file | score 0, exit 1 |
| File with only template placeholders, no user content | score 0, all caps applied, exit 1 |
| Chosen Solution section missing entirely | score capped at 60, rule fails |
| Chosen Solution present but ≤ 30 words | score capped at 60, rule fails (cap applies to thin sections too) |
| ≥ 3 unfilled `<!-- … -->` | score capped at 70 |
| Risks & Pitfalls placeholder left as `<!-- describe -->` | counts as 1 unfilled placeholder; other rules unaffected |
| Problem Statement > 30 words but > 60% deny-list tokens | boilerplate filter triggers; section treated as failing, score loses 25 pts |
| All sections filled, 0 placeholders | score 100, exit 0 |
| Malformed markdown (no headings) | treated as all sections empty, score 0 |
| `RunCheck` called on a file that causes a panic | `recover()` catches it; warning printed, hook exits 0 |
