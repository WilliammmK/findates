// Integration tests for calendar-related functionality.
// These tests validate business day determination, schedule generation, and business day counting
// based on calendar rules (weekends, holidays) and adjustment conventions.

use chrono::{Datelike, NaiveDate, Weekday};
use findates::algebra;
use findates::calendar;
use findates::conventions::AdjustRule;

// ============================================================================
// Business Day Tests
// ============================================================================

#[test]
fn is_business_day_test() {
    let mut basic_cal = calendar::basic_calendar();

    // Sunday should not be a business day
    let sunday = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Sun);
    assert!(!algebra::is_business_day(&sunday.unwrap(), &basic_cal));

    // Monday should be a business day
    let monday = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Mon);
    assert!(algebra::is_business_day(&monday.unwrap(), &basic_cal));

    // Christmas should be a business day before being added to holidays
    let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    assert!(algebra::is_business_day(&christmas_day, &basic_cal));

    // After adding to calendar, Christmas should not be a business day
    basic_cal.add_holidays(&[christmas_day].into_iter().collect());
    assert!(!algebra::is_business_day(&christmas_day, &basic_cal));
}

// ============================================================================
// Business Day Schedule Tests
// ============================================================================

#[test]
fn schedule_test() {
    let mut test_schedule: Vec<NaiveDate> = [].to_vec();
    let hol = NaiveDate::from_ymd_opt(2023, 9, 22).unwrap();

    // Create test vector with all the dates
    for i in 2..31 {
        let dt = NaiveDate::from_ymd_opt(2023, 9, i).unwrap();
        // Exclude weekends
        if dt.weekday() != Weekday::Sat && dt.weekday() != Weekday::Sun && dt != hol {
            test_schedule.push(dt);
        }
    }

    let mut cal = calendar::basic_calendar();
    cal.add_holidays(&[hol].into_iter().collect());

    let start_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 2).unwrap();
    let end_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    let res: Vec<NaiveDate> =
        algebra::bus_day_schedule(&start_date, &end_date, &cal, Some(AdjustRule::ModFollowing));

    assert_eq!(test_schedule, res);
}

#[test]
fn bus_days_between_test() {
    let mut test_schedule: Vec<NaiveDate> = [].to_vec();
    let hol: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 22).unwrap();

    // Create test vector with all the dates
    for i in 1..29 {
        let dt = NaiveDate::from_ymd_opt(2023, 9, i).unwrap();
        // Exclude weekends
        if dt.weekday() != Weekday::Sat && dt.weekday() != Weekday::Sun && dt != hol {
            test_schedule.push(dt);
        }
    }

    let mut cal = calendar::basic_calendar();
    cal.add_holidays(&[hol].into_iter().collect());

    let start_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 1).unwrap();
    let end_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 29).unwrap();
    let res: u64 =
        algebra::business_days_between(&start_date, &end_date, &cal, Some(AdjustRule::Preceding));

    assert_eq!(test_schedule.len() as u64, res);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn bus_day_schedule_single_day_test() {
    // start == end: the schedule should contain exactly that one business day.
    let cal = calendar::basic_calendar();
    let monday = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
    let result = algebra::bus_day_schedule(&monday, &monday, &cal, None);
    assert_eq!(result, vec![monday]);
}

#[test]
fn business_days_between_same_day_test() {
    let cal = calendar::basic_calendar();
    let monday = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
    // Same day: zero business days between (end excluded).
    assert_eq!(
        algebra::business_days_between(&monday, &monday, &cal, None),
        0
    );
}

#[test]
fn bus_day_schedule_holiday_gap_test() {
    // A week with Christmas (Wednesday) and Boxing Day (Thursday) as holidays.
    let mut cal = calendar::basic_calendar();
    let xmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    let boxing_day = NaiveDate::from_ymd_opt(2024, 12, 26).unwrap();
    cal.add_holidays(&[xmas, boxing_day].into_iter().collect());

    let start = NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(); // Monday
    let end = NaiveDate::from_ymd_opt(2024, 12, 27).unwrap(); // Friday
    let days = algebra::bus_day_schedule(&start, &end, &cal, None);
    // Mon 23, Tue 24, Fri 27 — Wed/Thu are holidays
    assert_eq!(
        days,
        vec![
            NaiveDate::from_ymd_opt(2024, 12, 23).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 24).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 27).unwrap(),
        ]
    );
}

#[test]
fn calendar_default_is_empty_test() {
    let cal = calendar::Calendar::default();
    assert!(cal.get_holidays().is_empty());
    assert!(cal.get_weekend().is_empty());
}
