// Integration tests for schedule behavior.
// These tests validate schedule functionality including next-date calculations
// and schedule generation with various frequency rules and adjustments.

use chrono::{Datelike, NaiveDate};
use findates::calendar;
use findates::conventions::{AdjustRule, Frequency};
use findates::schedule::{schedule_next_adjusted, Schedule};
use std::collections::HashSet;

// Test setup with calendar and holidays
struct ScheduleSetup {
    cal: calendar::Calendar,
}

impl ScheduleSetup {
    fn new() -> Self {
        let mut basic_cal = calendar::basic_calendar();
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();
        let new_holidays: HashSet<NaiveDate> = [christmas_day, boxing_day].into_iter().collect();
        basic_cal.add_holidays(&new_holidays);
        Self { cal: basic_cal }
    }
}

// ============================================================================
// Next-Date Behavior Tests
// ============================================================================

#[test]
fn daily_next_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 29).unwrap();
    // Create a new schedule
    let sch = Schedule {
        frequency: Frequency::Daily,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::ModFollowing),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    // Even though this is falls on a next month, Next will force the next date for daily frequencies
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap());
    // With No Adjustment Rule, it will disregard the calendar and return the next date regardless if it is a good day or not.
    let sch = Schedule {
        frequency: Frequency::Daily,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 9, 30).unwrap()); // This is a Saturday.
                                                                    // Preceding Rule will also force the next day to be output for daily frequencies.
    let sch = Schedule {
        frequency: Frequency::Daily,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::Preceding),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap());
    // Or even for nearest
    let sch = Schedule {
        frequency: Frequency::Daily,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::Nearest),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap());
}

#[test]
fn weekly_next_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 29).unwrap();
    // Create a new weekly schedule
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    // Test for no adjustment, it should always return a date with the same weekday.
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(anchor.weekday(), res.weekday());
    // Even with an Adjustment rule, it is a Friday so weekday should be the same
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::Preceding),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(anchor.weekday(), res.weekday());
    // If the next date falls on a holiday, it will be adjusted according to adjust Rule
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::Preceding),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 22).unwrap()); // This is a Friday
                                                                     // Adjusting with Following
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::Following),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 27).unwrap());
    // With no calendar
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: None,
        adjust_rule: Some(AdjustRule::Following),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 25).unwrap());
    assert_eq!(anchor.weekday(), res.weekday());
    // A Start date will not be adjusted, but the next date will
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap(); // Boxing day
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::ModFollowing),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(res, NaiveDate::from_ymd_opt(2024, 1, 2).unwrap());
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 23).unwrap(); // Saturday
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::ModFollowing),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 29).unwrap());
}

#[test]
fn biweekly_next_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    // Create a new weekly schedule
    let sch = Schedule {
        frequency: Frequency::Biweekly,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    // Test for no adjustment, it should always return a date with the same weekday.
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(anchor.weekday(), res.weekday());
    // With adjustment
    let sch = Schedule {
        frequency: Frequency::Biweekly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::Nearest),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_ne!(anchor.weekday(), res.weekday());
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 13).unwrap());
}

#[test]
fn fourweeks_next_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    // Create a new weekly schedule
    let sch = Schedule {
        frequency: Frequency::EveryFourthWeek,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    // Test for no adjustment, it should always return a date with the same weekday.
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(anchor.weekday(), res.weekday());
    // With adjustment
    let sch = Schedule {
        frequency: Frequency::EveryFourthWeek,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::Nearest),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_ne!(anchor.weekday(), res.weekday());
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 27).unwrap());
}

#[test]
fn monthly_next_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    // Create a new weekly schedule
    let sch = Schedule {
        frequency: Frequency::Monthly,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    // Test for no adjustment, it should always return a date with the same day.
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(anchor.day(), res.day());
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 30).unwrap());
    // Even with no adjustment, a 31st will return a 30th.
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 31).unwrap();
    let res = schedule_next_adjusted(&sch, anchor);
    assert_ne!(anchor.day(), res.day());
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 11, 30).unwrap());
    // Now with an adjustment
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();
    let sch = Schedule {
        frequency: Frequency::Monthly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::ModPreceding),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_ne!(anchor.day(), res.day());
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 3, 17).unwrap());
}

#[test]
fn bimonthly_next_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    // Create a new weekly schedule
    let sch = Schedule {
        frequency: Frequency::Bimonthly,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    // Test for no adjustment, it should always return a date with the same day.
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(anchor.day(), res.day());
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 11, 30).unwrap());
    // No adjustment, a 31st will return a 31st.
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 31).unwrap();
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(anchor.day(), res.day());
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    // Now with an adjustment
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();
    let sch = Schedule {
        frequency: Frequency::Bimonthly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::ModPreceding),
    };
    let res = schedule_next_adjusted(&sch, anchor);
    assert_eq!(anchor.day(), res.day());
    assert_eq!(res, NaiveDate::from_ymd_opt(2023, 4, 18).unwrap());
}

// ============================================================================
// Schedule Generator Tests
// ============================================================================

#[test]
fn daily_generator_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 9).unwrap();
    // Create a new schedule
    let sch = Schedule {
        frequency: Frequency::Daily,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    let dates = sch.generate(&anchor, &end).unwrap();
    // No adjustment, so expected is all dates from anchor to end.
    let dates_str = [
        "2023-09-30",
        "2023-10-01",
        "2023-10-02",
        "2023-10-03",
        "2023-10-04",
        "2023-10-05",
        "2023-10-06",
        "2023-10-07",
        "2023-10-08",
        "2023-10-09",
    ];
    let expected_dates: Vec<NaiveDate> = dates_str
        .into_iter()
        .map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap())
        .collect();
    assert_eq!(expected_dates, dates);
    // With an adjustment rule
    let sch = Schedule {
        frequency: Frequency::Daily,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::ModPreceding),
    };
    let dates = sch.generate(&anchor, &end).unwrap();
    let dates_str = [
        "2023-09-29",
        "2023-10-02",
        "2023-10-03",
        "2023-10-04",
        "2023-10-05",
        "2023-10-06",
        "2023-10-09",
    ];
    let expected_dates: Vec<NaiveDate> = dates_str
        .into_iter()
        .map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap())
        .collect();
    assert_eq!(expected_dates, dates);
}

#[test]
fn weekly_generator_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 11, 5).unwrap();
    // Create a new schedule
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    let dates = sch.generate(&anchor, &end).unwrap();
    // No adjustment, so expected is all dates from anchor to end.
    let dates_str = [
        "2023-09-30",
        "2023-10-07",
        "2023-10-14",
        "2023-10-21",
        "2023-10-28",
        "2023-11-04",
    ];
    let expected_dates: Vec<NaiveDate> = dates_str
        .into_iter()
        .map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap())
        .collect();
    assert_eq!(expected_dates, dates);
    // With an adjustment rule
    let sch = Schedule {
        frequency: Frequency::Weekly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::ModFollowing),
    };
    let dates = sch.generate(&anchor, &end).unwrap();
    let dates_str = [
        "2023-09-29",
        "2023-10-09",
        "2023-10-16",
        "2023-10-23",
        "2023-10-30",
        "2023-11-06",
    ];
    let expected_dates: Vec<NaiveDate> = dates_str
        .into_iter()
        .map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap())
        .collect();
    assert_eq!(expected_dates, dates);
}

#[test]
fn biweekly_generator_test() {
    let setup = ScheduleSetup::new();
    let cal = setup.cal;
    let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 11, 26).unwrap();
    let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 5).unwrap();
    // Create a new schedule
    let sch = Schedule {
        frequency: Frequency::Biweekly,
        calendar: Some(&cal),
        adjust_rule: None,
    };
    let dates = sch.generate(&anchor, &end).unwrap();
    // No adjustment, so expected is all dates from anchor to end.
    let dates_str = [
        "2023-11-26",
        "2023-12-10",
        "2023-12-24",
        "2024-01-07",
        "2024-01-21",
        "2024-02-04",
    ];
    let expected_dates: Vec<NaiveDate> = dates_str
        .into_iter()
        .map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap())
        .collect();
    assert_eq!(expected_dates, dates);
    // With an adjustment rule
    let sch = Schedule {
        frequency: Frequency::Biweekly,
        calendar: Some(&cal),
        adjust_rule: Some(AdjustRule::ModPreceding),
    };
    let dates = sch.generate(&anchor, &end).unwrap();
    let dates_str = [
        "2023-11-24",
        "2023-12-08",
        "2023-12-22",
        "2024-01-05",
        "2024-01-19",
        "2024-02-02",
    ];
    let expected_dates: Vec<NaiveDate> = dates_str
        .into_iter()
        .map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap())
        .collect();
    assert_eq!(expected_dates, dates);
}
