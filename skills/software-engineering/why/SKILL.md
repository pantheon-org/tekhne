---
name: why
description:
  "Use for 'why does X work this way', 'why we picked Y', design rationale, regressions, postmortems, or data-backed thresholds. Discovers available MCPs and queries each evidence category (source
  control, issue tracker, long-form docs, real-time chat, infrastructure observability, error tracking, product analytics warehouse) in parallel, then returns a cited read on decisions and tradeoffs.
  DO NOT use for explaining current runtime behavior (use how instead), or for proving a change is safe to ship (use blast-radius instead). Triggers: why does X work this way, why did we pick Y over
  Z, what motivated this, why does this still exist, history of X."
---

# Why

> Ported from the `why` skill in [cursor/plugins pstack](https://github.com/cursor/plugins/tree/main/pstack/skills) — Cursor, a separate AI-coding-editor product, not to be confused with a text/DB
> cursor — for use with this harness's `Agent` tool. Agent-spawning mechanics use this repo's actual subagent types and model/effort params in place of the original's source-specific model names and
> `readonly` flag; MCP discovery is adapted to how this session surfaces connected servers.
>
> **Git-history steps prefer a code-graph MCP first, when one is connected.** If your environment has one (e.g. `tokensave`), prefer its graph tools over raw shell git commands for codebase research
> wherever an equivalent exists — a per-symbol blame/log tool gives structural history tracked across renames and is the first move for any target that's a function, method, class, or type. Raw
> `git blame`/`git log --follow -p` are the fallback for what a code-graph tool doesn't cover (or for when none is connected): line-precise blame that doesn't align to a symbol boundary, full raw
> patch text, or file kinds outside the tool's indexed languages. A semantic added/removed/modified-symbol diff tool, if available, is cheaper to read than a raw patch before deciding which commits
> are worth opening in full. See Step 2 for the full mapping. If such a tool reports a stale index for the current worktree, fall back to raw git for that worktree's own uncommitted or unindexed
> changes.
>
> **Map your own MCP stack before spawning investigators — don't assume any of these are connected.** Source-control-hosting MCPs (GitLab, GitHub) commonly double as the issue-tracker category too;
> issue-tracker/long-form-docs MCPs (e.g. Jira/Confluence) commonly split into two investigators, one per product; chat MCPs (e.g. Slack) cover real-time chat. Infrastructure observability, error
> tracking, and product-analytics-warehouse MCPs (Datadog, Sentry, a data warehouse) are frequently absent — when they are, their investigators should come back "skipped, no MCP available," which is
> an honest gap, not a failure to search harder. If the repo has no remote, source-control investigation is local git (or code-graph) history only — skip straight to that rather than treating the
> absence of `gh pr view`/remote MR tooling as a search failure.

Investigate the motivation and intent behind code. Why was it built this way? What edge cases were considered? What product, business, or operational constraints shaped the design? What alternatives
were rejected, and why?

Companion to the `how` skill. `how` answers what the code does and how it works. `why` answers what forces led to its shape.

## Prerequisites

- Git is always available for source control history; beyond that, whatever MCP servers are actually connected this session determine which of the other six evidence categories can be investigated
- A specific code target, pattern, or named design decision to anchor the investigation — a vague "why is this codebase the way it is" has no anchor to build a code anchor from
- Willingness to report an honest "we don't know" — this skill's output is only as good as its willingness to say a category came up empty

## When to Use

- "Why does X work this way", "why did we pick Y over Z", "what motivated this" — design rationale and tradeoff questions.
- "Why does this still exist" — dead-code or legacy-behavior archaeology.
- Regressions and postmortems: what edge case, incident, or constraint originally shaped the current shape of the code.
- Data-backed thresholds: where a specific constant or limit actually came from.

## When NOT to Use

- The question is about what the code currently does or how it works ("how does X work") — use `how` instead.
- The question is about whether a change is safe to ship ("what could this break") — use `blast-radius` instead.
- There's no concrete code target, pattern, or named decision to anchor the investigation in.

## Verification

Before returning the synthesizer's output, run the same check the synthesizer's own "Quality Check Before Returning" calls for: spot-check at least one citation in "What We Found" by re-opening the
source it points to, and confirm it actually says what the claim says it says. Run this the same way you'd run a test before shipping — don't just trust that the investigator quoted accurately.

## Mindset

- Evidence before narrative — collect what's actually there before deciding what story it supports, never the reverse
- An honest "we searched X, Y, Z and found nothing" is a complete, valuable answer, not a failed investigation
- The user's own guess at "why" is a hypothesis to check, not a conclusion to agree with
- Code is mechanics; motivation always comes from an external source, or it's inference and must be labeled as such

## How this skill works

Historical context spreads across seven evidence categories: source control history, issue or ticket tracking, long-form documents, real-time team chat, infrastructure observability, error or
exception tracking, and product analytics warehouses. You cannot predict from the question alone which one holds the answer, so the skill enumerates available MCPs at run time, maps each to a
category, queries all seven in parallel, then synthesizes with explicit confidence calibration. Null results from searched categories are first-class evidence about how the decision was made; report
them alongside positive findings. The default is coverage, not minimalism.

## Operating Posture

Operate as a careful, cautious, precise investigator. Think like a detective piecing together a historical case from fragmentary records. When the record is thin, say so.

Concretely:

- **Evidence before narrative.** Collect the pieces first, then see what story they support. Never pick a story and recruit the evidence that fits it.
- **Precision over polish.** Prefer the exact quote and citation over a smooth paraphrase. A reader should be able to follow any claim back to its source and verify it in under a minute.
- **Consider what you haven't seen.** The evidence you find is a sample, not the whole truth. Before concluding, ask what you would expect to see if an alternative explanation were true, and whether
  you looked for it.
- **Name the gaps.** If a thread goes cold, a source isn't searchable, or a question has no answer, document the gap. Don't paper it over with an authoritative-sounding guess.
- **Hedge on purpose.** When evidence is indirect, your language should signal it ("appears to", "likely", "suggests"). Confidence-matching phrasing is a feature of the output, not a stylistic choice
  the synthesizer may override.
- **No shortcut by code-reading.** The code tells you what it does, rarely why it exists. Resist inferring intent from code shape.

This posture is the working method, not a disclaimer.

## Core Epistemics

This skill builds a **patchwork understanding** from fragmented historical evidence. Tickets go stale. Chat threads get deleted. Commit messages lie. People change their minds between the PR
description and the implementation. The original author may have left the company.

Be ruthlessly honest about what you know versus what you're inferring. The goal is not a satisfying story; it is to surface evidence, calibrate confidence, and let the user decide.

Principles:

- **Cite everything.** Every claim about intent should reference a specific commit hash, PR/MR number, ticket ID, doc URL, chat permalink, or code comment. If you can't cite it, it's inference, not
  fact, and must be labeled as such.
- **Prefer "appears to" over "because".** Hedge when evidence is indirect. Reserve confident language for direct, explicit evidence.
- **Surface contradictions.** If two sources disagree, show both. Don't quietly pick the one that fits your narrative.
- **Acknowledge gaps.** If a question has no answer in any source you searched, say so. An honest "we couldn't find out why" beats a confident guess.
- **Multiple hypotheses are valid.** When the evidence fits several stories, present them all with the evidence for each. Let the user triangulate.
- **Beware rationalization.** Code that makes sense today may have been written for reasons that no longer apply, or for no good reason at all. Don't retrofit intent.

Read `references/epistemics.md` for the full confidence framework and phrasing guide. The synthesizer must follow it.

## Step 1. Understand the Target and the Question

Parse what the user is asking. The **target** is usually a chunk of code, a pattern, a feature, or a named design decision. The **question** is usually one of:

- "Why was X designed this way?" Design rationale.
- "Why do we do X instead of Y?" Tradeoff or alternatives.
- "What edge cases motivated this?" Defensive reasoning.
- "What business or product constraint led to this?" External forcing function.
- "Why does this code still exist?" Dead-code territory.
- "What's the history of X?" Broad archaeological sweep.

If the target is vague ("why do we do it this way?" with no clear referent), make your best guess from conversation context (open files, recent edits, what was just discussed). State your
interpretation briefly so the user can redirect if you're off, then proceed.

## Step 2. Establish the Code Anchor

Before spawning investigators, anchor the investigation in concrete code. You need:

- The relevant file path(s) and line range(s)
- The key symbols (function names, class names, constants)
- An initial commit list. The last few commits touching the target.
- PR/MR numbers from merge commits (pattern `(#1234)` or `(!1234)` in the subject line)

Build this inline. It's cheap, and every investigator needs it.

**First choice — `tokensave`, when the target resolves to a symbol** (a function, method, class, type, or const with a name, not an arbitrary line):

```text
tokensave_blame(symbol, file?)          # most recent commit that structurally changed this symbol
tokensave_log(symbol, file?, limit)     # every commit that structurally changed it, oldest-first, across renames
tokensave_diff(from?, to?, path?)       # semantic added/removed/modified summary for a ref range or the working tree
tokensave_changelog(from_ref, to_ref)   # same idea, explicit ref-to-ref
```

These are cheaper to read than raw patches (they name the symbols that changed, not line-by-line text) and — unlike `git log --follow` — they track the symbol itself across file moves and renames via
structural fingerprints, not just the file path. Start here; only drop to raw git for what these can't give you.

**Fallback — raw git, for what tokensave doesn't cover:**

```bash
# Line-precise blame that doesn't align to one symbol (a line inside a large function, a config block, a comment)
git blame -L <start>,<end> <file>

# Full raw patch text, or a file kind tokensave doesn't index (check tokensave_status's languages_by_file for coverage)
git log --follow -p -- <file>

# Last N commits touching the file, PR/MR numbers visible — a file-level view tokensave's symbol-scoped tools don't give directly
git log --oneline -20 -- <file>

# Extract PR/MR numbers from a commit message once you already have the hash
git log -1 --format=%B <commit>
```

Where a remote exists, pull PR/MR bodies and discussion for any substantive commits — `gh pr view <number> --json title,body,author,createdAt,mergedAt,labels,closingIssuesReferences,comments,reviews`
for GitHub, or the equivalent GitLab MCP/CLI tools for GitLab. This is a distinct concern from the history tools above — it's fetching an _existing_ PR/MR's remote metadata and discussion, not
deriving local structural history, so a local code-graph tool (which only knows the local git graph) doesn't cover it either way. If the repo has no remote (check `git remote -v` first), skip
straight to local history and note the absence of PR/MR discussion as a structural gap, not a search failure.

Capture this as seed context (file paths, symbols, commits, PR/MR numbers, linked ticket IDs). Pass it to the investigators so they don't rediscover it.

## Step 3. Spawn Parallel Investigators (default posture)

**Default to the full parallel investigation.** Each evidence category lives in a different kind of system, and you cannot tell from the question alone which one holds the answer without looking. So
look across every available category, in parallel, by default.

### Discovery

Before spawning investigators, work out which MCP servers are actually connected in this session — they're named in the system-reminder tool listings (`mcp__<server>__*` tool names) and in any
deferred-tools notice. Use `ToolSearch` to confirm a server's tools if you're unsure what it can do.

Map each connected MCP to one evidence category:

1. Source control history
2. Issue / ticket tracker
3. Long-form documents
4. Real-time team chat
5. Infrastructure observability
6. Error / exception tracking
7. Product analytics warehouse

Source control is always available through git — via `tokensave`'s per-symbol history tools first, raw git as fallback (see Step 2) — plus `gh`/`mcp-gitlab` when a remote exists. For the other six,
classify using the MCP's server name, its instructions block, and its tool names. If an MCP could fit more than one category (e.g. `mcp-atlassian` covers both issue tracker via Jira and long-form docs
via Confluence — treat it as two investigators, one per Atlassian product), split it into separate investigators rather than merging categories. Record ambiguous or absent cases in the coverage map.

Aim for a complete **coverage map**, not a minimal one. A null result from an issue tracker is evidence the decision was not ticketed, a useful fact in itself. Document the null, don't skip the
search.

Launch all matching investigators in a single `Agent` message so they run concurrently. One investigator per category lets each specialize in one tool's query vocabulary and result shape. Don't ask
one agent to cover multiple MCPs.

Subagent config (each):

- `subagent_type`: `general-purpose` — not because `Explore` lacks MCP access (its tool grant is "everything except `Agent`/`Artifact`/`ExitPlanMode`/`Edit`/`Write`/`NotebookEdit`," so it _can_ call
  MCP tools), but because `Explore` is scoped and prompted for locating code by pattern; reading ticket threads, chat, and docs to compile a cited evidence report is a different task shape and
  `general-purpose` fits it
- `model`: omit to inherit the session model unless a category needs deeper reasoning to parse (e.g. a long incident postmortem)

Investigators must not write anything (no file edits, no comments posted, no tickets updated). That's a posture to hold deliberately, not a sandboxing mechanism — instruct each investigator explicitly
not to mutate state, since `general-purpose` has write-capable tools available.

Each investigator gets:

1. The base prompt from `references/investigator-prompt.md`
2. The category playbook `references/sources/<source>.md` for the selected MCP, adapted from the examples in `references/source-playbook.md`
3. The cross-cutting `references/sources/incident-postmortem.md` **if the target code looks defensive** (null checks, retry logic, timeout handling, rate limiting, feature flags, egress guards, OOM
   handlers)
4. The code anchor from Step 2 (file paths, symbols, commit hashes, PR/MR numbers, ticket IDs)
5. The user's original question

### Investigator roster. One per available evidence category

Spawn one investigator per category that has a matching MCP. Each owns exactly one tool or MCP.

Each entry lists what the category physically contains and the kind of "why" it uniquely surfaces. Use it to know what to expect back, how to name a gap when a category returns empty, and (only in the
rare provably-irrelevant case) to justify a skip. Every category overlaps, but each owns a kind of evidence the others cannot recover.

1. **Source control investigator**. Git history via `tokensave_blame`/`tokensave_log`/`tokensave_diff` first (raw git as fallback, per Step 2), `gh`/`mcp-gitlab` for PRs/MRs, code comments, tests.
   Always spawn; the only guaranteed source. Best at surfacing _implementation-time rationale captured during review_. PR/MR descriptions stating the problem, review threads debating alternatives,
   inline comments encoding non-obvious constraints, test names that encode motivating edge cases, and commit messages linking tickets or incidents. Most trustworthy because it ties directly to the
   diff that shipped.

2. **Issue / ticket tracker investigator** (Jira, GitLab Issues, Linear, or equivalent; adapt the `linear.md` playbook to whichever is connected). Tickets, project docs, status updates, spec
   attachments. Best at surfacing _the product or business forcing function_. Customer requests, compliance deadlines, parent-initiative framing, ticket-level scope changes, and labels that
   categorize the motivation. Strongest when the why is external to engineering. In a regulated domain, treat regulator-driven tickets (a compliance mandate, an audit finding, a data-protection
   requirement) as their own signal worth calling out explicitly.

3. **Long-form documents investigator** (Confluence via `mcp-atlassian`; adapt the `notion.md` playbook). PRDs, specs, RFCs, design docs, ADRs, postmortems, team pages, meeting notes. Best at
   surfacing _long-form design rationale_ written out before it becomes code.

4. **Real-time team chat investigator** (Slack). Feature-name and symbol searches, PR/MR URL mentions, incident channels, author-handle activity around the ship date. Best at surfacing _real-time
   deliberation that never reached a doc_.

5. **Infrastructure observability investigator** (Datadog, New Relic, CloudWatch, Grafana, Splunk MCP — none of these are connected in this session by default; check before assuming). Best at
   surfacing _infrastructure and runtime reality that motivated the code_: monitor thresholds matching code constants, metric spikes before a merge, incident timelines.

6. **Error / exception tracking investigator** (Sentry, Rollbar, Bugsnag — none connected by default). Best at surfacing _the specific exceptions and error trajectories that motivated defensive or
   corrective code_.

7. **Product analytics warehouse investigator** (Databricks, Snowflake, BigQuery — none connected by default). Best at surfacing _product and data reality that shaped the code_, e.g. where a threshold
   constant came from.

### When to skip an investigator

Only skip with an **explicit, written justification** that goes in the final "Sources Consulted" section. Two valid reasons:

- **No MCP is available for that category** in this environment. Flag this as a gap, not a choice. Example: "Infrastructure observability skipped. No Datadog/CloudWatch-equivalent MCP connected this
  session, so runtime telemetry was not searchable."
- **The source is provably irrelevant**, not just "probably irrelevant." A high bar. Example: "Error / exception tracking skipped. Target is a build-time script with no runtime code path." Not
  "probably not in error tracking, it's a feature not an error."

"It's pure feature code, error tracking won't have anything" is **not** sufficient, and neither is "I doubt long-form docs would have this." Run the search; let the null result speak. The cost of an
investigator returning empty is one subagent. The cost of missing a design doc that actually exists is a wrong answer.

If your scope assessment suggests a single-commit trivial target where the PR/MR description already contains the complete answer, you may answer inline **only after** confirming all available
category searches would be redundant. Say so explicitly. This should be rare.

## Step 4. Synthesize

Spawn one synthesizer subagent:

- `subagent_type`: `general-purpose` (its quality check spot-verifies citations, which can require MCP or file access)
- `model`: consider escalating (e.g. `opus`, effort `high`) for a contested or high-stakes "why" — the calibration work in Step 4 rewards deeper reasoning

The synthesizer gets:

1. The investigator findings, including any null results and any categories skipped with justification
2. The code anchor from Step 2 (file paths, symbols, commit hashes, PR/MR numbers, ticket IDs)
3. The user's original question
4. The epistemics framework from `references/epistemics.md`
5. The synthesizer prompt template from `references/synthesizer-prompt.md`

Its job is the final output: a confidence-weighted, evidence-cited narrative with clearly separated "what we know" and "what we're inferring" sections, plus honest acknowledgment of gaps and
null-result sources.

## Step 5. Present

Take the synthesizer's output and present it to the user. You may lightly edit for clarity or add context from the conversation, but **do not rewrite the confidence language**. The epistemic framing
is the product. Dropping the hedges to sound more authoritative is the exact failure mode this skill exists to prevent.

## Output Format

The final output uses this structure. Adapt as needed, but keep the confidence separation intact.

**The Question**. Restate what the user asked, concisely.

**The Code in Question**. File paths, line ranges, and key symbols. One or two lines so the reader is anchored.

**What We Found (direct evidence)**. Claims with explicit citations (PR/MR #, ticket ID, doc URL, chat permalink, commit hash, code comment with file:line). Each bullet is a thing we have textual
evidence for. Use present tense and quote or paraphrase the source.

**What We Can Reasonably Infer**. Claims well-supported by indirect evidence or combinations of signals, but not explicitly stated anywhere. Each bullet must explain the inference chain: "Given A and
B, it's likely that C." Use hedged language ("appears to", "likely", "suggests").

**Competing Hypotheses**. If the evidence fits multiple stories, list them. For each, give the hypothesis, the evidence for it, and the evidence against it. Don't force a winner when the record
doesn't support one. (Skip this section if there's a clear answer.)

**What We Don't Know**. Explicit gaps. Questions the user asked that the evidence didn't answer. Sources we searched and came up empty. Be specific. "We searched the issue tracker for 'rate limit' and
found no ticket discussing this specific threshold" is more useful than "we don't know why."

**Sources Consulted**. One line per investigator, including the ones that returned nothing. The reader should see at a glance (a) which MCPs were queried, (b) which came back empty, and (c) which were
skipped and why. This coverage map lets the user judge breadth and redirect if something obvious was missed.

Format each line as: `- <Source>: <what was searched>. <what was found, or "no relevant results," or "skipped. reason">.`

Example:

- Source control (git/tokensave): `tokensave_log('processRetry', file='backend/retry.ts')` for structural history, `git log --follow backend/retry.ts` for surrounding file context. Found the commit
  introducing exponential backoff, message references incident tracking issue #4421.
- Issue tracker (Jira): searched for "retry" and the linked ticket key. Found the ticket but no discussion of backoff parameters.
- Long-form docs (Confluence): searched for "retry policy," "backend retries." No relevant results.
- Real-time team chat (Slack): searched `#eng-backend` around the merge date. Found a thread where the author explained the choice of 3 retries.
- Infrastructure observability: skipped. No Datadog/CloudWatch-equivalent MCP connected this session. Gap: runtime telemetry not searched.
- Error / exception tracking: skipped. No Sentry-equivalent MCP connected this session. Gap: error trajectory not searched.
- Product analytics warehouse: skipped. No warehouse MCP connected this session. Gap: usage/threshold-origin data not searched.

After the Sources Consulted block, if the user's `why` question is a precursor to actually changing this code, convert the lineage findings into a Preserve / Change / Avoid / Risk constraint set
suitable for planning the change.

## Troubleshooting

- **No MCP is connected for a category:** report it in "Sources Consulted" as skipped with the reason "no MCP available," never silently drop the category from the coverage map.
- **Two sources give contradictory answers:** present both with their citations in the output rather than picking the one that fits a tidier narrative — see "When Evidence Contradicts" in
  `references/epistemics.md`.
- **The user's question embeds a hypothesis ("I assume it's for performance?"):** treat it as one candidate to check against the evidence, not a conclusion to confirm — see "The Sycophancy Trap" in
  `references/epistemics.md`.
- **Every category comes back empty:** say so plainly in "What We Don't Know" rather than padding the answer with confident-sounding speculation to avoid an unsatisfying result.
- **The target is a single trivial commit with an obviously complete PR/MR description:** you may answer inline without the full investigator fan-out, but only after explicitly confirming every
  available category search would be redundant — this should be rare.

## Common Failure Modes to Avoid

**NEVER** write a claim into "What We Found" without a citation, no matter how plausible it sounds.

**WHY:** a confident-sounding narrative built from thin evidence is the exact failure mode this skill exists to prevent. An uncited bullet belongs in "What We Can Reasonably Infer" or "Competing
Hypotheses," not "What We Found."

**NEVER** cite the code itself as evidence for why it exists.

**WHY:** "handles the null case because it checks for null" is mechanics, not motivation. Motivation comes only from an external source — PR/MR discussion, ticket, comment, conversation — anything
else is inference and must be labeled as such.

**NEVER** assume the most recent commit is the authoritative reason for the current shape.

**WHY:** the current shape is often the accretion of many earlier decisions. Trace back before concluding the last commit is "the" answer — recency is not the same as authority.

**NEVER** treat a hypothesis embedded in the user's own question as already confirmed.

**WHY:** if the user suggests a reason ("I assume this is for performance?"), it is a candidate to check against the evidence independently, not a conclusion to rubber-stamp. Sycophantic agreement
defeats the point of an investigation.

**NEVER** omit the "What We Don't Know" section, even when the investigation felt thorough.

**WHY:** an honest accounting of what you couldn't find out is part of the value this skill delivers. Skipping it hides exactly the uncertainty the user needs to see.

**NEVER** skip an investigator because you assume its category won't have anything.

**WHY:** deciding up front that "long-form docs probably don't have this" without searching defeats the default-to-all posture. A null result is a data point; a skipped search is a blind spot.

**NEVER** pool multiple evidence categories into a single investigator agent.

**WHY:** each MCP has its own query vocabulary, result shape, and pitfalls. Pooling them dilutes specialization and makes coverage harder to reason about — always spawn one investigator per category.

## Reference Files

- `references/epistemics.md`. Confidence tiers and phrasing guide. The synthesizer must follow it.
- `references/investigator-prompt.md`. Base prompt template for investigator subagents.
- `references/source-playbook.md`. Index pointing at the category playbooks below.
- `references/sources/*.md`. One self-contained example playbook per category, plus cross-cutting `incident-postmortem.md`. Give an investigator the single file that matches its category and adapt it
  to the available MCP (e.g. adapt `linear.md` for Jira, `notion.md` for Confluence).
- `references/synthesizer-prompt.md`. Prompt template for the synthesizer subagent, including the output format.
