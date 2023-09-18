//! Schedules
//! The output here can come from both methods or free functions.

use chrono::{NaiveDate, Duration, Months};

use crate::calendar::Calendar;
use crate::conventions::{AdjustRule,DayCount, DateUnit, Frequency, Tenor};
use crate::algebra;


/// A Schedule with an Anchor date
/// 
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule<'a> {
    pub frequency: Frequency,
    pub calendar: Option<&'a Calendar>,
    pub adjust_rule: Option<AdjustRule>,
}


/// Associated Schedule functions
impl<'a> Schedule<'a> {

    /// Create a new Schedule with a Frequency, Calendar and Adjust Rule
    pub fn new (frequency: Frequency, opt_calendar: Option<&'a Calendar>, opt_adjust_rule: Option<AdjustRule>) -> Self {
        Self {frequency:frequency, calendar:opt_calendar, adjust_rule: opt_adjust_rule}
    }

    /// Create an iterator as a method
    pub fn iter (&self, anchor: NaiveDate) -> ScheduleIterator {
        ScheduleIterator { schedule: self, anchor: anchor }
    }

    /// Generate a vector of dates for a given schedule with a start and an end date
    pub fn generate (&self, anchor_date: NaiveDate, end_date: NaiveDate ) -> Result<Vec<NaiveDate>, &'static str> {
        // Check input dates       
        if end_date <= anchor_date {
            return  Err("Anchor date must be before end date");
        } 
        // Use the iterator to collect into a Vec
        else {
            let res: Vec<NaiveDate>;
            let iter = self.iter(anchor_date);
            res = iter.take_while(|x| x < &end_date).collect();
            return Ok(res);
        }
    }
    
}


// The function below will add the specified duration to an
// anchor date and adjust it to a working day according to the 
// given calendar and adjust rule.
fn force_add_duration_adjust ( anchor_date: &NaiveDate, delta: Duration, opt_calendar: Option<&Calendar>
                       , opt_adjust_rule: Option<AdjustRule>) -> NaiveDate {    
    let mut res: NaiveDate;
    
    res = anchor_date.checked_add_signed(delta).unwrap_or_else(|| {
        panic!("Next Date for {} frequency is out of bounds, check chrono internals for the last date available", Frequency::Weekly);
    });
    res = algebra::adjust(&res, opt_calendar, opt_adjust_rule);
    // Case where the adjustment brings the date back to the same as the anchor
    if res <= *anchor_date {
        let mut dayi = 1;
        while res <= *anchor_date {
            res = anchor_date.checked_add_signed(Duration::days(dayi)).unwrap_or_else(|| {
                panic!("Next Date for {} frequency is out of bounds, check chrono internals for the last date available", Frequency::Weekly);
            });
            dayi += 1;
            res = algebra::adjust(&res, opt_calendar, opt_adjust_rule);
        }
    }
    return res; 
}

// There is no months Duration in chrono for some reason
// so using separate function whenever the unit is a month
fn force_add_months_adjust ( anchor_date: &NaiveDate, delta: Months, opt_calendar: Option<&Calendar>
                       , opt_adjust_rule: Option<AdjustRule>) -> NaiveDate {    
    let mut res: NaiveDate;
    
    res = anchor_date.checked_add_months(delta).unwrap_or_else(|| {
        panic!("Next Date for {} frequency is out of bounds, check chrono internals for the last date available", Frequency::Weekly);
    });
    res = algebra::adjust(&res, opt_calendar, opt_adjust_rule);
    // The corner case of a Months's long holiday
    if res <= *anchor_date {
        let mut dayi = 1;
        while res <= *anchor_date {
            res = anchor_date.checked_add_signed(Duration::days(dayi)).unwrap_or_else(|| {
                panic!("Next Date for {} frequency is out of bounds, check chrono internals for the last date available", Frequency::Weekly);
            });
            dayi += 1;
            res = algebra::adjust(&res, opt_calendar, opt_adjust_rule);
        }
    }
    return res; 
}
                       


// Gets the next date given an anchor date, a schedule and
// a frequency. The function will not adjust the anchor date,
// but it will adjust the next date if a calendar and adjust rule is passed.
pub fn schedule_next ( anchor_date: &NaiveDate, frequency: Frequency
                      , opt_calendar: Option<&Calendar>, opt_adjust_rule: Option<AdjustRule>) -> NaiveDate {
    
    // Calculate next for each of the Frequencies.
    match frequency {
        Frequency::Daily => {
            // For the case of Preceding, ModFollowing, Nearest, etc it will keep giving 
            // the function might simply return the same as anchor date after adjustment.
            // The loop below forces that the returned dat
            // Should only be an issue for the Daily Frequency.
            let delta = Duration::days(1);
            return force_add_duration_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule)
        },
        
        Frequency::Weekly => {
            let delta = Duration::weeks(1);
            return force_add_duration_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule)
        },
        
        Frequency::Biweekly => {
            let delta = Duration::weeks(2);
            return force_add_duration_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule)
        },

        Frequency::EveryFourthWeek => {
            let delta = Duration::weeks(4);
            return force_add_duration_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule)
        },

        Frequency::Monthly => {
            // There is no months Duration, so using Months struct from Chrono
            let delta = Months::new(1);
            return force_add_months_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule);
        },

        Frequency::Bimonthly => {
            let delta = Months::new(2);
            return force_add_months_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule);
        },

        Frequency::Quarterly => {
            let delta = Months::new(3);
            return force_add_months_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule);
        },

        Frequency::EveryFourthMonth => {
            let delta = Months::new(4);
            return force_add_months_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule);
        },

        Frequency::Semiannual => {
            let delta = Months::new(6);
            return force_add_months_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule);
        },

        Frequency::Annual => {
            let delta = Months::new(12);
            return force_add_months_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule);
        },

        Frequency::Once => {return *anchor_date;}

    }


}


/// Iterator over dates of a schedule.
/// This is an unbounded
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleIterator<'a> {
    schedule: &'a Schedule<'a>,
    anchor: NaiveDate
}

impl<'a> ScheduleIterator<'a> {
    pub fn new (schedule: &'a Schedule<'a>, anchor: NaiveDate) -> Self {
        Self {schedule: schedule, anchor: anchor}
    }
}

impl<'a> Iterator for ScheduleIterator<'a> {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        let res = schedule_iterator_next(&mut self.schedule, self.anchor);
        self.anchor = res.unwrap();
        return res;
    }
}

// Next function for the Schedule iterator
fn schedule_iterator_next<'a> (schedule: & Schedule, anchor: NaiveDate) -> Option<NaiveDate> {
    
   Some( schedule_next(&anchor, schedule.frequency, schedule.calendar, schedule.adjust_rule))    
    
}


/// Unit Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use chrono::{NaiveDate, Datelike};
    use crate::calendar as c;
    use crate::conventions::{Frequency, AdjustRule };
    use crate::schedule::ScheduleIterator;
    use super::{Schedule, schedule_next};

        // Setup for variables to be used in multiples tests
        struct Setup {
            cal: c::Calendar,
            _test_weekend: NaiveDate,
            _test_holiday: NaiveDate
        }
        // Setup constructor
        impl Setup {
            fn  new() -> Self {
                let mut basic_cal: c::Calendar = c::basic_calendar();
                let christmas_day = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
                let boxing_day = NaiveDate::from_ymd_opt(2023,12,26).unwrap();
                let new_holidays: HashSet<NaiveDate> =  [christmas_day, boxing_day].into_iter().collect();
                let test_weekend: NaiveDate = NaiveDate::from_ymd_opt(2023, 9,2).unwrap(); // this is a Saturday
                basic_cal.add_holidays(&new_holidays);
                Self { 
                        cal : basic_cal,
                        _test_holiday: christmas_day,
                        _test_weekend: test_weekend
                }
            }
        }


    // Schedule Generator tests
    // Daily Frequency test
    #[test]
    fn daily_next_test () {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 29).unwrap();
        // Create a new schedule
        let sch = Schedule {frequency: Frequency::Daily, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModFollowing)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        // Even though this is falls on a next month, Next will force the next date for daily frequencies
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap() );
        // With No Adjustment Rule, it will disregard the calendar and return the next date regardless if it is a good day or not.
        let sch = Schedule {frequency: Frequency::Daily, calendar: Some(&cal), adjust_rule: None};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 9, 30).unwrap() ); // This is a Saturday.
        // Preceding Rule will also force the next day to be output for daily frequencies.
        let sch = Schedule {frequency: Frequency::Daily, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Preceding)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap() );
        // Or even for nearest
        let sch = Schedule {frequency: Frequency::Daily, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Nearest)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap() );
        // let iter = ScheduleIterator {schedule: sch, anchor: anchor}.clone(); 
        // println!("ITERATING NOW:");
        // for dayit in iter.clone() {
        //     if dayit > NaiveDate::from_ymd_opt(2023, 10, 15).unwrap() { break;}
        //     else {
        //         println!("{}", dayit);
        //     }
        // }
        // let new_iter = iter.map(|x| NaiveDate::checked_add_days(x, chrono::Days::new(1)).unwrap());
        // println!("MAP ITERATING NOW:");
        // for dayit in new_iter.clone() {
        //     if dayit > NaiveDate::from_ymd_opt(2023, 10, 15).unwrap() { break;}
        //     else {
        //         println!("{}", dayit);
        //     }
        // }

    }



    // Weekly Frequency test
    #[test]
    fn weekly_next_test () {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 29).unwrap();
        // Create a new weekly schedule
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: None};
        // Test for no adjustment, it should always return a date with the same weekday.
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(anchor.weekday(), res.weekday());
        // Even with an Adjustment rule, it is a Friday so weekday should be the same
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Preceding)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(anchor.weekday(), res.weekday());
        // If the next date falls on a holiday, it will be adjusted according to adjust Rule
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Preceding)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 22).unwrap() ); // This is a Friday
        // Adjusting with Following
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Following)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 27).unwrap() );
        // With no calendar
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
        let sch = Schedule {frequency: Frequency::Weekly, calendar: None, adjust_rule: Some(AdjustRule::Following)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 25).unwrap() );
        assert_eq!(anchor.weekday(), res.weekday());
        // A Start date will not be adjusted, but the next date will
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap(); // Boxing day
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModFollowing)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2024, 1, 2).unwrap());
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 23).unwrap(); // Saturday
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModFollowing)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 29).unwrap() );


    }

    // Biweekly Frequency test
    #[test]
    fn biweekly_next_test () {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        // Create a new weekly schedule
        let sch = Schedule {frequency: Frequency::Biweekly, calendar: Some(&cal), adjust_rule: None};
        // Test for no adjustment, it should always return a date with the same weekday.
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(anchor.weekday(), res.weekday());
        // With adjustment
        let sch = Schedule {frequency: Frequency::Biweekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Nearest)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_ne!(anchor.weekday(), res.weekday());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 13).unwrap());
    }

    // EveryFourWeeks Frequency test
    #[test]
    fn fourweeks_next_test () {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        // Create a new weekly schedule
        let sch = Schedule {frequency: Frequency::EveryFourthWeek, calendar: Some(&cal), adjust_rule: None};
        // Test for no adjustment, it should always return a date with the same weekday.
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(anchor.weekday(), res.weekday());
        // With adjustment
        let sch = Schedule {frequency: Frequency::EveryFourthWeek, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Nearest)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_ne!(anchor.weekday(), res.weekday());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 27).unwrap());
    }

    // Monthly Frequency test
    #[test]
    fn monthly_next_test () {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        // Create a new weekly schedule
        let sch = Schedule {frequency: Frequency::Monthly, calendar: Some(&cal), adjust_rule: None};
        // Test for no adjustment, it should always return a date with the same day.
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(anchor.day(), res.day());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 30).unwrap());
        // Even with no adjustment, a 31st will return a 30th.
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 31).unwrap();
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_ne!(anchor.day(), res.day());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 11, 30).unwrap());
        // Now with an adjustment
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();
        let sch = Schedule {frequency: Frequency::Monthly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModPreceding)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_ne!(anchor.day(), res.day());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 3, 17).unwrap());        
    }

    // BiMonthly Frequency test
    #[test]
    fn bimonthly_next_test () {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        // Create a new weekly schedule
        let sch = Schedule {frequency: Frequency::Bimonthly, calendar: Some(&cal), adjust_rule: None};
        // Test for no adjustment, it should always return a date with the same day.
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(anchor.day(), res.day());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 11, 30).unwrap());
        // No adjustment, a 31st will return a 31st.
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 31).unwrap();
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(anchor.day(), res.day());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
        // Now with an adjustment
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 18).unwrap();
        let sch = Schedule {frequency: Frequency::Bimonthly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModPreceding)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(anchor.day(), res.day());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 4, 18).unwrap());        
    }


    // Schedule Iterator test
    #[test]
    fn schedule_iterator_test () {


    }




}