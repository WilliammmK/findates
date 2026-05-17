use std::collections::HashSet;

use chrono::NaiveDate;
use findates::algebra::{add_business_days, subtract_business_days};
use findates::calendar::{basic_calendar, Calendar};
use findates::error::BusinessDayError;

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).unwrap()
}

fn calendar_with_holiday(date: NaiveDate) -> Calendar {
    let mut cal = basic_calendar();
    cal.add_holidays(&[date].into_iter().collect());
    cal
}

fn calendar_with_holidays(dates: impl IntoIterator<Item = NaiveDate>) -> Calendar {
    let mut cal = basic_calendar();
    let set: HashSet<NaiveDate> = dates.into_iter().collect();
    cal.add_holidays(&set);
    cal
}

// ── add_business_days ─────────────────────────────────────────────────────────

#[test]
fn add_bd_invalid_start_weekend_test() {
    let cal = basic_calendar();
    let saturday = d(2024, 3, 16);
    assert_eq!(
        add_business_days(&saturday, 1, &cal),
        Err(BusinessDayError::InvalidStartDate)
    );
}

#[test]
fn add_bd_invalid_start_holiday_test() {
    let monday = d(2024, 3, 18);
    let cal = calendar_with_holiday(monday);
    assert_eq!(
        add_business_days(&monday, 1, &cal),
        Err(BusinessDayError::InvalidStartDate)
    );
}

#[test]
fn add_bd_n_zero_non_business_day_test() {
    let cal = basic_calendar();
    let saturday = d(2024, 3, 16);
    assert_eq!(
        add_business_days(&saturday, 0, &cal),
        Err(BusinessDayError::InvalidStartDate)
    );
}

#[test]
fn add_bd_n_zero_valid_test() {
    let cal = basic_calendar();
    let monday = d(2024, 3, 18);
    assert_eq!(add_business_days(&monday, 0, &cal), Ok(monday));
}

#[test]
fn add_bd_across_weekend_test() {
    let cal = basic_calendar();
    let friday = d(2024, 3, 15);
    assert_eq!(
        add_business_days(&friday, 1, &cal).unwrap(),
        d(2024, 3, 18) // Monday
    );
}

#[test]
fn add_bd_across_month_boundary_test() {
    let cal = basic_calendar();
    // 2024-03-29 (Friday) + 1 bd → 2024-04-01 (Monday)
    let friday = d(2024, 3, 29);
    assert_eq!(
        add_business_days(&friday, 1, &cal).unwrap(),
        d(2024, 4, 1)
    );
}

#[test]
fn add_bd_across_year_boundary_test() {
    let cal = basic_calendar();
    // 2024-12-31 (Tuesday) + 1 bd → 2025-01-01 (Wednesday, no holidays in basic_calendar)
    let tuesday = d(2024, 12, 31);
    assert_eq!(
        add_business_days(&tuesday, 1, &cal).unwrap(),
        d(2025, 1, 1)
    );
}

#[test]
fn add_bd_long_holiday_test() {
    // Mon–Fri 2024-03-18 through 2024-03-22 are holidays.
    // 2024-03-15 (Friday) + 1 bd skips the holiday week and the following
    // weekend, landing on 2024-03-25 (Monday).
    let cal = calendar_with_holidays((18u32..=22).map(|day| d(2024, 3, day)));
    let friday = d(2024, 3, 15);
    assert_eq!(
        add_business_days(&friday, 1, &cal).unwrap(),
        d(2024, 3, 25)
    );
}

// ── subtract_business_days ────────────────────────────────────────────────────

#[test]
fn sub_bd_invalid_start_weekend_test() {
    let cal = basic_calendar();
    let saturday = d(2024, 3, 16);
    assert_eq!(
        subtract_business_days(&saturday, 1, &cal),
        Err(BusinessDayError::InvalidStartDate)
    );
}

#[test]
fn sub_bd_invalid_start_holiday_test() {
    let monday = d(2024, 3, 18);
    let cal = calendar_with_holiday(monday);
    assert_eq!(
        subtract_business_days(&monday, 1, &cal),
        Err(BusinessDayError::InvalidStartDate)
    );
}

#[test]
fn sub_bd_n_zero_non_business_day_test() {
    let cal = basic_calendar();
    let saturday = d(2024, 3, 16);
    assert_eq!(
        subtract_business_days(&saturday, 0, &cal),
        Err(BusinessDayError::InvalidStartDate)
    );
}

#[test]
fn sub_bd_n_zero_valid_test() {
    let cal = basic_calendar();
    let monday = d(2024, 3, 18);
    assert_eq!(subtract_business_days(&monday, 0, &cal), Ok(monday));
}

#[test]
fn sub_bd_across_weekend_test() {
    let cal = basic_calendar();
    let monday = d(2024, 3, 18);
    assert_eq!(
        subtract_business_days(&monday, 1, &cal).unwrap(),
        d(2024, 3, 15) // Friday
    );
}

#[test]
fn sub_bd_across_month_boundary_test() {
    let cal = basic_calendar();
    // 2024-04-01 (Monday) - 1 bd → 2024-03-29 (Friday)
    let monday = d(2024, 4, 1);
    assert_eq!(
        subtract_business_days(&monday, 1, &cal).unwrap(),
        d(2024, 3, 29)
    );
}

#[test]
fn sub_bd_across_year_boundary_test() {
    let cal = basic_calendar();
    // 2025-01-01 (Wednesday) - 1 bd → 2024-12-31 (Tuesday)
    let wednesday = d(2025, 1, 1);
    assert_eq!(
        subtract_business_days(&wednesday, 1, &cal).unwrap(),
        d(2024, 12, 31)
    );
}

#[test]
fn sub_bd_long_holiday_test() {
    // Mon–Fri 2024-03-18 through 2024-03-22 are holidays.
    // 2024-03-25 (Monday) - 1 bd skips the holiday week and the preceding
    // weekend, landing on 2024-03-15 (Friday).
    let cal = calendar_with_holidays((18u32..=22).map(|day| d(2024, 3, day)));
    let monday = d(2024, 3, 25);
    assert_eq!(
        subtract_business_days(&monday, 1, &cal).unwrap(),
        d(2024, 3, 15)
    );
}
