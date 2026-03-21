// Integration tests for business day schedule generation.
// These tests validate schedule generation and business day counting
// based on calendar rules and adjustment conventions.

use chrono::{Weekday, NaiveDate, Datelike};
use findates::algebra;
use findates::calendar;
use findates::conventions::AdjustRule;

#[test]
fn schedule_test() {
    let mut test_schedule: Vec<NaiveDate> = [].to_vec();
    let hol = NaiveDate::from_ymd_opt(2023, 9, 22).unwrap();
    
    // Create test vector with all the dates
    for i in 2..31 {
        let dt = NaiveDate::from_ymd_opt(2023, 9, i).unwrap();
        // Exclude weekends
        if dt.weekday() == Weekday::Sat || dt.weekday() == Weekday::Sun {
        }
        // Include a Holiday
        else if dt == hol {
        } else {
            test_schedule.push(dt)
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
        if dt.weekday() == Weekday::Sat || dt.weekday() == Weekday::Sun {
        }
        // Include a Holiday
        else if dt == hol {
        } else {
            test_schedule.push(dt)
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
