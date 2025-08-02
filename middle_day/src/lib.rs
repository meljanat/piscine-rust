use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        None
    } else {
        NaiveDate::from_ymd_opt(year as i32, 6, 15).map(|date| date.weekday())
    }
}
