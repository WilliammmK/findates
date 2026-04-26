// Integration tests for day count fraction calculations.
// These tests validate day count conventions (Act/360, Act/365, Actual/Actual ISDA, 30/360, BD252)
// with and without calendar adjustments.

use chrono::NaiveDate;
use findates::algebra::day_count_fraction;
use findates::calendar;
use findates::conventions::{AdjustRule, DayCount};
use findates::DayCountError;
use std::collections::HashSet;

fn round_decimals(x: f64) -> f64 {
    let multiplier = 100000.0;
    (x * multiplier).round() / multiplier
}

struct DayCountSetup {
    cal: calendar::Calendar,
}

impl DayCountSetup {
    fn new() -> Self {
        let mut basic_cal = calendar::basic_calendar();
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();
        let new_holidays: HashSet<NaiveDate> = [christmas_day, boxing_day].into_iter().collect();
        basic_cal.add_holidays(&new_holidays);
        Self { cal: basic_cal }
    }
}

#[test]
fn dcf_act360_test() {
    let setup = DayCountSetup::new();
    let cal = setup.cal;
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    let expected: f64 = 0.6305556;
    let res: f64 = day_count_fraction(&start, &end, DayCount::Act360, None, None).unwrap();
    // No calendar
    assert_eq!(round_decimals(res), round_decimals(expected));
    // With Calendar
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap(); // Adjusted to 02 Oct
    let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(); // Adjusted to 27 Dec
    let expected: f64 = 0.2388889;
    let res: f64 = day_count_fraction(
        &start,
        &end,
        DayCount::Act360,
        Some(&cal),
        Some(AdjustRule::Following),
    ).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));
}

#[test]
fn dcf_act365_test() {
    let setup = DayCountSetup::new();
    let cal = setup.cal;
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    let expected: f64 = 0.62191781;
    let res: f64 = day_count_fraction(&start, &end, DayCount::Act365, None, None).unwrap();
    // No calendar
    assert_eq!(round_decimals(res), round_decimals(expected));
    // With Calendar
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap(); // Adjusted to 02 Oct
    let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(); // Adjusted to 27 Dec
    let expected: f64 = 0.23561644;
    let res: f64 = day_count_fraction(
        &start,
        &end,
        DayCount::Act365,
        Some(&cal),
        Some(AdjustRule::Following),
    ).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));
}

#[test]
fn dcf_actactisda_test() {
    // The relevant test cases for this convention are when either or
    // both start date and end dates fall within a leap year.

    // Both dates within a leap year
    let setup = DayCountSetup::new();
    let cal = setup.cal;
    let start: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 5, 27).unwrap(); // This is a Monday within a Leap year
    let expected: f64 = 0.27868852;
    let res: f64 = day_count_fraction(&start, &end, DayCount::ActActISDA, Some(&cal), None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));

    // Both dates within a non-leap year
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 5, 27).unwrap(); // This will get adjusted to 29May2023
    let expected: f64 = 0.28219178;
    let res: f64 = day_count_fraction(&start, &end, DayCount::ActActISDA, Some(&cal), None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));

    // End date only within a leap year
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 5, 27).unwrap(); // This is a Monday in a Leap Year
    let expected: f64 = 1.27835167;
    let res: f64 = day_count_fraction(&start, &end, DayCount::ActActISDA, Some(&cal), None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));

    // Start date and end dates within a leap year
    let start: NaiveDate = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap(); // This is a Saturday, will get adjusted to 2nd of March
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 5, 27).unwrap(); // This is a Monday in a Leap Year
    let expected: f64 = 4.23497268;
    let res: f64 = day_count_fraction(&start, &end, DayCount::ActActISDA, Some(&cal), None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));
}

#[test]
fn dcf_d30360euro_test() {
    // Start date on the 31st
    let setup = DayCountSetup::new();
    let cal = setup.cal;
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(); // This is a Monday within a Leap year
    let expected: f64 = 1.04166667;
    let res: f64 = day_count_fraction(&start, &end, DayCount::D30360Euro, Some(&cal), None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));
    // End date on the 31st
    let start: NaiveDate = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(); // Although this is a 31st, it is a Sunday so will get adjusted to Following first,
                                                                          // since we are passing a calendar.
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 10, 31).unwrap();
    let expected: f64 = 0.5805556;
    let res: f64 = day_count_fraction(&start, &end, DayCount::D30360Euro, Some(&cal), None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));
    // Same dates but passing no calendar, i.e. no adjustment:
    let expected: f64 = 0.583333;
    let res: f64 = day_count_fraction(&start, &end, DayCount::D30360Euro, None, None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));
}

#[test]
fn dcf_d30365_test() {
    let setup = DayCountSetup::new();
    let cal = setup.cal;
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 24).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(); // This is a Monday within a Leap year
    let expected: f64 = 1.04383562;
    let res: f64 = day_count_fraction(&start, &end, DayCount::D30365, Some(&cal), None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));
}

#[test]
fn dcf_bd252_test() {
    // For a Business Day Calendar, the relevant test cases should
    // of course take into account Holidays and check if the business
    // days are being properly counted.
    let setup = DayCountSetup::new();
    let cal = setup.cal;
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 24).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();
    let expected: f64 = 1.09126984;
    let res: f64 = day_count_fraction(&start, &end, DayCount::Bd252, Some(&cal), None).unwrap();
    assert_eq!(round_decimals(res), round_decimals(expected));
    // Test case with an adjustment on the end date
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 24).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 23).unwrap(); // This will get adjusted to the 27th of Dec
    let end2: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 27).unwrap(); // This is a business day so won't be adjusted
    let res: f64 = day_count_fraction(&start, &end, DayCount::Bd252, Some(&cal), None).unwrap();
    let res2: f64 = day_count_fraction(&start, &end2, DayCount::Bd252, Some(&cal), None).unwrap();
    // Business day count for both end dates above should be the same
    assert_eq!(round_decimals(res), round_decimals(res2));
    // But if we pass a Preceding adjustment they should differ
    let res: f64 = day_count_fraction(
        &start,
        &end,
        DayCount::Bd252,
        Some(&cal),
        Some(AdjustRule::Preceding),
    ).unwrap();
    let res2: f64 = day_count_fraction(
        &start,
        &end2,
        DayCount::Bd252,
        Some(&cal),
        Some(AdjustRule::Preceding),
    ).unwrap();
    assert_ne!(round_decimals(res), round_decimals(res2));
    let expected: f64 = 0.94444444;
    assert_eq!(round_decimals(res), round_decimals(expected));
}

#[test]
fn dcf_bd252_no_calendar_returns_err_test() {
    let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 24).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();
    assert_eq!(
        day_count_fraction(&start, &end, DayCount::Bd252, None, None),
        Err(DayCountError::MissingCalendar),
    );
}

#[test]
fn dcf_bd252_with_calendar_returns_ok_test() {
    let cal = calendar::basic_calendar();
    let start = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 3, 22).unwrap();
    assert!(day_count_fraction(&start, &end, DayCount::Bd252, Some(&cal), None).is_ok());
}

#[test]
fn dcf_non_bd252_conventions_return_ok_without_calendar_test() {
    let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

    for dc in [
        DayCount::Act360,
        DayCount::Act365,
        DayCount::ActActISDA,
        DayCount::D30360Euro,
        DayCount::D30365,
    ] {
        assert!(
            day_count_fraction(&start, &end, dc, None, None).is_ok(),
            "{dc} should return Ok without a calendar",
        );
    }
}
