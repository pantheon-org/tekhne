# Worked Example: Should `.context/index.yaml` Be Split?

A real run of this pattern, from this repo's own history
(`.context/findings/index-yaml-split-review-2026-07-06.md`).

## 1. The decision, stated precisely

"Should `.context/index.yaml` be split into per-type files (`.context/plans/index.yaml`,
`.context/findings/index.yaml`, etc.), or stay one file?" — not "what should we do about
the index," which isn't debatable as stated.

## 2. Grounding facts gathered first

Before spawning anyone: current size (86 entries, 548 lines), the actual trigger (three
real `git` merge conflicts on this exact file within one hour, from three unrelated PRs
each adding one entry), and a list of every file in the repo that references
`.context/index.yaml` by exact path (40+, including skill scripts, `SKILL.md` files, and
eval scenarios). These facts went into every reviewer's brief verbatim.

## 3. Roles assigned

- **Advocate**: argue for splitting, propose a concrete design.
- **Skeptic**: argue for the status quo, given a newly-added mitigating rule
  ("regenerate, don't hand-merge, conflicts on auto-generated files") had *just* resolved
  the actual conflicts cheaply.
- **Migration/Risk**: assume the split happens — find everything that would break.

## 4. What each one actually found

- The **Advocate** read the generator script and found the *real* root cause: a shared,
  mutable header (a "last updated" date and an aggregate entry count) rewritten on every
  single regeneration regardless of which section changed — not entry-position shifting,
  which was the original assumption. This was a stronger, more precise argument than "the
  file is big."
- The **Skeptic** pointed out the just-added rule already neutralizes that exact pain for
  near-zero cost, proven three times in the hour that motivated the question — and that
  splitting only narrows the collision domain, it doesn't eliminate the header-churn
  mechanism the Advocate identified.
- **Migration/Risk** found something neither side had considered: an already-diverged,
  CI-unlinked vendored copy of the generator script under `.tessl/plugins/`, which would
  silently keep shipping old behavior if a split wasn't manually re-synced to it — and
  identified that the highest-value gate (`--check` mode reporting "index is fresh")
  would silently stop verifying correctness if the split wasn't done carefully.

## 5. The verdict

Not "it depends" — a decided "don't split now," because: the actual pain was already
solved cheaply by an existing mitigation, the proposed fix only partially addressed the
real root cause the Advocate found, and the Migration/Risk review surfaced concrete new
failure modes with no offsetting benefit large enough to justify them. A concrete revisit
trigger was named: entry count crossing a few hundred, or the existing mitigation proving
insufficient in practice over the following weeks.

## 6. What made this a good run of the pattern

- The Advocate's argument got *better*, not worse, from being grounded — it found a real
  mechanism, not a vague "files should be small" claim.
- The Skeptic didn't just say "no" — it directly engaged with the Advocate's mechanism
  and showed why the existing fix already covered it.
- The Migration/Risk role caught something a pure for/against framing structurally
  cannot: a consequence of the *implementation*, independent of whether the idea is good.
- The synthesis explicitly said which argument won and why, named a concrete revisit
  trigger, and was persisted as a finding so the question doesn't get re-litigated from
  scratch next time it comes up.
