# PUNCHLIST — 2026-04-08

## Recent Changes (last 10 commits)

| SHA | Change |
|-----|--------|
| 40a1a57 | feat(troubleshooting): require source code location as final investigation step |
| b274cd0 | feat(troubleshooting): add follow-up tickets section to session outcome |
| bd4bf60 | refactor(research): make triage skills domain-agnostic (#63) |
| c2df2d5 | refactor(documentation): rename agentic-context → research (#62) |
| 9cf34ef | refactor(agentic-context): move to documentation domain (#61) |
| c883f08 | feat(agentic-context): remediate triage-paper/triage-tool to A grade (#59) |
| c3e1d25 | feat(documentation): add proof-of-work skill (A grade, 129/140) (#56) |
| 68b5b6b | feat(project-mgmt): consolidate planning skills into planning-toolkit (#55) |
| b5bf4bc | feat(infrastructure): move aws-investigation-toolkit under infrastructure/aws (#54) |
| 0b3d1ae | chore(audits): migrate stale audit records to current skill paths (#53) |

---

## 1. STALE AUDIT PATHS (must mv, not delete)

The recent `agentic-context → documentation/research` rename left two audits stranded:

- [x] `mv .context/audits/agentic-context/triage-paper → .context/audits/documentation/research/triage-paper`
- [x] `mv .context/audits/agentic-context/triage-tool → .context/audits/documentation/research/triage-tool`

---

## 2. MISSING AUDITS

The `bun cli/index.ts audit status` reports no valid audit for the following skills.
Note: audit files exist on disk for `agentic-harness/*` and `development/*` — the CLI may have a path-resolution mismatch with the new skill structures. Verify before blindly re-running.

### documentation (16 skills)
- [x] `documentation/research/triage-tool` ← audit moved, CLI should now resolve
- [x] `documentation/research/triage-paper` ← audit moved, CLI should now resolve
- [ ] `documentation/markdown-authoring`
- [ ] `documentation/plain-english`
- [ ] `documentation/proof-of-work/skills/proof-of-work`
- [ ] `documentation/astro-starlight/skills/starlight-base`
- [ ] `documentation/astro-starlight/skills/starlight-custom-component`
- [ ] `documentation/astro-starlight/skills/starlight-theme`
- [ ] `documentation/journal-entry-creator`
- [ ] `documentation/acceptance-criteria`
- [x] `documentation/conventional-commits` — A grade (127/140)
- [ ] `documentation/obsidian/defuddle`
- [ ] `documentation/obsidian/json-canvas`
- [ ] `documentation/obsidian/obsidian-bases`
- [ ] `documentation/obsidian/obsidian-cli`
- [ ] `documentation/obsidian/obsidian-markdown`

### agentic-harness (9 skills)
> Audit files exist at `.context/audits/agentic-harness/*` — CLI may not be resolving them. Investigate before re-auditing.
- [x] `agentic-harness/agents-md` — A+ grade (133/140)
- [x] `agentic-harness/skill-quality-auditor` — A grade (127/140)
- [x] `agentic-harness/socratic-method` — A grade (128/140)
- [x] `agentic-harness/tessl/publish-public` — A grade (127/140)
- [x] `agentic-harness/opencode-toolkit/build-plugins` — A grade (129/140)
- [x] `agentic-harness/opencode-toolkit/build-tool` — A grade (128/140)
- [x] `agentic-harness/opencode-toolkit/configure` — A grade (128/140)
- [x] `agentic-harness/opencode-toolkit/design-agents` — A grade (127/140)
- [x] `agentic-harness/opencode-toolkit/design-commands` — A grade (128/140)

### development (10 skills)
> Audit files exist at `.context/audits/development/*` — same CLI resolution concern.
- [ ] `development/biome-complete`
- [ ] `development/bun-development`
- [ ] `development/commanderjs`
- [ ] `development/typescript-advanced`
- [ ] `development/front-end/web-reference-sheet-generator`
- [ ] `development/front-end/website-theme-porter`
- [ ] `development/scripting/bash-script/generator`
- [ ] `development/scripting/bash-script/validator`
- [ ] `development/scripting/makefile/generator`
- [ ] `development/scripting/makefile/validator`

### other domains (5 skills)
- [ ] `observability/logql-generator`
- [ ] `package-mgmt/mise-complete`
- [ ] `specialized/colyseus-multiplayer`
- [ ] `specialized/github-copilot-models`
- [ ] `specialized/gitlab-api`

---

## 3. MISSING EVALS

Almost all `evals/` directories are empty. Only 3 files exist across the whole repo (in `development/front-end/web-reference-sheet-generator`). The `astro-starlight` skills have a top-level `evals.md` file rather than individual files in `evals/` — these may or may not be counted.

**Every skill needs ≥5 eval scenarios** (see `feat(evals)` commit d70ce65).

### Skills with 0 eval files (sample — nearly all skills)

**agentic-harness** (all 9 above)
**ci-cd** (all 12 generator/validator pairs)
**development** (all 10 above, except web-reference-sheet-generator has 3/5)
**documentation** (all 16 above; astro-starlight 3 have evals.md but 0 files in evals/)
**infrastructure** (ansible, aws-cdk/cdk-nag, aws/investigation-toolkit, cfn, dockerfile, k8s, terraform, terragrunt)
**observability/logql-generator**
**package-mgmt/mise-complete**
**project-mgmt** (create-context-file, moscow-prioritization, planning-toolkit)
**repository-mgmt** (nx, nx/biome-integration, nx/bun-integration, nx/vite-integration, nx/workspace-patterns)
**software-engineering/design-principles** (clean-architecture, design-patterns, solid-principles, testable-design)
**specialized** (all 3)
**testing** (bdd-testing, test-driven-development, ui-debug-workflow)

### Partial (needs top-up)
- [ ] `development/front-end/web-reference-sheet-generator` — 3/5 files

### Format ambiguity
- [ ] Decide: are `evals.md` (astro-starlight style) acceptable, or should all skills use `evals/*.md`?

---

## 4. AUDIT CLI INVESTIGATION

Fixed. Three bugs in `cli/lib/audit/audit-status.ts`:
1. Path built as `join(skillPath, ".context/audits", skillPath)` — wrong anchor and double `skills/` prefix. Fixed to `join(".context/audits", skillPath.replace(/^skills\//, ""))`.
2. Read `auditData.score` — field doesn't exist (evaluator writes `total`). Fixed.
3. Displayed `/120` — evaluator max is 140. Fixed.

Result after fix: **84/87 compliant (97%)**. Three skills genuinely have no audit files:
- [ ] `documentation/proof-of-work/skills/proof-of-work`
- [ ] `development/front-end/website-theme-porter`
- [ ] `development/front-end/web-reference-sheet-generator`

---

## Priority Order

1. ~~Fix stale audit mv~~ — done
2. ~~Investigate audit CLI path bug~~ — fixed (84/87 compliant)
3. Run missing audits for 3 remaining skills
4. Fill evals to 5 per skill (largest volume of work)
5. Resolve evals format ambiguity for astro-starlight skills
