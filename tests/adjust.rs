// Integration tests for date adjustment rules.
// These tests validate the behavior of different business day adjustment conventions
// applied to dates according to a calendar's holidays and weekends.

use chrono::{Datelike, Days, NaiveDate, Weekday};
use findates::algebra;
use findates::calendar;
use findates::conventions::AdjustRule;

mod setup;
use setup::AdjustSetup;

#[test]
fn adjust_following_test() {
    let setup = AdjustSetup::new();
    let cal = setup.cal;
    assert_eq!(
        algebra::adjust(&setup.test_weekend, Some(&cal), Some(AdjustRule::Following)),
        NaiveDate::from_ymd_opt(2023, 9, 4).unwrap()
    );
    assert_eq!(
        algebra::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Following)),
        NaiveDate::from_ymd_opt(2023, 12, 27).unwrap()
    );
    assert_ne!(
        algebra::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Following)),
        NaiveDate::from_ymd_opt(2023, 12, 26).unwrap()
    );
}

#[test]
fn adjust_preceding_test() {
    let setup = AdjustSetup::new();
    let cal = setup.cal;
    let sunday = setup.test_weekend.checked_add_days(Days::new(1)).unwrap();
    assert_eq!(sunday.weekday(), Weekday::Sun);
    assert_eq!(
        algebra::adjust(&sunday, Some(&cal), Some(AdjustRule::Preceding)),
        NaiveDate::from_ymd_opt(2023, 9, 1).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &setup.test_holiday.checked_add_days(Days::new(1)).unwrap(),
            Some(&cal),
            Some(AdjustRule::Preceding)
        ),
        NaiveDate::from_ymd_opt(2023, 12, 22).unwrap()
    );
    assert_ne!(
        algebra::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Preceding)),
        NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
    );
}

#[test]
fn adjust_modfollowing_test() {
    let setup = AdjustSetup::new();
    let cal = setup.cal;
    let eom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    assert_eq!(
        algebra::adjust(&eom, Some(&cal), Some(AdjustRule::ModFollowing)),
        NaiveDate::from_ymd_opt(2023, 9, 29).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &setup.test_weekend,
            Some(&cal),
            Some(AdjustRule::ModFollowing)
        ),
        NaiveDate::from_ymd_opt(2023, 9, 4).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &setup.test_holiday,
            Some(&cal),
            Some(AdjustRule::ModFollowing)
        ),
        NaiveDate::from_ymd_opt(2023, 12, 27).unwrap()
    );
}

#[test]
fn adjust_modpreceding_test() {
    let setup = AdjustSetup::new();
    let mut cal = setup.cal;
    cal.add_holidays(
        &[NaiveDate::from_ymd_opt(2023, 2, 1).unwrap()]
            .into_iter()
            .collect(),
    );
    let bom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 1).unwrap();
    let boy: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    assert_eq!(
        algebra::adjust(&bom, Some(&cal), Some(AdjustRule::ModPreceding)),
        NaiveDate::from_ymd_opt(2023, 9, 1).unwrap()
    );
    assert_eq!(
        algebra::adjust(&boy, Some(&cal), Some(AdjustRule::ModPreceding)),
        NaiveDate::from_ymd_opt(2023, 1, 2).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(),
            Some(&cal),
            Some(AdjustRule::ModPreceding)
        ),
        NaiveDate::from_ymd_opt(2023, 2, 2).unwrap()
    );
}

#[test]
fn adjust_halfmonthmodfollowing_test() {
    let setup = AdjustSetup::new();
    let mut cal = setup.cal;
    let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
    cal.add_holidays(&[new_hol].into_iter().collect());
    let eom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // Saturday
    assert_eq!(
        algebra::adjust(
            &setup.test_weekend,
            Some(&cal),
            Some(AdjustRule::HalfMonthModFollowing)
        ),
        NaiveDate::from_ymd_opt(2023, 9, 4).unwrap()
    );
    assert_eq!(
        algebra::adjust(&eom, Some(&cal), Some(AdjustRule::HalfMonthModFollowing)),
        NaiveDate::from_ymd_opt(2023, 9, 29).unwrap()
    );
    assert_eq!(
        algebra::adjust(&mom, Some(&cal), Some(AdjustRule::HalfMonthModFollowing)),
        NaiveDate::from_ymd_opt(2023, 1, 13).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &new_hol,
            Some(&cal),
            Some(AdjustRule::HalfMonthModFollowing)
        ),
        NaiveDate::from_ymd_opt(2023, 2, 14).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(),
            Some(&cal),
            Some(AdjustRule::ModPreceding)
        ),
        NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()
    );
}

#[test]
fn adjust_nearest_test() {
    let setup = AdjustSetup::new();
    let mut cal = setup.cal;
    let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
    cal.add_holidays(&[new_hol].into_iter().collect());
    let bom: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
    let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // Saturday
    assert_eq!(
        algebra::adjust(&bom, Some(&cal), Some(AdjustRule::Nearest)),
        NaiveDate::from_ymd_opt(2023, 10, 2).unwrap()
    );
    assert_eq!(
        algebra::adjust(&mom, Some(&cal), Some(AdjustRule::Nearest)),
        NaiveDate::from_ymd_opt(2023, 1, 13).unwrap()
    );
    assert_eq!(
        algebra::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Nearest)),
        NaiveDate::from_ymd_opt(2023, 12, 27).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(),
            Some(&cal),
            Some(AdjustRule::Nearest)
        ),
        NaiveDate::from_ymd_opt(2023, 12, 22).unwrap()
    );
}

#[test]
fn adjust_unadjusted_test() {
    let setup = AdjustSetup::new();
    let mut cal = setup.cal;
    let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
    cal.add_holidays(&[new_hol].into_iter().collect());
    let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // Saturday
    assert_eq!(
        algebra::adjust(&new_hol, Some(&cal), Some(AdjustRule::Unadjusted)),
        NaiveDate::from_ymd_opt(2023, 2, 15).unwrap()
    );
    assert_eq!(
        algebra::adjust(&mom, Some(&cal), Some(AdjustRule::Unadjusted)),
        NaiveDate::from_ymd_opt(2023, 1, 14).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &setup.test_holiday,
            Some(&cal),
            Some(AdjustRule::Unadjusted)
        ),
        NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
    );
    assert_eq!(
        algebra::adjust(
            &NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(),
            Some(&cal),
            Some(AdjustRule::Unadjusted)
        ),
        NaiveDate::from_ymd_opt(2023, 12, 24).unwrap()
    );
}

// ============================================================================
// Year Arithmetic Tests
// ============================================================================

#[test]
fn checked_add_years_forward_test() {
    let d = NaiveDate::from_ymd_opt(2023, 8, 15).unwrap();
    assert_eq!(
        algebra::checked_add_years(&d, 1),
        NaiveDate::from_ymd_opt(2024, 8, 15)
    );
    assert_eq!(
        algebra::checked_add_years(&d, 10),
        NaiveDate::from_ymd_opt(2033, 8, 15)
    );
}

#[test]
fn checked_add_years_backward_test() {
    let d = NaiveDate::from_ymd_opt(2023, 8, 15).unwrap();
    assert_eq!(
        algebra::checked_add_years(&d, -1),
        NaiveDate::from_ymd_opt(2022, 8, 15)
    );
}

#[test]
fn checked_add_years_feb29_leap_to_nonleap_test() {
    // Feb 29 in a leap year cannot be added to a non-leap year.
    let leap_day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    assert!(algebra::checked_add_years(&leap_day, 1).is_none()); // 2025 is not a leap year
    assert!(algebra::checked_add_years(&leap_day, 4).is_some()); // 2028 is a leap year
}

#[test]
fn adjust_unadjusted_holiday_unchanged_test() {
    let mut cal = calendar::basic_calendar();
    let xmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    cal.add_holidays(&[xmas].into_iter().collect());
    let result = algebra::adjust(&xmas, Some(&cal), Some(AdjustRule::Unadjusted));
    assert_eq!(result, xmas);
}
