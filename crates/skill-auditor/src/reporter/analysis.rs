//! Markdown analysis summary (Go `reporter.Analysis`). Output strings are
//! reproduced verbatim from the Go source for downstream parity, including the
//! punctuation in the headings.

use super::DIMENSION_ORDER;
use crate::scorer::Result;
use std::fmt::Write;

/// Return a markdown summary of a [`Result`] suitable for `Analysis.md`.
pub fn analysis(r: &Result) -> String {
    let mut sb = String::new();

    let _ = write!(sb, "# Skill Audit \u{2014} {}\n\n", r.skill);
    let _ = write!(
        sb,
        "**Grade:** {} ({}/{})\n\n",
        r.grade, r.total, r.max_total
    );

    let _ = write!(sb, "## Dimension Scores\n\n");
    let _ = writeln!(sb, "| Dimension | Score | Max |");
    let _ = writeln!(sb, "|---|---|---|");
    for (key, label, max) in DIMENSION_ORDER {
        let Some(score) = r.dimensions.get(*key) else {
            continue;
        };
        let _ = writeln!(sb, "| {label} | {score} | {max} |");
    }

    if !r.error_details.is_empty() || !r.warning_details.is_empty() {
        let _ = writeln!(sb, "\n## Diagnostics");
        if !r.error_details.is_empty() {
            let _ = write!(sb, "\n### Errors\n\n");
            for d in &r.error_details {
                let _ = writeln!(sb, "- **{}** {}", d.dimension, d.message);
            }
        }
        if !r.warning_details.is_empty() {
            let _ = write!(sb, "\n### Warnings\n\n");
            for d in &r.warning_details {
                let _ = writeln!(sb, "- **{}** {}", d.dimension, d.message);
            }
        }
    } else {
        let _ = writeln!(sb, "\n## Diagnostics\n\nNo errors or warnings.");
    }

    sb
}
