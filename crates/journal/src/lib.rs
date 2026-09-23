//! `journal`: create and validate structured journal entries.
//!
//! This crate promotes the `journal-entry-creator` skill into a self-contained
//! CLI. It renders timestamped `.md` entries into `YYYY/MM-Month/DD-Weekday/`
//! directories with YAML frontmatter, triple-synced dates, and the required
//! sections for each of the five entry types, and it validates entries against
//! the same rules the skill's `validate-journal-entry.sh` enforces. The
//! bundled skill is embedded at build time and can be installed into agent
//! directories via the shared
//! `skill-install` crate. No LLM access is involved. One deliberate exception
//! to an otherwise offline toolset: `archive_media` fetches already-discovered
//! media URLs over HTTP (see that module for why network access is scoped to
//! just this one command). `status` is a second deliberate exception of a
//! different kind: unlike every other module here, it reads a *consuming
//! repo's own* operational conventions (`.context/index.yaml`,
//! `.context/merge-queue.yaml`) rather than anything the generic
//! `journal-entry-creator` skill defines, and simply gathers nothing from
//! those sources in a repo that doesn't use them.

pub mod archive_media;
pub mod backfill;
pub mod date;
pub mod entry;
pub mod index;
pub mod install_cmd;
pub mod kb;
pub mod lint;
pub mod scan;
pub mod skill_bundle;
pub mod status;
pub mod taxonomy;
pub mod taxonomy_sync;
pub mod validate;
