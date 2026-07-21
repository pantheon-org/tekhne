# C1: authoritative Python and glue inventory

**Status**: Complete (21-07-2026). Source of truth for C2, C2b, and C3.
**Task**: C1 (Wave 1) of the monorepo tools distribution migration.
**Allowlist file**: `python-allowlist.txt` (repo root).

## 1. Python by skill, with disposition

Every skill directory containing `.py`, classified per decisions D1 (research) and D5 (validators/generators). Disposition drives C2b (convert or keep) and C3 (delete or preserve).

| Skill | Category | Disposition |
|-------|----------|-------------|
| documentation/research/pubmed-search | Research (D1) | Keep (allowlist); marker in C1 |
| documentation/research/sci-hub-search | Research (D1) | Keep (allowlist); marker in C1 |
| documentation/research/notebooklm | Research (D1) | Keep (allowlist); marker in C1 |
| documentation/research/sci-data-extractor | Research (D1) | Keep (allowlist); marker in C1 |
| documentation/research/google-scholar-search | Research (D1) | Keep (allowlist); marker in C1 |
| documentation/research/semantic-scholar-search | Research (D1) | Keep (allowlist); marker in C1 |
| observability/promql/validator | Custom-logic validator (D5 ii) | Keep (allowlist); marker in C2b |
| ci-cd/gitlab-ci/validator | Custom-logic validator (D5 ii) | Keep (allowlist); marker in C2b |
| ci-cd/azure-pipelines/validator | Custom-logic validator (D5 ii) | Keep (allowlist); marker in C2b |
| ci-cd/jenkinsfile/generator | Custom-logic generator (D5 ii) | Keep (allowlist); marker in C2b |
| ci-cd/fluentbit/generator | Custom-logic generator (D5 ii) | Keep (allowlist); marker in C2b |
| ci-cd/fluentbit/validator | External-linter wrapper (D5 i) | Convert to bash in C2b; Python deleted in C3 |
| ci-cd/helm/validator | External-linter wrapper (D5 i) | Convert to bash in C2b; Python deleted in C3 |
| infrastructure/ansible/validator | External-linter wrapper (D5 i) | Convert to bash in C2b; Python deleted in C3 |
| infrastructure/k8s/debug | External-linter wrapper (D5 i, shells kubectl) | Convert to bash in C2b; Python deleted in C3 |
| infrastructure/k8s/yaml-validator | External-linter wrapper (D5 i) | Convert to bash in C2b; Python deleted in C3 |
| infrastructure/terraform/validator | External-linter wrapper (D5 i) | Convert to bash in C2b; Python deleted in C3 |
| infrastructure/terragrunt/validator | External-linter wrapper (D5 i) | Convert to bash in C2b; Python deleted in C3 |

Note: fluentbit has both a `generator` (kept) and a `validator` (converted); the paths disambiguate them.

## 2. requirements.txt (8 files)

Tracked skill files:
- skills/documentation/research/pubmed-search/requirements.txt
- skills/documentation/research/sci-hub-search/requirements.txt
- skills/documentation/research/notebooklm/requirements.txt
- skills/documentation/research/sci-data-extractor/requirements.txt
- skills/documentation/research/google-scholar-search/requirements.txt
- skills/documentation/research/semantic-scholar-search/requirements.txt

Vendored copies (carve out of C3 verification; not deleted, not double-counted):
- .agents/skills/pubmed-search/requirements.txt
- .agents/skills/sci-hub-search/requirements.txt

All six tracked `requirements.txt` belong to allowlisted research skills, so all are retained by C3. Category-(ii) validators/generators carry `.py` but no `requirements.txt` (stdlib only).

## 3. Inline python3 (not .py files)

- hk.pkl:97 — auditor grade JSON parse.
- .github/workflows/skill-audit.yml:70-71 — auditor grade + total JSON parse.

Disposition: C2 replaces both with `jq` and pins `jq` in mise.toml (behaviour-preserving).

## 4. CI / dependency surfaces

- .github/dependabot.yml:93 — `package-ecosystem: "pip"` block (globs the research skills). Disposition: C3 narrows to the allowlist (do not delete; retained Python keeps dependency tracking).
- .github/workflows/codeql.yml:48 — `language: python` matrix entry. Disposition: C3 narrows to the allowlist; A6c handles the `main` code-scanning ruleset change in lockstep (PR #126).

## 5. Vendored .agents copies

`.agents/skills/{pubmed-search,sci-hub-search}/` contain their own `.py` + `requirements.txt` (installed copies of allowlisted research skills). They are gitignored (`.agents/` in .gitignore) but present in the working tree. C3 verification ("zero non-allowlisted `.py`") must carve out `.agents/skills/**`.

## 6. Allowlist summary (11 skills)

- Research (D1): 6 skills, markers applied in C1.
- Custom-logic validators/generators (D5 ii): 5 skills (promql, gitlab-ci, azure-pipelines validators; jenkinsfile, fluentbit generators), markers applied in C2b.
- External-linter wrappers (D5 i): 7 skills converted to bash in C2b, Python deleted in C3, NOT on the allowlist.

See `python-allowlist.txt` for the machine-readable list.
