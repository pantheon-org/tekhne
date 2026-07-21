# R1: release-please x cargo-dist versioning policy

**Status**: Policy decided; mechanical extra-files repoint applied; drift + stale-entry reconciliation flagged for decision (21-07-2026).
**Task**: R1 (Wave 1) of the monorepo tools distribution migration. Consumed by A5, A6, A7, A8.

## 1. The two release systems

The monorepo has two independent release mechanisms that must not collide:

- **release-please** owns **plain skills**. Per-skill semver lives in `.release-please-manifest.json`; release-please opens release PRs, bumps the manifest, tags, and syncs the version into each skill's `.tessl-plugin/plugin.json` via `extra-files`. Config: `release-please-config.json` (61 packages), workflow `release-please.yml`.
- **cargo-dist** owns **tool crates** (skill-auditor, adr, journal, and any D5 Rust ports). Per-tool semver lives in the crate `Cargo.toml`; releases are cut on per-tool git tags and build binaries via the generated workspace `release.yml`. Config: `dist-workspace.toml`.

## 2. Coexistence policy (decisions)

1. **Disjoint tag namespaces.** release-please uses its component tags for skills. cargo-dist uses `<tool>-v<semver>` tags (e.g. `skill-auditor-v1.2.3`). The `rust-release.yml` prototype (A1) triggers only on `*-v[0-9]+.[0-9]+.[0-9]+`, so skill release tags never trigger tool releases and vice versa.
2. **Version source of truth by package type.** Skills: the release-please manifest is authoritative; `plugin.json` `$.version` is a **derived mirror** synced by extra-files. Tool crates: `Cargo.toml` is authoritative; cargo-dist reads it. Neither system writes the other's source of truth.
3. **Disjoint workflow triggers.** `release-please.yml` runs on push to `main`. The generated cargo-dist `release.yml` runs on tag push. No shared trigger, no race.
4. **Relocated / retired tool-backed skills exit release-please cleanly.** When a skill becomes a tool crate (e.g. `journal` in A8) or is relocated/consolidated, its release-please package entry AND its `.release-please-manifest.json` entry are removed in the same change, and any husk directory (leftover `CHANGELOG.md`) is deleted. Its versioning then lives only in `Cargo.toml` + cargo-dist tags (for tool crates) or under its new path's entry (for relocations). This is the process A5/A6/A7/A8 follow.

## 3. D4 fix: extra-files repoint (applied)

**Done in this task.** All 61 `extra-files` entries in `release-please-config.json` were repointed from the removed `tile.json` to `.tessl-plugin/plugin.json` (same `$.version` jsonpath). Background: PR #93 migrated all skills from `tile.json` to `.tessl-plugin/plugin.json` but left the release-please `extra-files` pointing at the old path, so the version sync has been broken since. Repointing restores the intended sync on the next release.

## 4. Findings requiring decision (not executed here)

The repoint alone does not make the release config healthy. Two problems were uncovered and are **flagged for decision** because they change release scope / published versions (outward-facing):

### 4a. Total version drift (all 55 valid packages)

Every one of the 55 packages that has a `plugin.json` shows a version mismatch between `.release-please-manifest.json` and `plugin.json`, in both directions, some by multiple majors:

- plugin ahead of manifest (e.g. `agents-md` manifest `0.2.0` vs plugin `0.2.1`, most opencode-toolkit entries).
- manifest far ahead of plugin (e.g. `azure-pipelines` / `fluentbit` manifest `4.0.0` vs plugin `1.0.1`; `tessl/publish-public` manifest `1.3.0` vs plugin `1.1.1`).

Consequence of the repoint: on each package's next release, release-please stamps the manifest version into `plugin.json`, which for the manifest-ahead cases is a large, user-visible jump for tessl consumers. The repoint does **not** retroactively reconcile existing drift; it only syncs going forward, per package, on next release.

**Recommended reconciliation (decision needed):** adopt the manifest as source of truth (policy 2). Where `plugin.json` is *ahead* of the manifest, bump the manifest up to match in a one-time edit so no published version appears to regress. Where the manifest is *ahead*, accept that the next release syncs `plugin.json` up (review the large-jump packages: azure-pipelines, fluentbit, and any other multi-major gaps). Do this before enabling releases, not as a silent side effect.

### 4b. Six stale package entries (relocated / renamed skills)

Six release-please packages point at directories that hold only a leftover tracked `CHANGELOG.md` and no `plugin.json`:

| Stale entry | Manifest version | Resolution |
|-------------|------------------|------------|
| skills/documentation/acceptance-criteria | 0.2.0 | Relocated to `skills/project-mgmt/issue-tracker-toolkit/acceptance-criteria` |
| skills/project-mgmt/moscow-prioritization | 0.2.0 | Relocated to `skills/project-mgmt/issue-tracker-toolkit/moscow-prioritization` |
| skills/project-mgmt/implementation-planner | 0.8.0 | Relocated to `skills/project-mgmt/planning-toolkit/implementation-planner` |
| skills/project-mgmt/wave-execution-planner | 1.3.0 | Relocated to `skills/project-mgmt/planning-toolkit/wave-execution-planner` |
| skills/project-mgmt/implementation-plan-splitter | 2.4.0 | No direct match; likely renamed/merged. **Resolve before removal.** |
| skills/project-mgmt/pr-decomposition | 0.2.0 | No direct match; likely renamed to `planning-toolkit/pr-stacker`. **Confirm before removal.** |

Note: the `planning-toolkit` and `issue-tracker-toolkit` directories have their own `.tessl-plugin/plugin.json` but are **not** release-please packages, so the relocated skills are currently unmanaged by release-please.

**Recommended resolution (decision needed):** per policy 4, remove the 6 stale package entries and their manifest versions and delete the husk `CHANGELOG.md` files; decide whether the toolkits (or their sub-skills) should become release-please packages, and migrate the retained versions accordingly; and first resolve where `implementation-plan-splitter` and `pr-decomposition` went. This is release-scope surgery and is left for a reviewed follow-up rather than executed here.

## 5. Handoff

- **A5** updates release-please config/manifest when the auditor's bundled skill relocates into `crates/skill-auditor/skill/`, following policy 4.
- **A6/A7/A8** follow policy 4 when `journal`/`adr` leave the skill tree for crates.
- The 4a drift reconciliation and 4b stale-entry removal should be scheduled as a reviewed follow-up before the next skill release, since both change published versions / release scope.
