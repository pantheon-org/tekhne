# PUNCHLIST — 2026-04-10

## Recent Changes (last 10 commits)

| SHA | Change |
|-----|--------|
| e4b7531 | chore(evals): migrate scenario-*/ subdirs to scenario-NN.md flat files (#80) |
| b4218d1 | refactor(planning-toolkit): abstract model tiers from provider-specific names (#79) |
| 9de5550 | feat(planning-toolkit): add wave-executor skill and extend wave format with Model column (#78) |
| 319a634 | feat(notebooklm): add scripts and port upstream improvements (#77) |
| ed5459a | feat(research): add notebooklm skill to research tile (#76) |
| e98d594 | feat(research): add scholar-evaluation and scientific-schematics skills (v0.2.5) (#75) |
| 40a1a57 | feat(troubleshooting): require source code location as final investigation step |
| b274cd0 | feat(troubleshooting): add follow-up tickets section to session outcome |
| bd4bf60 | refactor(research): make triage skills domain-agnostic (#63) |
| c2df2d5 | refactor(documentation): rename agentic-context → research (#62) |

---

## 1. STALE AUDIT PATHS (must mv, not delete)

The recent `agentic-context → documentation/research` rename left two audits stranded:

- [x] `mv .context/audits/agentic-context/triage-paper → .context/audits/documentation/research/triage-paper`
- [x] `mv .context/audits/agentic-context/triage-tool → .context/audits/documentation/research/triage-tool`

---

## 2. MISSING AUDITS

CLI fixed (see §4). Post-fix status: **87/87 compliant (100%)**.

### Still missing (resolved — all 3 skills now audited)

- [x] `documentation/proof-of-work/skills/proof-of-work` — A grade (129/140)
- [x] `development/front-end/website-theme-porter` — A grade (128/140)
- [x] `development/front-end/web-reference-sheet-generator` — A grade (126/140)

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
- [x] `development/front-end/web-reference-sheet-generator` — migrated 3 existing scenario-*/ dirs to scenario-NN.md (still only 3/5 canonical files — 2 more evals needed to reach minimum)

### Format ambiguity — resolved

Canonical format: `evals/scenario-NN.md` (one file per scenario). See `skills/agentic-harness/skill-quality-auditor/references/eval-format.md`.

Converted:
- [x] `software-engineering/design-principles` — 12 `.yaml` → 12 `scenario-NN.md`
- [x] `documentation/astro-starlight` sub-skills — 3 `evals.md` → 19 `scenario-NN.md` (6+7+6)
- [x] Retired meta-files (`instructions.json`, `summary*.json`) removed repo-wide (~300 files)
- [x] ~425 `scenario-*/` subdirectory-style evals across 87 skills → 464 `scenario-NN.md` flat files (#80)

---

## 4. AUDIT CLI INVESTIGATION

Fixed. Three bugs in `cli/lib/audit/audit-status.ts`:
1. Path built as `join(skillPath, ".context/audits", skillPath)` — wrong anchor and double `skills/` prefix. Fixed to `join(".context/audits", skillPath.replace(/^skills\//, ""))`.
2. Read `auditData.score` — field doesn't exist (evaluator writes `total`). Fixed.
3. Displayed `/120` — evaluator max is 140. Fixed.

Result after fix: **85/87 compliant (98%)**. Two skills genuinely have no audit files:
- [ ] `development/front-end/website-theme-porter`
- [ ] `development/front-end/web-reference-sheet-generator`

---

## 5. Addtional Skills

Any import should be following the same flow:

1. do a shallow git clone locally (gitignored)
2. determin the domain it should fit in (see @skills)
3. copy the file(s) to the adequate location
4. run a quality audit
5. commit the changes
6. publish to tessl registry
7. push to remote
8. create PR and merge it

- [x] Import <https://github.com/digital-stoic-org/agent-skills/tree/main/dstoic/skills/create-context>
- [x] Import <https://github.com/digital-stoic-org/agent-skills/tree/main/dstoic/skills/pick-model>
- [x] Import <https://github.com/digital-stoic-org/agent-skills/tree/main/dstoic/skills/save-context>
- [x] Import <https://github.com/digital-stoic-org/agent-skills/tree/main/dstoic/skills/load-context>
- [x] Investigate if there is any more skills that would fit our current project in <https://github.com/digital-stoic-org/agent-skills/tree/main>

### Additional high-value candidates also imported (PR #82)

- [x] `agentic-harness/pin` — B 112/140
- [x] `software-engineering/frame-problem` — B 113/140
- [x] `software-engineering/bridge` — B 116/140
- [x] `software-engineering/probe` — B 115/140
- [x] `software-engineering/challenge` — B 112/140
- [x] `software-engineering/troubleshoot` — B 113/140
- [x] `specialized/retrospect-collab` — B+ 122/140
- [x] `specialized/retrospect-domain` — B+ 122/140

---

## Priority Order

1. ~~Fix stale audit mv~~ — done
2. ~~Investigate audit CLI path bug~~ — fixed (84/87 compliant)
3. ~~Run missing audits for 2 remaining skills~~ — done (87/87, 100%)
4. ~~Fill evals — migrate old scenario-*/ dirs~~ — done (#80, 464 files across 87 skills). Remaining: `web-reference-sheet-generator` needs 2 more evals to reach 5/5.
5. ~~Resolve evals format ambiguity~~ — done (`evals/scenario-NN.md` canonical, see eval-format.md)
6. ~~Import digital-stoic-org skills~~ — done (#82, 12 skills, all >= B grade, published to tessl registry)
