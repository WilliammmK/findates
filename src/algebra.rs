use std::ops::Add;
use std::ops::Sub;


use chrono::{NaiveDate, Datelike, Days};

use crate::calendar::Calendar;
use crate::calendar as c;
use crate::conventions::{DayCount, AdjustRule, Frequency};


pub struct Date(NaiveDate);



/// Check if a date is a good business day in a given calendar.
pub fn is_business_day (date: &NaiveDate, calendar: &Calendar) -> bool {
    if calendar.weekend.contains(&date.weekday()) {
        return false;
    } else if calendar.holidays.contains(date) {
        return false;
    } else {
        return true;    
    }
}


/// Adjust a date to a business day according to a Calendar and a AdjustRule
/// This function returns a new NaiveDate without modifying the input.
pub fn adjust (date: &NaiveDate, calendar: &Calendar, adjust_rule: Option<AdjustRule>) -> NaiveDate {
    // If it is a good day, just return it.
    if is_business_day(date, calendar) {
        return date.clone();
    } else {
        let adj_date: NaiveDate;
        match adjust_rule {
            None                                => return date.clone(),

            Some(AdjustRule::Unadjusted)  => return date.clone(),

            Some(AdjustRule::Following)   => {
                return add_adjust(date, calendar);
            },
    
            Some(AdjustRule::ModFollowing)  => {
                adj_date = add_adjust(date, calendar);
                if adj_date.month() != date.month() {
                    return sub_adjust(date, calendar);
                } else {
                    return adj_date;
                }
            },
    
            Some(AdjustRule::Preceding)  => {
                return sub_adjust(date, calendar);
            }
            
            Some(AdjustRule::ModPreceding)  => {
                adj_date = sub_adjust(date, calendar);
                if adj_date.month() != date.month() {
                    return add_adjust(date, calendar);
                } else {
                    return adj_date;
                }
            }

            Some(AdjustRule::HalfMonthModFollowing)  => {
                adj_date = add_adjust(date, calendar);
                if adj_date.month() != date.month() {
                    return sub_adjust(date, calendar);
                } else if date.day() <= 15 && adj_date.day() > 15  {
                    return sub_adjust(date, calendar);
                } else {
                    return adj_date;
                }
            }

            Some(AdjustRule::Nearest)  => {
                let follow_date: NaiveDate = add_adjust(date, calendar);
                let prec_date: NaiveDate   = sub_adjust(date, calendar);
                if (follow_date - *date).num_days().abs() <= (prec_date - *date).num_days().abs() {
                    return follow_date;
                } else {
                    return prec_date;
                }
            }
        } 
    }


}

// Auxiliary function to adjust, not to be exported
fn add_adjust (date: &NaiveDate, calendar: &Calendar) -> NaiveDate {
    let mut t = 1;
    let mut adj_date: NaiveDate = date.checked_add_days(Days::new(t)).unwrap(); // add_days function does not modify the original date
    loop {
        if is_business_day(&adj_date, calendar) {
            break;
        } else {
            t += 1;
            adj_date = date.checked_add_days(Days::new(t)).unwrap();
        }
    }
    return adj_date;
}

// Auxiliary function to adjust, not to be exported
fn sub_adjust (date: &NaiveDate, calendar: &Calendar) -> NaiveDate {
    let mut t = 1;
    let mut adj_date: NaiveDate = date.checked_sub_days(Days::new(t)).unwrap(); // add_days function does not modify the original date
    loop {
        if is_business_day(&adj_date, calendar) {
            break;
        } else {
            t += 1;
            adj_date = date.checked_sub_days(Days::new(t)).unwrap();
        }
    }
    return adj_date;
}

/// Schedule Generation between two dates.
/// Start date and End date will be adjusted according to the given calendar
/// and included in the output vector.
/// AdjustRule will default to Following if nothing is passed
pub fn bus_day_schedule ( start_date: &NaiveDate, end_date: &NaiveDate
                        , calendar: &Calendar, adjust_rule: Option<AdjustRule>) -> Vec<NaiveDate> {
    
    // Following rule as default
    let rule: Option<AdjustRule>;
    if adjust_rule == None {
        rule = Some(AdjustRule::Following);
    } else {
        rule = adjust_rule;
    }

    // Adjust the start and end date if needed.    
    let new_start: NaiveDate = adjust(start_date, calendar, rule);
    let new_end: NaiveDate = adjust(end_date, calendar, rule);
    // Initialize the output vector with the adjusted start date.
    let mut schedule: Vec<NaiveDate> = vec![new_start];
    // Auxiliary variable for looping
    let mut previous_bus_day: NaiveDate = new_start;
        
    
    while previous_bus_day <= new_end {
        // Counter for Days to be added
        let mut t = 1;     
        // Need the loop for when the AdjustRule land at a prior date such as Preceding
        let mut next_bus_day: NaiveDate = adjust(&previous_bus_day.checked_add_days(Days::new(t)).unwrap(), calendar, rule);
        
        // Check if the adjustment didn't land at the same date
        loop {
            if next_bus_day > previous_bus_day {
                break;
            } else {
                t += 1;
                next_bus_day = adjust(&previous_bus_day.checked_add_days(Days::new(t)).unwrap(), calendar, rule);
            }   
        }
        // Add it to the result vector
        schedule.push(next_bus_day);
        // Reset the auxiliary variable to the latest date
        previous_bus_day = next_bus_day;
    }
    
    return schedule;

    }


/// Business Day counter
/// !!! Needs the Schedule generator

/// Day count fraction calculation from a start and an end date.
/// If no Calendar is passed, there will be no adjustment to the dates.
/// If a Calendar and AdjustRule are passed, the dates will be adjusted before the calculation.
/// If a Calendar is passed and no adjust rule, the adjust rule will default to Following.
/// If the end date passed is before the start date, it will calculate the fraction on the 
/// absolute time difference.
pub fn day_count_fraction (start_date: &NaiveDate , end_date: &NaiveDate, daycount: DayCount, 
                           calendar: Option<&Calendar>, adjust_rule: Option<AdjustRule>) -> f64 {
    // !!! Stub only
    let delta: i64;
    if calendar == None {
        delta = (*end_date - *start_date).num_days().abs();
    } else {
        let start_adjust: NaiveDate = adjust(start_date, calendar.unwrap(), adjust_rule);
        let end_adjust: NaiveDate   = adjust(end_date, calendar.unwrap(), adjust_rule);
        delta = (start_adjust - end_adjust).num_days().abs();
    }
    
    match daycount {
        DayCount::Act360 => {
           return delta as f64/360.0; 
        }

        DayCount::Act365 => {
            return delta as f64/365.0; 
         }

         DayCount::ActAct => {
            return 3.0; // !!! stub
         }

         DayCount::D30360 => {
            return 3.0; // !!! stub
         }

         DayCount::D30365 => {
            return 3.0; // !!! stub
         }

         DayCount::Bd252 => {
            return 3.0; // !!! stub
         }
        
    }

    
}







/// Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use chrono::Datelike;
    use chrono::NaiveDate;
    use chrono::Weekday;
    use chrono::Days;
    use crate::algebra::bus_day_schedule;
    use crate::calendar as c;
    use crate::algebra as a;
    use crate::conventions::{AdjustRule, Frequency, DayCount, DateUnit} ;

    use super::day_count_fraction;
    
    fn round_decimals(x: f64) -> f64 {
        let multiplier = 100000.0; 
        (x * multiplier).round() / multiplier
    }
    

    // Is business day function test.
    #[test]
    fn is_business_day_test() {
        let mut basic_cal: c::Calendar = c::basic_calendar();
        let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Sun);
        assert_eq!(false, a::is_business_day(&my_date.unwrap(), &basic_cal));
        let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Mon);
        assert_eq!(true, a::is_business_day(&my_date.unwrap(), &basic_cal));
        let christmas_day = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
        assert_eq!(true, a::is_business_day(&christmas_day, &basic_cal));
        basic_cal.add_holidays(&[christmas_day].into_iter().collect());
        assert_eq!(false, a::is_business_day(&christmas_day, &basic_cal));

    }

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


    // Adjust tests
    #[test]
    fn adjust_following_test() { 
        let setup: setup = setup::new();
        let cal: c::Calendar = setup.cal; 
        assert_eq!(a::adjust(&setup.test_weekend, &cal, Some(AdjustRule::Following)), NaiveDate::from_ymd_opt(2023, 9,4).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday, &cal, Some(AdjustRule::Following)), NaiveDate::from_ymd_opt(2023, 12, 27).unwrap());
        assert_ne!(a::adjust(&setup.test_holiday, &cal, Some(AdjustRule::Following)), NaiveDate::from_ymd_opt(2023, 12, 26).unwrap());
    }

    #[test]
    fn adjust_preceding_test() {
        let setup: setup = setup::new();
        let cal: c::Calendar = setup.cal; 
        let sunday = setup.test_weekend.checked_add_days(Days::new(1)).unwrap();
        assert_eq!(sunday.weekday(), Weekday::Sun);
        assert_eq!(a::adjust(&sunday, &cal, Some(AdjustRule::Preceding)), NaiveDate::from_ymd_opt(2023, 9,1).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday.checked_add_days(Days::new(1)).unwrap(), &cal, Some(AdjustRule::Preceding)), NaiveDate::from_ymd_opt(2023, 12, 22).unwrap());
        assert_ne!(a::adjust(&setup.test_holiday, &cal, Some(AdjustRule::Preceding)), NaiveDate::from_ymd_opt(2023, 12, 25).unwrap());
    }

    #[test]
    fn adjust_modfollowing_test() {
        let setup: setup = setup::new();
        let cal: c::Calendar = setup.cal; 
        let eom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        assert_eq!(a::adjust(&eom, &cal, Some(AdjustRule::ModFollowing)), NaiveDate::from_ymd_opt(2023, 9,29).unwrap());
        assert_eq!(a::adjust(&setup.test_weekend, &cal, Some(AdjustRule::ModFollowing)), NaiveDate::from_ymd_opt(2023, 9, 4).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday, &cal, Some(AdjustRule::ModFollowing)), NaiveDate::from_ymd_opt(2023, 12, 27).unwrap());
    }

    #[test]
    fn adjust_modpreceding_test() {
        let setup: setup = setup::new();
        let mut cal: c::Calendar = setup.cal; 
        cal.add_holidays(&[NaiveDate::from_ymd_opt(2023, 2, 1).unwrap()].into_iter().collect());
        let bom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 1).unwrap();
        let boy: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(a::adjust(&bom, &cal, Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 9,1).unwrap());
        assert_eq!(a::adjust(&boy, &cal, Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 1,2).unwrap());
        assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(), &cal, Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 2,2).unwrap());
    }

    #[test]
    fn adjust_halfmonthmodfollowing_test() {
        let setup: setup = setup::new();
        let mut cal: c::Calendar = setup.cal; 
        let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        cal.add_holidays(&[new_hol].into_iter().collect());
        let eom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // This is a Saturday
        assert_eq!(a::adjust(&setup.test_weekend, &cal, Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 9,4).unwrap());
        assert_eq!(a::adjust(&eom, &cal, Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 9,29).unwrap());
        assert_eq!(a::adjust(&mom, &cal, Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 1,13).unwrap());
        assert_eq!(a::adjust(&new_hol, &cal, Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 2,14).unwrap());
        assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(), &cal, Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 6,15).unwrap());
    }

    #[test]
    fn adjust_nearest_test() {
        let setup: setup = setup::new();
        let mut cal: c::Calendar = setup.cal;
        let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        cal.add_holidays(&[new_hol].into_iter().collect());
        let bom: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // This is a Saturday
        assert_eq!(a::adjust(&bom, &cal, Some(AdjustRule::Nearest)), NaiveDate::from_ymd_opt(2023, 10,2).unwrap());
        assert_eq!(a::adjust(&mom, &cal, Some(AdjustRule::Nearest)), NaiveDate::from_ymd_opt(2023, 1,13).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday, &cal, Some(AdjustRule::Nearest)), NaiveDate::from_ymd_opt(2023, 12,27).unwrap());
        assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(), &cal, Some(AdjustRule::Nearest)), NaiveDate::from_ymd_opt(2023, 12,22).unwrap());
    }

    #[test]
    fn adjust_unadjusted_test() {
        let setup: setup = setup::new();
        let mut cal: c::Calendar = setup.cal;
        let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        cal.add_holidays(&[new_hol].into_iter().collect());
        let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // This is a Saturday
        assert_eq!(a::adjust(&new_hol, &cal, Some(AdjustRule::Unadjusted)), NaiveDate::from_ymd_opt(2023, 2,15).unwrap());
        assert_eq!(a::adjust(&mom, &cal, Some(AdjustRule::Unadjusted)), NaiveDate::from_ymd_opt(2023, 1,14).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday, &cal, Some(AdjustRule::Unadjusted)), NaiveDate::from_ymd_opt(2023, 12,25).unwrap());
        assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(), &cal, Some(AdjustRule::Unadjusted)), NaiveDate::from_ymd_opt(2023, 12,24).unwrap());
    }

    // Business Day schedule test
    #[test]
    fn schedule_test() {
        let mut test_schedule: Vec<NaiveDate> = [].to_vec();
        // Create test vector with all the dates
        for i in 1 .. 31 {
            let dt = NaiveDate::from_ymd_opt(2023,9,i).unwrap();
            // Exclude weekends
            if dt.weekday() == Weekday::Sat || dt.weekday() == Weekday::Sun {} 
            // Include a Holiday
            else if dt == NaiveDate::from_ymd_opt(2023,9 , 22).unwrap() {}
            else {
                test_schedule.push(dt)
            }            
        }

    println!("{:?}", test_schedule);
    let setup: setup = setup::new();
    let mut cal: c::Calendar = setup.cal;
    let start_date: NaiveDate = NaiveDate::from_ymd_opt(2023,9,1).unwrap();
    let end_date: NaiveDate = NaiveDate::from_ymd_opt(2023,9,30).unwrap();
    let res = bus_day_schedule(&start_date, &end_date, &cal, Some(AdjustRule::Following));
    println!("{:?}", res);


    }

    // Day count Fraction tests
    #[test]
    fn dcf_act360_test() {
        let setup: setup = setup::new();
        let cal: c::Calendar = setup.cal;
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        let expected: f64 = 0.6305556;
        let res: f64 = day_count_fraction(&start, &end
                                        , DayCount::Act360, None, None);
        // No calendar
        assert_eq!(round_decimals(res), round_decimals(expected) );    
        // With Calendar
        let start = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap(); // Adjusted to 02 Oct
        let end = NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(); // Adjusted to 27 Dec
        let expected = 0.2388889;
        let res = day_count_fraction(&start, &end
                                        , DayCount::Act360, Some(&cal), Some(AdjustRule::Following));
        assert_eq!(round_decimals(res), round_decimals(expected));
    }


}
