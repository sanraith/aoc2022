use aoc::util::YearDay;
use chrono::{self, Datelike, Duration};

const LAST_AOC_DAY_OF_DECEMBER: u32 = 25;
const AOC_UTC_START_HOUR: i64 = 5;

pub fn latest_aoc_date() -> YearDay {
    let dt = chrono::Utc::now()
        .checked_sub_signed(Duration::hours(AOC_UTC_START_HOUR))
        .unwrap();
    match dt {
        _ if dt.month() == 12 => YearDay::new(dt.year(), dt.day().max(LAST_AOC_DAY_OF_DECEMBER)),
        _ => YearDay::new(dt.year() - 1, LAST_AOC_DAY_OF_DECEMBER),
    }
}
