# Skill Quality Audit — 2026-03-16

**Branch:** feat/references-section-standard
**Skills evaluated:** 72
**Framework:** skill-judge 9-dimension / 140-point scale
**Script:** `skills/agentic-harness/skill-quality-auditor/scripts/audit-skills.sh`

---

## Grade Distribution

| Grade | Count | % |
|---|---|---|
| A | 10 | 13% |
| B | 9 | 12% |
| C | 26 | 36% |
| D | 24 | 33% |
| F | 3 | 4% |

**Average score: 103/140**

---

## Full Results (sorted by score desc)

| Skill | Score | Grade |
|---|---|---|
| agentic-harness/opencode-toolkit/build-plugins | 129 | A |
| testing/test-driven-development | 129 | A |
| agentic-harness/opencode-toolkit/build-tool | 128 | A |
| agentic-harness/opencode-toolkit/design-commands | 128 | A |
| agentic-harness/skill-quality-auditor | 128 | A |
| documentation/plain-english | 128 | A |
| infrastructure/k8s/debug | 128 | A |
| agentic-harness/opencode-toolkit/configure | 127 | A |
| agentic-harness/opencode-toolkit/design-agents | 126 | A |
| infrastructure/k8s/yaml-validator | 126 | A |
| testing/bdd-testing | 125 | B+ |
| agentic-harness/agents-md | 124 | B+ |
| documentation/acceptance-criteria | 122 | B+ |
| project-mgmt/implementation-planner | 119 | B+ |
| infrastructure/k8s/yaml-generator | 118 | B |
| infrastructure/cfn/behavior-validator | 117 | B |
| documentation/journal-entry-creator | 116 | B |
| infrastructure/cfn/template-compare | 116 | B |
| development/front-end/website-theme-porter | 113 | B |
| development/biome-complete | 110 | C+ |
| repository-mgmt/nx/workspace-patterns | 106 | C+ |
| development/bun-development | 105 | C+ |
| development/front-end/web-reference-sheet-generator | 105 | C+ |
| documentation/markdown-authoring | 105 | C+ |
| repository-mgmt/nx/bun-integration | 105 | C+ |
| specialized/gitlab-api | 105 | C+ |
| testing/ui-debug-workflow | 105 | C+ |
| development/typescript-advanced | 104 | C |
| repository-mgmt/nx/biome-integration | 104 | C |
| infrastructure/aws-cdk/cdk-nag | 103 | C |
| infrastructure/dockerfile/generator | 103 | C |
| development/scripting/makefile/generator | 102 | C |
| repository-mgmt/nx/nx-plugin-authoring | 102 | C |
| ci-cd/azure-pipelines/generator | 101 | C |
| project-mgmt/moscow-prioritization | 100 | C |
| software-engineering/design-principles/clean-architecture | 100 | C |
| software-engineering/design-principles/design-patterns | 100 | C |
| development/scripting/bash-script/validator | 99 | C |
| infrastructure/terraform/generator | 99 | C |
| project-mgmt/create-context-file | 99 | C |
| software-engineering/design-principles/testable-design | 99 | C |
| ci-cd/helm/generator | 98 | C |
| ci-cd/jenkinsfile/generator | 98 | C |
| package-mgmt/mise-complete | 98 | C |
| software-engineering/design-principles/solid-principles | 98 | C |
| ci-cd/fluentbit/generator | 97 | D |
| ci-cd/gitlab-ci/generator | 97 | D |
| infrastructure/ansible/validator | 97 | D |
| infrastructure/terragrunt/generator | 97 | D |
| observability/promql/validator | 97 | D |
| specialized/colyseus-multiplayer | 97 | D |
| development/scripting/bash-script/generator | 96 | D |
| infrastructure/dockerfile/validator | 96 | D |
| repository-mgmt/nx/vite-integration | 96 | D |
| specialized/github-copilot-models | 96 | D |
| ci-cd/fluentbit/validator | 95 | D |
| ci-cd/helm/validator | 95 | D |
| ci-cd/github-actions/validator | 94 | D |
| infrastructure/terraform/validator | 94 | D |
| infrastructure/terragrunt/validator | 94 | D |
| observability/promql/generator | 94 | D |
| agentic-harness/tessl/publish-public | 93 | D |
| ci-cd/github-actions/generator | 93 | D |
| development/commanderjs | 93 | D |
| observability/logql-generator | 93 | D |
| ci-cd/jenkinsfile/validator | 92 | D |
| infrastructure/ansible/generator | 92 | D |
| ci-cd/azure-pipelines/validator | 91 | D |
| ci-cd/gitlab-ci/validator | 91 | D |
| development/scripting/makefile/validator | 90 | F |
| documentation/conventional-commits | 88 | F |
| project-mgmt/implementation-plan-splitter | 0 | F |

---

## Key Findings

### Strengths
- opencode-toolkit cluster (5 skills) all A-grade — strong reference section and anti-pattern coverage
- testing/ domain solid: TDD (129), BDD (125), both above B+ threshold
- k8s cluster performing well: debug (128), yaml-validator (126), yaml-generator (118)

### Weak clusters (systematic D-grade)
- **ci-cd/** — all 12 generator+validator pairs between 91–101; consistent pattern of weak anti-patterns (D3) and poor progressive disclosure (D5)
- **observability/** — all 3 skills between 93–97; logql and promql need anti-patterns and eval coverage
- **infrastructure/terraform + terragrunt** — 4 skills between 94–99; long files (474–631 lines) with no navigation hub pattern
- **infrastructure/ansible** — generator 92, validator 97; both below B threshold

### F-grade issues
- `development/scripting/makefile/validator` (90) — just below D/F boundary; likely missing anti-patterns and eval validation
- `documentation/conventional-commits` (88) — short skill (109 lines) with 0 reference files; likely low D1 and missing D9
- `project-mgmt/implementation-plan-splitter` (0) — **evaluate.sh returned score 0**; skill may have been merged/redirected and no longer has a resolvable SKILL.md at the expected path

### References section compliance (new D4 bonus)
Not surfaced directly in this batch report — individual `evaluate.sh --json` runs include `referenceSectionCompliant` per skill. Recommend running per-skill JSON output to triage which of the D/F skills would immediately gain +1 by fixing their `## References` heading.

### Pre-existing display bug noted
`audit-skills.sh` reports scores as `/120` and labels the framework as "8-dimension" — both incorrect. Scores are computed against `/140` and the framework has 9 dimensions. Fixed in same session.

---

## Recommended Remediation Priority

| Priority | Skills | Action |
|---|---|---|
| 1 — Immediate | `implementation-plan-splitter` | Investigate why score=0; check path |
| 2 — Critical | `conventional-commits`, `makefile/validator` | Add anti-patterns, fix References section |
| 3 — High | All 24 D-grade ci-cd + observability + infra | Add anti-patterns (D3), split long files (D5), fix References heading |
| 4 — Medium | 26 C-grade skills | Improve description keywords (D7), add eval scenarios (D9) |
