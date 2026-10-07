# Changelog

## Unreleased

### Features

* **create-context-file:** follow-ups can carry `severity` (`CRITICAL`, `HIGH`, `MEDIUM`, `LOW`), `priority` (`P1`, `P2`, `P3`) and `graded` (the day the user confirmed them). The generator takes `--severity`, `--priority` and `--graded`, requires all three together, accepts them on follow-ups only, has no default for `--graded`, and writes them last in the frontmatter. The schema declares the three fields at the root with no type restriction. A new `references/follow-up-grading.md` holds the decision test for each value, and `SKILL.md` gains a "Filing a follow-up" section: the agent proposes grades with a reason and passes the flags only after the user confirms.
* **context-index:** `regenerate-context-index.sh` writes `severity:`, `priority:` and `graded:` into the index entry of a follow-up that sets them, after `date:`, and warns on stderr about an unknown value or an incomplete set. The entry is kept as written so readers can show it as needing a grade.
* **follow-up:** a new workflow step ranks active follow-ups by grade (severity, then priority, then newest) and lists those needing a grade separately; the active-status match is now case-insensitive.
* **create-context-file:** `singular_of` maps `handovers` to `handover`, matching the `handovers` typology that `context-index` now indexes. This was already the result of the trailing-`s` fallback, so generated files are unchanged. `handovers` is deliberately not in `KNOWN_TYPES`, because handovers are written by `handover-document-creator`, not this script.
* **context-index:** the paired `check-context-filenames.sh` now treats `handovers/` as a known directory, so projects that run it also enforce the date-first filename and the filename-date-equals-frontmatter-date check for handovers.

## [1.0.0](https://github.com/pantheon-org/tekhne/compare/v0.4.0...v1.0.0) (2026-09-08)

### ⚠ BREAKING CHANGES

* **create-context-file:** filenames are now date-prefixed (YYYY-MM-DD-slug.md) instead of three-word IDs, and types are an open typology set instead of the fixed plan/justification/scratch enum.

### Features

* **create-context-file:** organize by typology with date-prefixed filenames ([#170](https://github.com/pantheon-org/tekhne/issues/170)) ([f37ffc8](https://github.com/pantheon-org/tekhne/commit/f37ffc8139c7c43250cbc2112aeb13deeb51939b))
* **project-mgmt:** add blocks/blocked-by frontmatter + context-ready.sh ([#284](https://github.com/pantheon-org/tekhne/issues/284)) ([61ef46d](https://github.com/pantheon-org/tekhne/commit/61ef46d69dff1cd1e7a357d32614b4a9ed51e917))
* **skill-quality-auditor:** enforce References table standard + add eval suites across 40+ skills ([#26](https://github.com/pantheon-org/tekhne/issues/26)) ([2d4c8cf](https://github.com/pantheon-org/tekhne/commit/2d4c8cfb0a57290e20e70e72186b8021bf802687))
* **skills:** upstream adr-capture, context-index, and journal-entry-creator capabilities from the Journal repo ([#275](https://github.com/pantheon-org/tekhne/issues/275)) ([f805467](https://github.com/pantheon-org/tekhne/commit/f8054674e09be4da5f6049bad00154172b9c7aa1))

### Bug Fixes

* **create-context-file:** write singular type in frontmatter, not the plural folder name ([#274](https://github.com/pantheon-org/tekhne/issues/274)) ([8604f76](https://github.com/pantheon-org/tekhne/commit/8604f7610906dbc5b3707c6e646bfb7bca04667d))

## [0.4.0](https://github.com/pantheon-org/tekhne/compare/v0.3.0...v0.4.0) (2026-07-10)

### Features

* **skill-quality-auditor:** enforce References table standard + add eval suites across 40+ skills ([#26](https://github.com/pantheon-org/tekhne/issues/26)) ([e6da355](https://github.com/pantheon-org/tekhne/commit/e6da355d773aa5646dea9ec6128af0fee0a43ebb))

## [0.3.0](https://github.com/pantheon-org/tekhne/compare/v0.2.0...v0.3.0) (2026-05-15)

### Features

* **skill-quality-auditor:** enforce References table standard + add eval suites across 40+ skills ([#26](https://github.com/pantheon-org/tekhne/issues/26)) ([e6da355](https://github.com/pantheon-org/tekhne/commit/e6da355d773aa5646dea9ec6128af0fee0a43ebb))
