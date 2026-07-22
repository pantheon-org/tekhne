//! Shared helpers ported from the Go validator's `util` package: number
//! formatting, half-away-from-zero rounding, pluralisation, and sorted-key
//! extraction. Behaviour matches the Go originals byte-for-byte.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

// Shared markdown-stripping regexes ported from the Go `util` package. Go's
// `\w` and `\s` are ASCII-only; `\w` is scoped with `(?-u:...)` and the dot
// (`.`) stays in Unicode mode so `regex::Regex` accepts the pattern (it never
// matches invalid UTF-8). `(?s)` makes the dot span newlines, as Go's `(?s)`.

/// Removes fenced code blocks (backtick and tilde) from markdown
/// (Go `util.CodeBlockStrip`).
pub static CODE_BLOCK_STRIP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)(?:```|~~~)(?-u:\w)*\r?\n.*?(?:```|~~~)").expect("CODE_BLOCK_STRIP")
});

/// Removes inline code spans (Go `util.InlineCodeStrip`).
pub static INLINE_CODE_STRIP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`[^`]+`").expect("INLINE_CODE_STRIP"));

/// Extracts fenced code block bodies in capture group 1
/// (Go `util.CodeBlockPattern`).
pub static CODE_BLOCK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)(?:```|~~~)(?-u:\w)*\r?\n(.*?)(?:```|~~~)").expect("CODE_BLOCK_PATTERN")
});

/// Round `val` to `places` decimal places (half away from zero).
///
/// Mirrors Go `util.RoundTo`: `math.Round(val*pow) / pow`. Rust's
/// [`f64::round`] is also half-away-from-zero, so results agree at 4dp.
pub fn round_to(val: f64, places: i32) -> f64 {
    let pow = 10f64.powi(places);
    (val * pow).round() / pow
}

/// Format an integer with thousands-separator commas (Go `util.FormatNumber`).
pub fn format_number(n: i64) -> String {
    let s = n.to_string();
    if n < 1000 && n > -1000 {
        return s;
    }
    // Match the Go implementation, which only inserts separators for the
    // digit run and never runs on negative inputs in practice.
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(bytes.len() + bytes.len() / 3);
    let len = bytes.len();
    for (i, c) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push(',');
        }
        out.push(*c as char);
    }
    out
}

/// Return "s" when `n != 1`, empty otherwise (Go `util.PluralS`).
pub fn plural_s(n: usize) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

/// Derive a skill name from a directory path (Go `util.SkillNameFromDir`).
pub fn skill_name_from_dir(dir: &Path) -> String {
    dir.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

/// Quote a string the way Go's `fmt` verb `%q` (strconv.Quote) does: a
/// double-quoted literal with the usual escapes. Printable Unicode is kept
/// verbatim; this covers the skill names and values the validator quotes.
pub fn go_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '\x07' => out.push_str("\\a"),
            '\x08' => out.push_str("\\b"),
            '\x0b' => out.push_str("\\v"),
            '\x0c' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Return the keys of a map sorted lexicographically (Go `util.SortedKeys`).
///
/// A [`BTreeMap`] is already ordered, so this simply collects its keys.
pub fn sorted_keys<V>(m: &BTreeMap<String, V>) -> Vec<String> {
    m.keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_to_matches_go() {
        assert_eq!(round_to(0.22222, 4), 0.2222);
        assert_eq!(round_to(0.5, 0), 1.0);
        assert_eq!(round_to(2.0 / 9.0, 4), 0.2222);
    }

    #[test]
    fn format_number_inserts_commas() {
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(25_000), "25,000");
        assert_eq!(format_number(1_234_567), "1,234,567");
    }

    #[test]
    fn plural_s_agrees_with_go() {
        assert_eq!(plural_s(1), "");
        assert_eq!(plural_s(0), "s");
        assert_eq!(plural_s(2), "s");
    }
}
