use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

use crate::content::Entry;

/// Select the entry for a given date from the sorted collection.
///
/// The hash is fixed rather than relying on the standard library hasher so
/// that selections stay stable across Rust versions.
pub fn daily<'a>(entries: &'a [Entry], date: &str) -> &'a Entry {
    &entries[daily_index(entries, date)]
}

fn daily_index(entries: &[Entry], date: &str) -> usize {
    fnv1a(date) as usize % entries.len()
}

/// Select an entry using a time seeded xorshift generator.
///
/// The entry for today is skipped so a random request does not repeat the
/// daily encounter.
pub fn random<'a>(entries: &'a [Entry], date: &str) -> &'a Entry {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or(0);
    let mut state = nanos ^ (std::process::id() as u64);
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;

    let mut index = (state as usize) % entries.len();
    if entries.len() > 1 && index == daily_index(entries, date) {
        index = (index + 1) % entries.len();
    }
    &entries[index]
}

/// Format the current UTC date as YYYY-MM-DD.
pub fn today() -> Result<String> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_secs() as i64;
    let days = seconds.div_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    Ok(format!("{year:04}-{month:02}-{day:02}"))
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

fn fnv1a(text: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn civil_from_days_matches_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(20_454), (2026, 1, 1));
    }

    #[test]
    fn fnv1a_is_stable() {
        assert_eq!(fnv1a("2026-11-25"), fnv1a("2026-11-25"));
        assert_ne!(fnv1a("2026-11-25"), fnv1a("2026-11-26"));
    }

    #[test]
    fn daily_selection_repeats_for_the_same_date() {
        let entries = crate::content::load().expect("embedded collection should parse");
        let first = daily(&entries, "2026-11-25");
        let second = daily(&entries, "2026-11-25");
        assert_eq!(first.id, second.id);
    }

    #[test]
    fn sample_dates_select_multiple_entries() {
        let entries = crate::content::load().expect("embedded collection should parse");
        let mut selected = HashSet::new();
        for day in 1..=28 {
            let date = format!("2026-11-{day:02}");
            selected.insert(daily(&entries, &date).id.clone());
        }
        assert!(selected.len() > 1);
    }

    #[test]
    fn random_never_matches_todays_entry() {
        let entries = crate::content::load().expect("embedded collection should parse");
        let date = "2026-11-25";
        let today = daily(&entries, date).id.clone();
        for _ in 0..64 {
            assert_ne!(random(&entries, date).id, today);
        }
    }
}
