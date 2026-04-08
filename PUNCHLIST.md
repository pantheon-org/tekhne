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

CLI fixed (see §4). Post-fix status: **84/87 compliant (97%)**.

### Still missing (3 skills — no audit files at all)

- [ ] `documentation/proof-of-work/skills/proof-of-work`
- [ ] `development/front-end/website-theme-porter`
- [ ] `development/front-end/web-reference-sheet-generator`

### Resolved (84 skills)

**documentation** — triage-tool (127), triage-paper (120), markdown-authoring (104),
plain-english (128), conventional-commits (127), acceptance-criteria (104),
journal-entry-creator (96), astro-starlight/starlight-base (126),
starlight-custom-component (131), starlight-theme (130), obsidian/defuddle (119),
json-canvas (123), obsidian-bases (120), obsidian-cli (123), obsidian-markdown (122)

**agentic-harness** — agents-md A+ (133), skill-quality-auditor A (127),
socratic-method A (128), tessl/publish-public A (127), opencode-toolkit/build-plugins A (129),
build-tool A (128), configure A (128), design-agents A (127), design-commands A (128)

**development** — biome-complete, bun-development, commanderjs, typescript-advanced (103),
scripting/bash-script/generator, bash-script/validator, makefile/generator, makefile/validator

**other** — observability/logql-generator (94), package-mgmt/mise-complete,
specialized/colyseus-multiplayer (97), github-copilot-models (96), gitlab-api (94)

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
