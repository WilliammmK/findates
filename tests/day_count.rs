// Integration tests for day count fraction calculations.
// These tests validate day count conventions (Act/360, Act/365, Actual/Actual ISDA, 30/360, BD252)
// with and without calendar adjustments.

use chrono::NaiveDate;
use findates::algebra::day_count_fraction;
use findates::calendar;
use findates::conventions::{AdjustRule, DayCount};
use findates::DayCountError;

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
        basic_cal.add_holidays([christmas_day, boxing_day]);
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
fn dcf_act365fixed_leap_year_test() {
    // Over a full leap year (366 actual days) Act365Fixed still divides by 365,
    // so the result is 366/365, not 1.0. This distinguishes it from Act365 and
    // ActActISDA which both return 1.0 over a full leap year.
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end   = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let dcf = day_count_fraction(&start, &end, DayCount::Act365Fixed, None, None).unwrap();
    assert!((dcf - 366.0 / 365.0).abs() < 1e-9);
}

#[test]
fn dcf_act365fixed_non_leap_year_test() {
    // Over a full non-leap year (365 actual days) Act365Fixed returns exactly 1.0.
    let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let end   = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let dcf = day_count_fraction(&start, &end, DayCount::Act365Fixed, None, None).unwrap();
    assert!((dcf - 1.0).abs() < 1e-9);
}

#[test]
fn dcf_thirty360us_start_on_31st_test() {
    // Rule 1: start on 31st → treated as 30th.
    // start: 2023-01-31 → 30; end: 2023-04-15 (unchanged)
    // res = 360*0 + 30*(4-1) + (15-30) = 90-15 = 75 → 75/360
    let start = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
    let end   = NaiveDate::from_ymd_opt(2023, 4, 15).unwrap();
    let dcf = day_count_fraction(&start, &end, DayCount::Thirty360US, None, None).unwrap();
    assert!((dcf - 75.0 / 360.0).abs() < 1e-9);
}

#[test]
fn dcf_thirty360us_end_on_31st_start_on_30th_test() {
    // Rule 3: end on 31st and start is 30th → end treated as 30th.
    // start: 2023-01-30; end: 2023-03-31 → 30
    // res = 360*0 + 30*(3-1) + (30-30) = 60 → 60/360
    let start = NaiveDate::from_ymd_opt(2023, 1, 30).unwrap();
    let end   = NaiveDate::from_ymd_opt(2023, 3, 31).unwrap();
    let dcf = day_count_fraction(&start, &end, DayCount::Thirty360US, None, None).unwrap();
    assert!((dcf - 60.0 / 360.0).abs() < 1e-9);
}

#[test]
fn dcf_thirty360us_end_on_31st_start_before_30th_test() {
    // Rule 3 does NOT fire when start < 30: end stays at 31 (i.e. counts as 1st of next month).
    // start: 2023-01-15; end: 2023-03-31 (stays 31)
    // res = 360*0 + 30*(3-1) + (31-15) = 60+16 = 76 → 76/360
    // D30360Euro would give 75/360 (unconditionally treats end 31→30).
    let start = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();
    let end   = NaiveDate::from_ymd_opt(2023, 3, 31).unwrap();
    let dcf = day_count_fraction(&start, &end, DayCount::Thirty360US, None, None).unwrap();
    assert!((dcf - 76.0 / 360.0).abs() < 1e-9);
}

#[test]
fn dcf_thirty360us_start_on_eom_feb_nonleap_test() {
    // Rule 2: start on last day of February (non-leap: Feb 28) → treated as 30th.
    // start: 2023-02-28 → 30; end: 2023-06-15 (unchanged)
    // res = 360*0 + 30*(6-2) + (15-30) = 120-15 = 105 → 105/360
    let start = NaiveDate::from_ymd_opt(2023, 2, 28).unwrap();
    let end   = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
    let dcf = day_count_fraction(&start, &end, DayCount::Thirty360US, None, None).unwrap();
    assert!((dcf - 105.0 / 360.0).abs() < 1e-9);
}

#[test]
fn dcf_thirty360us_both_on_eom_feb_test() {
    // Rules 2 + 4: both dates on last day of February → both treated as 30th.
    // start: 2023-02-28 → 30; end: 2024-02-29 (leap) → 30
    // res = 360*(2024-2023) + 30*(2-2) + (30-30) = 360 → 360/360 = 1.0
    let start = NaiveDate::from_ymd_opt(2023, 2, 28).unwrap();
    let end   = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    let dcf = day_count_fraction(&start, &end, DayCount::Thirty360US, None, None).unwrap();
    assert!((dcf - 1.0).abs() < 1e-9);
}

#[test]
fn dcf_thirty360us_differs_from_d30360euro_for_february_test() {
    // Confirms Rule 2 produces a different result than D30360Euro.
    // D30360Euro does not treat Feb EOM as day 30, so start stays at 28.
    // start: 2023-02-28, end: 2023-06-15
    //   Thirty360US: start→30 → res = 105/360
    //   D30360Euro:  start=28 → res = 107/360
    let start = NaiveDate::from_ymd_opt(2023, 2, 28).unwrap();
    let end   = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
    let us  = day_count_fraction(&start, &end, DayCount::Thirty360US,  None, None).unwrap();
    let eu  = day_count_fraction(&start, &end, DayCount::D30360Euro,   None, None).unwrap();
    assert!((us  - 105.0 / 360.0).abs() < 1e-9);
    assert!((eu  - 107.0 / 360.0).abs() < 1e-9);
    assert_ne!(round_decimals(us), round_decimals(eu));
}

#[test]
fn dcf_non_bd252_conventions_return_ok_without_calendar_test() {
    let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

    for dc in [
        DayCount::Act360,
        DayCount::Act365,
        DayCount::Act365Fixed,
        DayCount::ActActISDA,
        DayCount::D30360Euro,
        DayCount::Thirty360US,
        DayCount::D30365,
    ] {
        assert!(
            day_count_fraction(&start, &end, dc, None, None).is_ok(),
            "{dc} should return Ok without a calendar",
        );
    }
}
