//! A tiny, dependency-free calendar date and wall-clock time, with an
//! injectable "now" so entry generation is deterministic under test.
//!
//! Only the conversions the journal needs are implemented: a UTC instant to a
//! civil `(year, month, day, hour, minute)`, an ISO `YYYY-MM-DD` rendering, a
//! long `Month D, YYYY` rendering (matching the shell validator's
//! `date "+%B %-d, %Y"`), and a weekday name (for the entry path's
//! `DD-Weekday` segment).

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

const WEEKDAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
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
        format!("{} {}, {}", self.month_name(), self.day, self.year)
    }

    /// Full month name, e.g. `"September"`.
    pub fn month_name(&self) -> &'static str {
        MONTHS
            .get((self.month.saturating_sub(1)) as usize)
            .copied()
            .unwrap_or("Unknown")
    }

    /// Full weekday name, e.g. `"Wednesday"`, per the proleptic Gregorian
    /// calendar (the same one `civil_from_days` assumes).
    pub fn weekday_name(&self) -> &'static str {
        let days = days_from_civil(self.year, self.month, self.day);
        // 1970-01-01 (day 0) was a Thursday; WEEKDAYS is Sunday-first, so the
        // offset from day 0 to Sunday-indexed is +4. `rem_euclid` keeps the
        // index correct for dates before the epoch.
        let idx = (days + 4).rem_euclid(7) as usize;
        WEEKDAYS[idx]
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

/// Convert a civil date to a day count relative to the Unix epoch -- the
/// inverse of [`civil_from_days`], via the same source (Howard Hinnant's
/// `days_from_civil`).
fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y as i64 - 1 } else { y as i64 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400; // [0, 399]
    let mp = if m > 2 { m - 3 } else { m + 9 } as i64; // [0, 11]
    let doy = (153 * mp + 2) / 5 + d as i64 - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146_097 + doe - 719_468
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

    #[test]
    fn month_name_matches_long() {
        assert_eq!(Date::new(2026, 9, 23).month_name(), "September");
        assert_eq!(Date::new(2026, 1, 1).month_name(), "January");
        assert_eq!(Date::new(2026, 12, 31).month_name(), "December");
    }

    #[test]
    fn weekday_name_matches_known_dates() {
        // 2000-01-01 is a well-known reference Saturday.
        assert_eq!(Date::new(2000, 1, 1).weekday_name(), "Saturday");
        // 1970-01-01 (the epoch itself) was a Thursday.
        assert_eq!(Date::new(1970, 1, 1).weekday_name(), "Thursday");
        // 2026-09-23, this repo's own worked example in the path-restructuring
        // plan (`2026/09-September/23-Wednesday/...`).
        assert_eq!(Date::new(2026, 9, 23).weekday_name(), "Wednesday");
    }

    #[test]
    fn days_from_civil_round_trips_through_civil_from_days() {
        for (y, m, d) in [
            (1970, 1, 1),
            (2000, 1, 1),
            (2024, 2, 29),
            (2026, 7, 22),
            (2026, 12, 31),
            (1969, 12, 31),
        ] {
            let days = days_from_civil(y, m, d);
            assert_eq!(civil_from_days(days), Date::new(y, m, d), "{y}-{m}-{d}");
        }
    }
}
