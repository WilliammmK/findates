//! Schedules
//! The output here can come from both methods or free functions.

use std::collections::HashSet;

use chrono::{NaiveDate, Duration, Months, Datelike, Days};

use crate::calendar::Calendar;
use crate::conventions::{AdjustRule, Frequency};
use crate::algebra::{self, adjust, checked_add_years};


/// A Schedule
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

    /// Generate a vector of dates for a given schedule with a start and an end date, including both.
    pub fn generate (&self, anchor_date: &NaiveDate, end_date: &NaiveDate ) -> Result<HashSet<NaiveDate>, &'static str> {
        // Check input dates       
        if end_date <= anchor_date {
            return  Err("Anchor date must be before end date");
        } 
        // Use the iterator to collect into a Vec
        else {
            let res: HashSet<NaiveDate>;
            let iter = self.iter(*anchor_date);
            res =  iter.take_while(|x| x < &end_date)
                                .map(|x| adjust(&x, self.calendar, self.adjust_rule))
                                .collect();
            return Ok(res);
        }
    }
    
}


// For the case of Preceding, ModFollowing, Nearest, etc it will keep giving 
// the function might simply return the same as anchor date after adjustment.
// The loop below forces that the returned date is after the anchor date.
// Should only be an issue for the Daily Frequency, but it covers all cases.
fn force_adjust ( anchor_date: &NaiveDate, next_date: &NaiveDate, opt_calendar: Option<&Calendar>
                       , opt_adjust_rule: Option<AdjustRule>) -> NaiveDate {    
    let mut res: NaiveDate = algebra::adjust(next_date, opt_calendar, opt_adjust_rule);
    // Case where the adjustment brings the date back to the same as the anchor
    if res <= *anchor_date {
        let mut dayi = 1;
        while res <= *anchor_date {
            res = next_date.checked_add_signed(Duration::days(dayi)).unwrap_or_else(|| {
                panic!("Next Adjusted Date is out of bounds, check chrono internals for the last date available");
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
pub fn schedule_next ( anchor_date: &NaiveDate, frequency: Frequency) -> Option<NaiveDate> {
    
    // Calculate next for each of the Frequencies.
    match frequency {
        Frequency::Daily => {
            return anchor_date.checked_add_days(Days::new(1));
        },
        
        Frequency::Weekly => {
            return anchor_date.checked_add_signed(Duration::weeks(1));
        },
        
        Frequency::Biweekly => {
            return anchor_date.checked_add_signed(Duration::weeks(2));
        },

        Frequency::EveryFourthWeek => {
            return anchor_date.checked_add_signed(Duration::weeks(4));
        },

        Frequency::Monthly => {
            // There is no months Duration, so using Months struct from Chrono
            return anchor_date.checked_add_months(Months::new(1));
        },

        Frequency::Bimonthly => {
            return anchor_date.checked_add_months(Months::new(2));
        },

        Frequency::Quarterly => {
            return anchor_date.checked_add_months(Months::new(3));
        },

        Frequency::EveryFourthMonth => {
            return anchor_date.checked_add_months(Months::new(4));
        },

        Frequency::Semiannual => {
            return anchor_date.checked_add_months(Months::new(6));
        },

        Frequency::Annual => {
            let delta = 1;
            return checked_add_years(anchor_date, delta);
        },

        Frequency::Once => {return Some(*anchor_date);}

    }


}


/// Iterator over dates of a schedule.
/// This is an unbounded
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleIterator<'a> {
    schedule: &'a Schedule<'a>,
    anchor: NaiveDate,
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
        self.anchor = res.expect("Next date for this schedule is out of bounds.");
        return res;
    }
}

// Next function for the Schedule iterator
fn schedule_iterator_next<'a> (schedule: & Schedule, anchor: NaiveDate) -> Option<NaiveDate> {
    
   schedule_next(&anchor, schedule.frequency)  
    
}

pub fn schedule_next_adjusted<'a> (schedule: & Schedule, anchor: NaiveDate) -> NaiveDate {
    // Call next and then adjust.
    let next = schedule_next(&anchor, schedule.frequency).expect("Next date for this schedule is out of bounds or malformed");
    force_adjust(&anchor, &next, schedule.calendar, schedule.adjust_rule)
         
}
    




/// Unit Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use chrono::{NaiveDate, Datelike};
    use crate::calendar as c;
    use crate::conventions::{Frequency, AdjustRule };
    use super::{Schedule,ScheduleIterator, schedule_next_adjusted};

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
        let res = schedule_next_adjusted(&sch, anchor);
        // Even though this is falls on a next month, Next will force the next date for daily frequencies
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap() );
        // With No Adjustment Rule, it will disregard the calendar and return the next date regardless if it is a good day or not.
        let sch = Schedule {frequency: Frequency::Daily, calendar: Some(&cal), adjust_rule: None};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 9, 30).unwrap() ); // This is a Saturday.
        // Preceding Rule will also force the next day to be output for daily frequencies.
        let sch = Schedule {frequency: Frequency::Daily, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Preceding)};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap() );
        // Or even for nearest
        let sch = Schedule {frequency: Frequency::Daily, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Nearest)};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 10, 2).unwrap() );


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
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(anchor.weekday(), res.weekday());
        // Even with an Adjustment rule, it is a Friday so weekday should be the same
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Preceding)};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(anchor.weekday(), res.weekday());
        // If the next date falls on a holiday, it will be adjusted according to adjust Rule
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Preceding)};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 22).unwrap() ); // This is a Friday
        // Adjusting with Following
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Following)};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 27).unwrap() );
        // With no calendar
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
        let sch = Schedule {frequency: Frequency::Weekly, calendar: None, adjust_rule: Some(AdjustRule::Following)};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 25).unwrap() );
        assert_eq!(anchor.weekday(), res.weekday());
        // A Start date will not be adjusted, but the next date will
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap(); // Boxing day
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModFollowing)};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(res, NaiveDate::from_ymd_opt(2024, 1, 2).unwrap());
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 23).unwrap(); // Saturday
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModFollowing)};
        let res = schedule_next_adjusted(&sch, anchor);
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
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(anchor.weekday(), res.weekday());
        // With adjustment
        let sch = Schedule {frequency: Frequency::Biweekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Nearest)};
        let res = schedule_next_adjusted(&sch, anchor);
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
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(anchor.weekday(), res.weekday());
        // With adjustment
        let sch = Schedule {frequency: Frequency::EveryFourthWeek, calendar: Some(&cal), adjust_rule: Some(AdjustRule::Nearest)};
        let res = schedule_next_adjusted(&sch, anchor);
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
        let sch = Schedule {frequency: Frequency::Monthly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModPreceding)};
        let res = schedule_next_adjusted(&sch, anchor);
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
        let sch = Schedule {frequency: Frequency::Bimonthly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModPreceding)};
        let res = schedule_next_adjusted(&sch, anchor);
        assert_eq!(anchor.day(), res.day());
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 4, 18).unwrap());        
    }


    // Schedule Iterator test
    #[test]
    fn schedule_iterator_test () {


    }




}