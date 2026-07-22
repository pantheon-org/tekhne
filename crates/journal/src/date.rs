//! A tiny, dependency-free calendar date and wall-clock time, with an
//! injectable "now" so entry generation is deterministic under test.
//!
//! Only the conversions the journal needs are implemented: a UTC instant to a
//! civil `(year, month, day, hour, minute)`, an ISO `YYYY-MM-DD` rendering, and
//! a long `Month D, YYYY` rendering (matching the shell validator's
//! `date "+%B %-d, %Y"`).

use std::time::{SystemTime, UNIX_EPOCH};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// A calendar date in the proleptic Gregorian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    /// Four-digit year.
    pub year: i32,
    /// Month of the year, 1-12.
    pub month: u32,
    /// Day of the month, 1-31.
    pub day: u32,
}

impl Date {
    /// Construct a date from its components without validation.
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// ISO 8601 `YYYY-MM-DD`, the filename and frontmatter form.
    pub fn iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Long `Month D, YYYY` form used in the H1 title, with no leading zero on
    /// the day (matches `date "+%B %-d, %Y"`).
    pub fn long(&self) -> String {
        let month = MONTHS.get((self.month.saturating_sub(1)) as usize).copied();
        format!("{} {}, {}", month.unwrap_or("Unknown"), self.day, self.year)
    }
}

/// A wall-clock instant: a calendar date plus the hour and minute (UTC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    /// The calendar date.
    pub date: Date,
    /// Hour of day, 0-23.
    pub hour: u32,
    /// Minute of the hour, 0-59.
    pub minute: u32,
}

impl Timestamp {
    /// The current UTC instant, read from the system clock.
    pub fn now() -> Self {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        Self::from_unix(secs)
    }

    /// Build a timestamp from a Unix epoch second count (UTC).
    pub fn from_unix(secs: i64) -> Self {
        let days = secs.div_euclid(86_400);
        let seconds_of_day = secs.rem_euclid(86_400);
        Self {
            date: civil_from_days(days),
            hour: (seconds_of_day / 3_600) as u32,
            minute: ((seconds_of_day % 3_600) / 60) as u32,
        }
    }

    /// A fixed timestamp, for deterministic construction in callers and tests.
    pub fn fixed(date: Date, hour: u32, minute: u32) -> Self {
        Self { date, hour, minute }
    }

    /// `HH:MM` rendering of the wall-clock time.
    pub fn hhmm(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }
}

/// Convert a day count relative to the Unix epoch to a civil date, using
/// Howard Hinnant's `civil_from_days` algorithm.
fn civil_from_days(days: i64) -> Date {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = if month <= 2 { year + 1 } else { year };
    Date::new(year as i32, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso_and_long_formats() {
        let d = Date::new(2026, 7, 22);
        assert_eq!(d.iso(), "2026-07-22");
        assert_eq!(d.long(), "July 22, 2026");
    }

    #[test]
    fn long_has_no_leading_zero_on_day() {
        assert_eq!(Date::new(2025, 2, 4).long(), "February 4, 2025");
    }

    #[test]
    fn unix_epoch_is_1970_01_01() {
        let ts = Timestamp::from_unix(0);
        assert_eq!(ts.date, Date::new(1970, 1, 1));
        assert_eq!(ts.hhmm(), "00:00");
    }

    #[test]
    fn known_instant_round_trips() {
        // 2026-07-22T14:30:00Z == 1784730600 seconds since the epoch.
        let ts = Timestamp::from_unix(1_784_730_600);
        assert_eq!(ts.date, Date::new(2026, 7, 22));
        assert_eq!(ts.hhmm(), "14:30");
    }

    #[test]
    fn leap_day_conversion() {
        // 2024-02-29T00:00:00Z == 1709164800 seconds since the epoch.
        let ts = Timestamp::from_unix(1_709_164_800);
        assert_eq!(ts.date, Date::new(2024, 2, 29));
    }
}
