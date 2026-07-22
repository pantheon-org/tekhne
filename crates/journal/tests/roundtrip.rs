//! End-to-end checks: every entry type the CLI can create must render into
//! `YYYY/MM/` and then pass the validator, and each type carries the right
//! required tag and template sections.

use journal::date::{Date, Timestamp};
use journal::entry::{EntrySpec, EntryType};
use journal::validate::{validate, Outcome};

fn spec(entry_type: EntryType, ticket: Option<&str>, source: Option<&str>) -> EntrySpec {
    EntrySpec {
        entry_type,
        title: "Example Investigation".to_string(),
        author: "Author".to_string(),
        timestamp: Timestamp::fixed(Date::new(2026, 7, 22), 9, 5),
        ticket: ticket.map(str::to_string),
        source: source.map(str::to_string),
    }
}

#[test]
fn each_type_creates_a_compliant_entry() {
    let cases = [
        (EntryType::Journal, None, None),
        (EntryType::Troubleshooting, Some("PROJ-1234"), None),
        (EntryType::Learning, None, None),
        (
            EntryType::ArticleSummary,
            None,
            Some("https://example.com/x"),
        ),
        (EntryType::TicketRefinement, Some("TICKET-123"), None),
    ];

    for (entry_type, ticket, source) in cases {
        let tmp = tempfile::tempdir().unwrap();
        let s = spec(entry_type, ticket, source);
        let path = s.create(tmp.path()).unwrap();

        // The entry lands in the date-partitioned tree.
        let rel = path.strip_prefix(tmp.path()).unwrap();
        assert!(
            rel.starts_with("2026/07"),
            "{entry_type:?} not under 2026/07: {rel:?}"
        );

        // And it passes every compliance check.
        assert_eq!(
            validate(&path),
            Ok(Outcome::Validated),
            "{entry_type:?} entry failed validation"
        );

        // The required per-type tag is present in the frontmatter.
        let doc = std::fs::read_to_string(&path).unwrap();
        assert!(
            doc.contains(&format!("- {}\n", entry_type.primary_tag())),
            "{entry_type:?} missing primary tag"
        );
    }
}

#[test]
fn troubleshooting_prefixes_ticket_in_filename() {
    let tmp = tempfile::tempdir().unwrap();
    let s = spec(EntryType::Troubleshooting, Some("PROJ-1234"), None);
    let path = s.create(tmp.path()).unwrap();
    let name = path.file_name().unwrap().to_string_lossy();
    assert!(
        name.starts_with("2026-07-22-proj-1234-"),
        "unexpected filename: {name}"
    );
}

#[test]
fn dates_are_triple_synced() {
    let tmp = tempfile::tempdir().unwrap();
    let path = spec(EntryType::Journal, None, None)
        .create(tmp.path())
        .unwrap();
    let doc = std::fs::read_to_string(&path).unwrap();
    let name = path.file_name().unwrap().to_string_lossy();

    assert!(name.starts_with("2026-07-22-"), "filename date");
    assert!(doc.contains("date: 2026-07-22"), "frontmatter date");
    assert!(
        doc.lines()
            .any(|l| l.starts_with("# ") && l.ends_with("July 22, 2026")),
        "H1 long date"
    );
}
