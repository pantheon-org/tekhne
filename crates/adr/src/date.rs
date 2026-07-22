//! A dependency-free "today" for default ADR dates.
//!
//! ADR commands accept an injectable date string so tests are deterministic;
//! the CLI defaults to [`today`], which converts the system clock to a
//! `YYYY-MM-DD` civil date using Howard Hinnant's `civil_from_days` algorithm.

use std::time::{SystemTime, UNIX_EPOCH};

/// The current UTC date as `YYYY-MM-DD`.
///
/// Falls back to the Unix epoch (`1970-01-01`) if the system clock is before
/// it, which keeps the function total without an error path.
pub fn today() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    format_date(secs.div_euclid(86_400))
}

/// Format a count of days since the Unix epoch as a `YYYY-MM-DD` string.
pub fn format_date(days_since_epoch: i64) -> String {
    let (y, m, d) = civil_from_days(days_since_epoch);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Convert days since 1970-01-01 to a `(year, month, day)` civil date.
///
/// This is Howard Hinnant's well-known `civil_from_days` algorithm, valid for
/// the full range of the proleptic Gregorian calendar.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32; // [1, 12]
    let year = if m <= 2 { y + 1 } else { y };
    (year, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_is_nineteen_seventy() {
        assert_eq!(format_date(0), "1970-01-01");
    }

    #[test]
    fn known_dates_round_trip() {
        // 2000-01-01 is 10957 days after the epoch.
        assert_eq!(format_date(10_957), "2000-01-01");
        // A leap day.
        assert_eq!(format_date(19_782), "2024-02-29");
        // The task reference date.
        assert_eq!(format_date(20_656), "2026-07-22");
    }

    #[test]
    fn today_has_expected_shape() {
        let t = today();
        assert_eq!(t.len(), 10);
        assert_eq!(t.as_bytes()[4], b'-');
        assert_eq!(t.as_bytes()[7], b'-');
    }
}
