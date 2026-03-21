// Integration tests for schedule next-date behavior.
// These tests validate the next date calculation for various frequency rules
// with and without calendar adjustments.

use std::collections::HashSet;
use chrono::{NaiveDate, Datelike};
use findates::calendar;
use findates::conventions::{Frequency, AdjustRule};
use findates::schedule::{Schedule, schedule_next_adjusted};

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
