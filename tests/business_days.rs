// Integration tests for business day determination.
// These tests validate whether a given date is a business day
// based on calendar rules (weekends and holidays).

use chrono::{NaiveDate, Weekday};
use findates::algebra;
use findates::calendar;

#[test]
fn is_business_day_test() {
    let mut basic_cal = calendar::basic_calendar();

    // Sunday should not be a business day
    let sunday = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Sun);
    assert_eq!(
        false,
        algebra::is_business_day(&sunday.unwrap(), &basic_cal)
    );

    // Monday should be a business day
    let monday = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Mon);
    assert_eq!(true, algebra::is_business_day(&monday.unwrap(), &basic_cal));

    // Christmas should be a business day before being added to holidays
    let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    assert_eq!(true, algebra::is_business_day(&christmas_day, &basic_cal));

    // After adding to calendar, Christmas should not be a business day
    basic_cal.add_holidays(&[christmas_day].into_iter().collect());
    assert_eq!(false, algebra::is_business_day(&christmas_day, &basic_cal));
}
