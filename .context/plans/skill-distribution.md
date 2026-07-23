# B1 + B3: skill distribution and ecosystem coverage

**Status**: Complete (21-07-2026). Corrects the plan's manifest premises after verifying each installer's real format.
**Tasks**: B1 (root manifests + agent coverage) and B3 (tessl scripts + publish workflow), Wave 1.

## 1. Premise corrections (verified 21-07-2026)

The plan assumed four "root manifests" (a root tessl `.tessl-plugin/plugin.json` with `skills: ./skills/`, a root `skills.toml`, `.claude-plugin/marketplace.json`, and an apm equivalent). Verification against official sources changed this:

- **Root tessl plugin: not a real concept.** tessl is strictly per-plugin; one `.tessl-plugin/plugin.json` = one publishable plugin. There is no aggregate root manifest that publishes `./skills/` as many plugins. The existing per-toolkit manifests are the correct model. (docs.tessl.io/reference/configuration)
- **`skills.toml`: does not exist.** skills.sh (`vercel-labs/skills`) has no TOML config; it discovers skills by directory convention and optionally reuses `.claude-plugin/marketplace.json`. (github.com/vercel-labs/skills)
- **`.claude-plugin/marketplace.json` and `apm.yml`: real, but structurally mismatched** to our two-level `skills/<domain>/<toolkit>/<subskill>/SKILL.md` layout (Claude plugins expect `<plugin>/skills/<name>/SKILL.md`; apm expects `.apm/skills/<name>/SKILL.md`). Adopting them natively would require restructuring, and is unnecessary (see section 2).

## 2. Distribution model (decision)

1. **skills.sh convention-discovery is the primary distribution path.** `npx skills add pantheon-org/tekhne --all` (or `-a <agent>`) discovers our catalog layout with no manifest, and installs into the correct per-agent directories. It supports ~74 agents, a superset of our 42, including Claude Code (`.claude/skills/`), Cursor, Codex/OpenCode (`.agents/skills/`), and the Gemini family. (github.com/vercel-labs/skills)
2. **tessl stays per-plugin.** Keep the N per-toolkit `.tessl-plugin/plugin.json` manifests and the existing `publish-skills.yml` (per-plugin, version-gated, `tessl plugin publish`). No root tessl manifest is added.
3. **Claude native marketplace and apm are not pursued now.** They require layout restructuring for no coverage gain over skills.sh. If native Claude-plugin distribution is later wanted, restructure toolkits to `<plugin>/skills/<name>/` and add `.claude-plugin/marketplace.json` (verified template in section 5); the same restructure would enable apm `.apm/skills/`.

## 3. Agent-coverage gap analysis (the 42)

The 42 agents in `cli/lib/types/agents.ts`: amp, antigravity, augment, claude-code, openclaw, cline, codebuddy, codex, command-code, continue, cortex, crush, cursor, droid, gemini-cli, github-copilot, goose, iflow-cli, junie, kilo, kimi-cli, kiro-cli, kode, mcpjam, mistral-vibe, mux, neovate, opencode, openhands, pi, qoder, qwen-code, replit, roo, trae, trae-cn, warp, windsurf, zencoder, pochi, adal, universal.

skills.sh's supported-agent table (~68 rows, many grouping multiple agents; badge "70 more" ≈ 74) installs into the same per-agent directories our bespoke installer targets. Conclusion: **dropping the bespoke 42-agent installer (B5) does not lose coverage** — skills.sh supersedes it.

Gap to verify before B5 lands (documented, not blocking): confirm each of the 42 slugs has either a dedicated row or a group row in the skills.sh table. Watch the ones that resolve to the shared `.agents/skills/` path rather than a tool-native path (e.g. `codex`, `opencode`); if any target requires a tool-native `.{tool}/skills/` directory that skills.sh routes to `.agents/`, note it as a per-agent caveat. No agent is expected to lose an install path.

## 4. B3: tessl scripts and publish workflow

- **`publish-skills.yml` survives the CLI drop.** It uses `tessl plugin lint` + `tessl plugin publish` via `tesslio/setup-tessl` (no `cli/` dependency), detecting changed per-plugin dirs and version-gating. No change needed for the migration; it stays as the tessl publish path.
- **package.json tessl scripts**: `tessl:lint`, `tessl:publish`, `tessl:review`, `tessl:import` call the `tessl` CLI directly and survive. `tessl:manage` and `tessl:publish-check` run through `bun cli/index.ts tessl …` and are the only tessl-related consumers of `cli/`. B5 must re-home or retire these two: `tessl` has no direct bulk-manage/publish-check equivalent, so the bulk plugin.json management capability either moves to a small script or is dropped with a documented gap. Flagged for B5's owner table (tessl -> B3 owner).

## 5. Verified manifest templates (for future use only)

Not committed (see section 2). Kept here so a future restructure can adopt them.

`.claude-plugin/marketplace.json` (schema json.schemastore.org/claude-code-marketplace.json):
```json
{
  "name": "pantheon-tools",
  "owner": { "name": "Pantheon" },
  "plugins": [
    { "name": "terraform-toolkit", "source": "./skills/infrastructure/terraform", "description": "Terraform generator and validator" }
  ]
}
```

`apm.yml` (microsoft.github.io/apm):
```yaml
name: tekhne-skills
version: "1.0.0"
description: Curated agent skills
type: skill
targets: [claude, copilot, cursor, codex, gemini, opencode, windsurf]
includes: auto
```

## 6. Handoff

- **B5** (drop `cli/`): rely on skills.sh for the 42-agent install fan-out; re-home or retire `tessl:manage` / `tessl:publish-check`; keep `publish-skills.yml`.
- **B2a/B2b** (Astro docs): document `npx skills add pantheon-org/tekhne` as the install story.
- Revisit `.claude-plugin/marketplace.json` + apm only if native (non-skills.sh) distribution is required, which entails the toolkit restructure in section 2.

## 7. Follow-up (23-07-2026): crate-embedded skills and the naked-skill coupling

Three skills are now embedded into Rust crates at build time and distributed by
each crate's `install` command, with their registry publish retired (see
`journal`, `adr`, `skill-auditor`):

- `documentation/journal-entry-creator` -> `journal`
- `documentation/adr-creator` -> `adr`
- `agentic-harness/skill-quality-auditor` -> `skill-auditor`

**The coupling problem.** These SKILL.md files now reference their companion
binary (e.g. `journal lint`). If the skill is installed by any mechanism that
does not also deliver the binary (skills.sh convention-discovery over `skills/`,
a manual copy, or a future registry re-publish), the skill is "naked": it points
at a CLI that is not on `PATH`. The canonical source still lives under `skills/`
(the crate embeds it via `build.rs`), so a skills-only install path remains
physically possible.

**Two resolutions, and what we chose:**

1. **Prerequisite / first-run check (done, in the taxonomy+lint PR).** The
   SKILL.md declares a Prerequisites section splitting features into
   self-contained (authoring, bundled `scripts/validate-journal-entry.sh`) and
   CLI-backed (`journal lint`), and gates CLI-backed steps on a `journal
   --version` check with install guidance. This makes the skill honest about its
   dependency on any install path and lets it degrade gracefully. It is the
   universal, low-cost fix and is now the standing pattern for any crate-backed
   feature added to an embedded skill.
2. **Move the skill tree into the crate (deferred, structural).** Moving
   `SKILL.md` + assets into `crates/<tool>/skill/` would make crate-install the
   only sanctioned way to obtain the skill, so the binary is always present. It
   does not help if someone extracts the SKILL.md from the binary by hand, and
   it carries a real re-homing cost: everything that globs `skills/` must learn a
   second root, specifically the hk pre-commit checks (`skills/**`,
   `skills/**/SKILL.md`), skill-auditor's default `--skills-dir skills`, and the
   catalog/README generator. Until that tooling is re-pointed, moving would drop
   these three skills from quality, eval, and catalog coverage, which we
   explicitly want to keep. Treat as its own migration PR if the structural
   guarantee is later judged worth the tooling churn.

**Rule for future embedded-skill features:** any SKILL.md instruction that
invokes the companion binary MUST sit behind the Prerequisites check, so the
skill never assumes a tool it cannot guarantee is installed.
