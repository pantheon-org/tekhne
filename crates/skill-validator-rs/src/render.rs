//! Report renderers: `text` (human-readable, default), `json` (stable and
//! machine-parseable), `markdown` (a results table), plus GitHub Actions
//! annotation lines (`--emit-annotations`). Renderers never decide the exit
//! code; they only present the [`CliReport`] built by the runners.

use skill_validator_rs::{ContaminationReport, ContentReport, Level, ValidationResult};

use crate::model::{CliReport, OutputFormat};

/// Render `report` in the requested `format` and return the string to print to
/// stdout. Pass results are omitted from text and markdown to keep the gate
/// output focused on findings.
pub fn render(report: &CliReport, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => render_text(report),
        OutputFormat::Json => render_json(report),
        OutputFormat::Markdown => render_markdown(report),
    }
}

fn render_json(report: &CliReport) -> String {
    serde_json::to_string_pretty(report).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
}

fn render_text(report: &CliReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("Skill: {}\n", report.skill_dir));

    let findings: Vec<&ValidationResult> = report
        .results
        .iter()
        .filter(|r| r.level != Level::Pass)
        .collect();

    if findings.is_empty() && report.results.is_empty() {
        // No structure group ran (e.g. `analyze content`).
    } else if findings.is_empty() {
        out.push_str("  No issues found.\n");
    } else {
        for r in findings {
            out.push_str(&format!("  {}\n", text_line(r)));
        }
    }

    if let Some(content) = &report.content {
        out.push_str(&render_content_text("Content metrics", content));
    }
    if let Some(contamination) = &report.contamination {
        out.push_str(&render_contamination_text(contamination));
    }
    for rr in &report.reference_reports {
        if let Some(content) = &rr.content {
            out.push_str(&render_content_text(
                &format!("Content ({})", rr.file),
                content,
            ));
        }
    }

    if !report.results.is_empty() {
        out.push_str(&format!(
            "Summary: {} error{}, {} warning{}\n",
            report.errors,
            plural(report.errors),
            report.warnings,
            plural(report.warnings),
        ));
    }
    out
}

fn text_line(r: &ValidationResult) -> String {
    let level = r.level.as_str().to_uppercase();
    let location = location(&r.file, r.line);
    if location.is_empty() {
        format!("[{level}] {}: {}", r.category, r.message)
    } else {
        format!("[{level}] {}: {} ({location})", r.category, r.message)
    }
}

fn render_content_text(title: &str, c: &ContentReport) -> String {
    format!(
        "{title}:\n  words={} code_blocks={} sentences={} imperative={} \
         sections={} list_items={}\n  code_block_ratio={:.4} imperative_ratio={:.4} \
         information_density={:.4} instruction_specificity={:.4}\n  code_languages={:?}\n",
        c.word_count,
        c.code_block_count,
        c.sentence_count,
        c.imperative_count,
        c.section_count,
        c.list_item_count,
        c.code_block_ratio,
        c.imperative_ratio,
        c.information_density,
        c.instruction_specificity,
        c.code_languages,
    )
}

fn render_contamination_text(c: &ContaminationReport) -> String {
    format!(
        "Contamination:\n  level={} score={:.4} scope_breadth={} language_mismatch={}\n  \
         primary_category={} mismatched_categories={:?}\n",
        c.contamination_level,
        c.contamination_score,
        c.scope_breadth,
        c.language_mismatch,
        c.primary_category,
        c.mismatched_categories,
    )
}

fn render_markdown(report: &CliReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Validation report: `{}`\n\n", report.skill_dir));

    let findings: Vec<&ValidationResult> = report
        .results
        .iter()
        .filter(|r| r.level != Level::Pass)
        .collect();

    if !report.results.is_empty() {
        if findings.is_empty() {
            out.push_str("No issues found.\n\n");
        } else {
            out.push_str("| Level | Category | Message | Location |\n");
            out.push_str("| ----- | -------- | ------- | -------- |\n");
            for r in findings {
                out.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    r.level.as_str(),
                    escape_pipes(&r.category),
                    escape_pipes(&r.message),
                    escape_pipes(&location(&r.file, r.line)),
                ));
            }
            out.push('\n');
        }
        out.push_str(&format!(
            "**{} error{}, {} warning{}**\n",
            report.errors,
            plural(report.errors),
            report.warnings,
            plural(report.warnings),
        ));
    }

    if let Some(content) = &report.content {
        out.push_str(&markdown_content(content));
    }
    out
}

fn markdown_content(c: &ContentReport) -> String {
    format!(
        "\n## Content metrics\n\n\
         - Words: {}\n- Code blocks: {}\n- Sentences: {}\n- Imperative sentences: {}\n\
         - Sections: {}\n- List items: {}\n- Code-block ratio: {:.4}\n\
         - Imperative ratio: {:.4}\n- Information density: {:.4}\n\
         - Instruction specificity: {:.4}\n",
        c.word_count,
        c.code_block_count,
        c.sentence_count,
        c.imperative_count,
        c.section_count,
        c.list_item_count,
        c.code_block_ratio,
        c.imperative_ratio,
        c.information_density,
        c.instruction_specificity,
    )
}

/// GitHub Actions annotation lines for every error and warning finding
/// (`::error` / `::warning`). Pass/Info results are not annotated.
pub fn render_annotations(report: &CliReport) -> String {
    let mut out = String::new();
    for r in &report.results {
        let command = match r.level {
            Level::Error => "error",
            Level::Warning => "warning",
            _ => continue,
        };
        let props = annotation_props(&report.skill_dir, &r.file, r.line);
        let message = escape_annotation_data(&format!("{}: {}", r.category, r.message));
        out.push_str(&format!("::{command}{props}::{message}\n"));
    }
    out
}

/// The `file=...,line=...` property block, joining the skill directory to the
/// result's relative file. Empty when the result carries no file.
fn annotation_props(skill_dir: &str, file: &str, line: usize) -> String {
    if file.is_empty() {
        return String::new();
    }
    let path = escape_annotation_prop(&format!("{skill_dir}/{file}"));
    if line > 0 {
        format!(" file={path},line={line}")
    } else {
        format!(" file={path}")
    }
}

fn location(file: &str, line: usize) -> String {
    match (file.is_empty(), line) {
        (true, _) => String::new(),
        (false, 0) => file.to_string(),
        (false, n) => format!("{file}:{n}"),
    }
}

fn plural(n: usize) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

fn escape_pipes(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

/// Escape a workflow-command data segment (GitHub Actions message escaping).
fn escape_annotation_data(s: &str) -> String {
    s.replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

/// Escape a workflow-command property value (additionally escapes `,` and `:`).
fn escape_annotation_prop(s: &str) -> String {
    escape_annotation_data(s)
        .replace(',', "%2C")
        .replace(':', "%3A")
}
