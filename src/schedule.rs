//! Schedules
//! The output here can come from both methods or free functions.

use std::ops::Add;

use chrono::{NaiveDate, Duration};

use crate::calendar::{Calendar, self};
use crate::conventions::{AdjustRule,DayCount, DateUnit, Frequency,Tenor};
use crate::algebra;


/// A Schedule with an Anchor date
/// 
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule<'a> {
    pub frequency: Frequency,
    pub calendar: Option<&'a Calendar>,
    pub adjust_rule: Option<AdjustRule>,
}



impl Schedule<'_> {

    // pub fn generate (&self, anchor_date: NaiveDate, end_date: NaiveDate ) -> Result<Vec<NaiveDate>, &'static str> {
    //     // Check input dates       
    //     if end_date <= anchor_date {
    //         return  Err("Anchor date must be before end date");
    //     } 
    //     // If no calendar and no adjustment just sum using the frequency.
    //     else if self.calendar == None && self.adjust_rule == None {
            
    //     }
    //     else {
    //         match self.calendar {
                
    //         }

    //         return Ok(());
    //     }

    // }


    
}


/// The function below will add the specified duration to an
/// anchor date and adjust it to a working day according to the 
/// given calendar and adjust rule.
pub fn force_add_adjust( anchor_date: &NaiveDate, delta: Duration, opt_calendar: Option<&Calendar>
                       , opt_adjust_rule: Option<AdjustRule>) -> NaiveDate {    
    let mut res = *anchor_date;
    res = anchor_date.checked_add_signed(delta).unwrap_or_else(|| {
        panic!("Next Date for {} frequency is out of bounds, check chrono internals for the last date available", Frequency::Weekly);
    });
    res = algebra::adjust(&res, opt_calendar, opt_adjust_rule);
    // The corner case of a Week's long holiday
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

                       


// Gets the next date given an anchor date and a schedule
// for a given frequency. The function will not adjust the anchor date,
// but it will adjust the next date if a calendar and adjust rule is passed.
pub fn schedule_next ( anchor_date: &NaiveDate, frequency: Frequency
                      , opt_calendar: Option<&Calendar>, opt_adjust_rule: Option<AdjustRule>) -> NaiveDate {
    
    // Return Date
    let mut res: NaiveDate = *anchor_date;
    // Calculate next for each of the Frequencies.
    match frequency {
        Frequency::Daily => {
            // For the case of Preceding, ModFollowing, Nearest, etc it will keep giving 
            // the function might simply return the same as anchor date after adjustment.
            // The loop below forces that the returned dat
            // Should only be an issue for the Daily Frequency.
            let delta = Duration::days(1);
            return force_add_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule)
        },
        
        Frequency::Weekly => {
            let delta = Duration::weeks(1);
            return force_add_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule)
            }
        
        Frequency::Biweekly => {
            let delta = Duration::weeks(2);
            return force_add_adjust(anchor_date, delta, opt_calendar, opt_adjust_rule)
        }

        // !!! stubs
        Frequency::Annual => {return res;}
        Frequency::Bimonthly => {return res;}
        Frequency::Monthly => {return res;}
        
        Frequency::EveryFourthMonth => {return res;}
        Frequency::EveryFourthWeek => {return res;}
        Frequency::Once => {return res;}
        Frequency::OtherFrequency => {return res;}
        Frequency::Quarterly => {return res;}
        Frequency::Semiannual => {return res;}
        


        



    }


}


/// Iterator over dates of a schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleIterator<'a> {
    schedule: Schedule<'a>,
    anchor: NaiveDate
}

impl<'a> Iterator for ScheduleIterator<'a> {
    type Item = Schedule<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        schedule_iterator_next(&mut self.schedule, self.anchor)
        
    }
}

// Next function for the Schedule iterator
fn schedule_iterator_next<'a> (schedule: &mut Schedule, anchor: NaiveDate) -> Option<Schedule<'a>> {
    
    
    return None;
}


/// Unit Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use chrono::{NaiveDate, Datelike};
    use crate::calendar as c;
    use crate::conventions::{Frequency, AdjustRule, DayCount };
    use super::{Schedule, schedule_next};

        // Setup for variables to be used in multiples tests
        struct setup {
            cal: c::Calendar,
            test_weekend: NaiveDate,
            test_holiday: NaiveDate
        }
        // Setup constructor
        impl setup {
            fn  new() -> Self {
                let mut basic_cal: c::Calendar = c::basic_calendar();
                let christmas_day = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
                let boxing_day = NaiveDate::from_ymd_opt(2023,12,26).unwrap();
                let new_holidays: HashSet<NaiveDate> =  [christmas_day, boxing_day].into_iter().collect();
                let test_weekend: NaiveDate = NaiveDate::from_ymd_opt(2023, 9,2).unwrap(); // this is a Saturday
                basic_cal.add_holidays(&new_holidays);
                Self { 
                        cal : basic_cal,
                        test_holiday: christmas_day,
                        test_weekend: test_weekend
                }
            }
        }


    // Schedule Generator tests
    // Daily Frequency test
    #[test]
    fn daily_next_test () {
        let setup: setup = setup::new();
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
    }

    // Weekly Frequency test
    #[test]
    fn weekly_next_test () {
        let setup: setup = setup::new();
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
        assert_eq!(res, NaiveDate::from_ymd_opt(2024, 1, 2).unwrap() );
        let anchor: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 23).unwrap(); // Saturday
        let sch = Schedule {frequency: Frequency::Weekly, calendar: Some(&cal), adjust_rule: Some(AdjustRule::ModFollowing)};
        let res = schedule_next(&anchor, sch.frequency, sch.calendar, sch.adjust_rule);
        assert_eq!(res, NaiveDate::from_ymd_opt(2023, 12, 29).unwrap() );


    }

    // Biweekly Frequency test
    #[test]
    fn biweekly_next_test () {
        
    }

    // Monthly Frequency test
    #[test]
    fn monthly_next_test () {
        
    }


}