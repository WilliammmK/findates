/// Functions using Schedules, Calendars and the Conventions.
/// A more functional approach was taken in this module,
/// with no side effects or altering of internal states.

use chrono::{NaiveDate, Datelike, Days};
use crate::calendar::Calendar;
use crate::conventions::{DayCount, AdjustRule};


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
pub fn adjust (date: &NaiveDate, opt_calendar: Option<&Calendar>, adjust_rule: Option<AdjustRule>) -> NaiveDate {
    
    // No adjustment given no calendar
    let calendar: &Calendar;    
    if opt_calendar == None {return date.clone();}
    else { calendar = opt_calendar.unwrap() }
    
    // If it is a good day, just return it.
    if is_business_day(date, calendar) {
        return date.clone();
    } else {
        let adj_date: NaiveDate;
        match adjust_rule {
            None                          => return date.clone(),

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
    let mut adj_date: NaiveDate = date.checked_add_days(Days::new(t)).unwrap_or_else(|| {
        panic!("Date is out of bounds, check chrono internals for the last date available");
    }); // add_days function does not modify the original date
    loop {
        if is_business_day(&adj_date, calendar) {
            break;
        } else {
            t += 1;
            adj_date = date.checked_add_days(Days::new(t)).unwrap_or_else(|| {
                panic!("Date is out of bounds, check chrono internals for the last date available");
            });
        }
    }
    return adj_date;
}



// Auxiliary function to adjust, not to be exported
fn sub_adjust (date: &NaiveDate, calendar: &Calendar) -> NaiveDate {
    let mut t = 1;
    let mut adj_date: NaiveDate = date.checked_sub_days(Days::new(t)).unwrap_or_else(|| {
        panic!("Date is out of bounds, check chrono internals for the first date available");
    }); // add_days function does not modify the original date
    loop {
        if is_business_day(&adj_date, calendar) {
            break;
        } else {
            t += 1;
            adj_date = date.checked_sub_days(Days::new(t)).unwrap_or_else(|| {
                panic!("Date is out of bounds, check chrono internals for the first date available");
            });
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
    let new_start: NaiveDate = adjust(start_date, Some(calendar), rule);
    let new_end: NaiveDate = adjust(end_date,  Some(calendar), rule);
    // Initialize the output vector with the adjusted start date.
    let mut schedule: Vec<NaiveDate> = vec![new_start];
    // Auxiliary variable for looping
    let mut previous_bus_day: NaiveDate = new_start;
        
    
    while previous_bus_day < new_end {
        // Counter for Days to be added
        let mut t = 1;     
        // Need the loop for when the AdjustRule land at a prior date such as Preceding
        let mut next_bus_day: NaiveDate = adjust(&previous_bus_day.checked_add_days(Days::new(t)).unwrap(),  Some(calendar), rule);
        
        // Check if the adjustment didn't land at the same date
        loop {
            if next_bus_day > previous_bus_day {
                break;
            } else {
                t += 1;
                next_bus_day = adjust(&previous_bus_day.checked_add_days(Days::new(t)).unwrap(),  Some(calendar), rule);
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
/// This includes the start date but excludes the end date – as 
/// it is common for financial calculations.
/// This uses the bus_day_schedule function to generate a schedule first, so input dates will be adjusted.
pub fn business_days_between (start_date: &NaiveDate, end_date: &NaiveDate, calendar: &Calendar, adjust_rule: Option<AdjustRule>) -> u64 {
    let schedule: Vec<NaiveDate> = bus_day_schedule(start_date, end_date, calendar, adjust_rule);
    // Since the schedule generated includes the end date we subtract one.
    return schedule.len() as u64 - 1;
}

/// Day count fraction calculation from a start and an end date.
/// If no Calendar is passed, there will be no adjustment to the dates.
/// If a Calendar and AdjustRule are passed, the dates will be adjusted before the calculation.
/// If a Calendar is passed and no adjust rule, the adjust rule will default to Following.
/// If the end date passed is before the start date, it will calculate the fraction on the 
/// absolute time difference.
pub fn day_count_fraction (start_date: &NaiveDate , end_date: &NaiveDate, daycount: DayCount, 
                           calendar: Option<&Calendar>, adjust_rule: Option<AdjustRule>) -> f64 {
    let delta: i64;
    let start_adjusted: NaiveDate;
    let end_adjusted: NaiveDate;
    let some_adjust_rule: Option<AdjustRule>;
    if calendar == None {
        start_adjusted = *start_date;
        end_adjusted   = *end_date;
        some_adjust_rule = adjust_rule;
        delta = (*end_date - *start_date).num_days().abs();

    } else {
        // Default Adjust rule to following
        some_adjust_rule = if adjust_rule == None { Some(AdjustRule::Following) } else { adjust_rule };
        start_adjusted = adjust(start_date,  calendar, some_adjust_rule);
        end_adjusted  = adjust(end_date, calendar, some_adjust_rule);
        delta = (start_adjusted - end_adjusted).num_days().abs();
    }
    // Auxiliary variables
    let start_year: i32 = start_adjusted.year();
    let start_month: i32 = start_adjusted.month() as i32;
    let mut start_day: i32 = start_adjusted.day() as i32;
    let end_year: i32 = end_adjusted.year();
    let end_month: i32 = end_adjusted.month() as i32;
    let mut end_day: i32 = end_adjusted.day() as i32;
    
    match daycount {
        DayCount::Act360 => {
           return delta as f64/360.0; 
        }

        DayCount::Act365 => {
            return delta as f64/365.0; 
         }

        DayCount::ActActISDA => {     
            if start_adjusted == end_adjusted { return 0.0; } 
            // If dates on the same leap year
            else if start_year == end_year && is_leap_year(start_year) { return delta as f64/366.0; }
            // If dates on the same 365 year
            else if start_year == end_year && !is_leap_year(start_year) { return delta as f64/365.0; }
            // If input start date is after end date
            else if start_adjusted > end_adjusted {
                return day_count_fraction(&end_adjusted, &start_adjusted, DayCount::ActActISDA, calendar, some_adjust_rule);
            }
            // Start date and end date in different years, that need to be checked if leap year or not.
            else {
                let mut dcf: f64 = end_year as f64 - start_year as f64 - 1.0;
                let base1: i32 = if is_leap_year(start_year) { 366 } else { 365 };
                let base2: i32 = if is_leap_year(end_year) { 366 } else { 365 };
                let dcf1: f64 = (NaiveDate::from_ymd_opt(start_year + 1, 1,1).unwrap() - start_adjusted).num_days() as f64 / base1 as f64;
                let dcf2: f64 = (end_adjusted - NaiveDate::from_ymd_opt(end_year, 1,1).unwrap()).num_days() as f64 / base2 as f64;               
                dcf = dcf + dcf1 as f64 + dcf2 as f64;
                return  dcf;
            }
         }

        DayCount::D30360Euro => {   
            // Adjust if day i the 31st
            if start_day == 31 { start_day = 30; } else {};
            if end_day == 31 { end_day = 30; } else {};
            
            let res = 360 * (end_year - start_year) + (30 * (end_month - start_month)) + (end_day - start_day);
            return res as f64 / 360.0;
         }

         DayCount::D30365 => {
            let res:f64 = 360.0 * (end_year - start_year) as f64 + (30.0 * (end_month - start_month) as f64) + (end_day - start_day) as f64; // Different than Quanlib's implementation.
            return res / 365.0;
         }

         DayCount::Bd252 => {
            // BD252 requires a calendar
            if calendar == None {
                // Review if panic is being done correctly
                panic!("Bd252 Day count requires a Calendar input!")
            } else {
                return business_days_between(&start_adjusted, &end_adjusted, calendar.unwrap(), some_adjust_rule) as f64/ 252.0;                
            }            
         }
        
    }

    
}

// Convenience function to add years since chrono doesn't provide one.
pub fn checked_add_years(date: &NaiveDate, years_to_add: i32) -> Option<NaiveDate> {
    let current_year = date.year();
    let current_month = date.month();
    let current_day = date.day();

    let new_year = current_year + years_to_add;

    NaiveDate::from_ymd_opt(new_year, current_month, current_day)
}


// Auxiliary function to check if a year in i32 
// format is a leap year.
fn is_leap_year (year: i32) -> bool {
    let date: Option<NaiveDate> = NaiveDate::from_ymd_opt(year, 2, 29);
    // If the date is Some, it's a valid leap year; otherwise, it's not.
    date.is_some()
}



// Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use chrono::Datelike;
    use chrono::NaiveDate;
    use chrono::Weekday;
    use chrono::Days;
    use crate::algebra::bus_day_schedule;
    use crate::algebra::business_days_between;
    use crate::calendar as c;
    use crate::algebra as a;
    use crate::conventions::{AdjustRule, DayCount} ;

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
    struct Setup {
        cal: c::Calendar,
        test_weekend: NaiveDate,
        test_holiday: NaiveDate
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
                    test_holiday: christmas_day,
                    test_weekend: test_weekend
            }
        }
    }


    // Adjust tests
    #[test]
    fn adjust_following_test() { 
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal; 
        assert_eq!(a::adjust(&setup.test_weekend, Some(&cal), Some(AdjustRule::Following)), NaiveDate::from_ymd_opt(2023, 9,4).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Following)), NaiveDate::from_ymd_opt(2023, 12, 27).unwrap());
        assert_ne!(a::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Following)), NaiveDate::from_ymd_opt(2023, 12, 26).unwrap());
    }

    #[test]
    fn adjust_preceding_test() {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal; 
        let sunday = setup.test_weekend.checked_add_days(Days::new(1)).unwrap();
        assert_eq!(sunday.weekday(), Weekday::Sun);
        assert_eq!(a::adjust(&sunday, Some(&cal), Some(AdjustRule::Preceding)), NaiveDate::from_ymd_opt(2023, 9,1).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday.checked_add_days(Days::new(1)).unwrap(), Some(&cal), Some(AdjustRule::Preceding)), NaiveDate::from_ymd_opt(2023, 12, 22).unwrap());
        assert_ne!(a::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Preceding)), NaiveDate::from_ymd_opt(2023, 12, 25).unwrap());
    }

    #[test]
    fn adjust_modfollowing_test() {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal; 
        let eom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        assert_eq!(a::adjust(&eom, Some(&cal), Some(AdjustRule::ModFollowing)), NaiveDate::from_ymd_opt(2023, 9,29).unwrap());
        assert_eq!(a::adjust(&setup.test_weekend, Some(&cal), Some(AdjustRule::ModFollowing)), NaiveDate::from_ymd_opt(2023, 9, 4).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::ModFollowing)), NaiveDate::from_ymd_opt(2023, 12, 27).unwrap());
    }

    #[test]
    fn adjust_modpreceding_test() {
        let setup: Setup = Setup::new();
        let  mut cal: c::Calendar = setup.cal;  
        cal.add_holidays(&[NaiveDate::from_ymd_opt(2023, 2, 1).unwrap()].into_iter().collect());
        let bom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 1).unwrap();
        let boy: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(a::adjust(&bom, Some(&cal), Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 9,1).unwrap());
        assert_eq!(a::adjust(&boy, Some(&cal), Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 1,2).unwrap());
        assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(), Some(&cal), Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 2,2).unwrap());
    }

    #[test]
    fn adjust_halfmonthmodfollowing_test() {
        let setup: Setup = Setup::new();
        let  mut cal: c::Calendar = setup.cal;  
        let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        cal.add_holidays(&[new_hol].into_iter().collect());
        let eom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // This is a Saturday
        assert_eq!(a::adjust(&setup.test_weekend, Some(&cal), Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 9,4).unwrap());
        assert_eq!(a::adjust(&eom, Some(&cal), Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 9,29).unwrap());
        assert_eq!(a::adjust(&mom, Some(&cal), Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 1,13).unwrap());
        assert_eq!(a::adjust(&new_hol, Some(&cal), Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 2,14).unwrap());
        assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(), Some(&cal), Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 6,15).unwrap());
    }

    #[test]
    fn adjust_nearest_test() {
        let setup: Setup = Setup::new();
        let  mut cal: c::Calendar = setup.cal;  
        let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        cal.add_holidays(&[new_hol].into_iter().collect());
        let bom: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // This is a Saturday
        assert_eq!(a::adjust(&bom, Some(&cal), Some(AdjustRule::Nearest)), NaiveDate::from_ymd_opt(2023, 10,2).unwrap());
        assert_eq!(a::adjust(&mom, Some(&cal), Some(AdjustRule::Nearest)), NaiveDate::from_ymd_opt(2023, 1,13).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Nearest)), NaiveDate::from_ymd_opt(2023, 12,27).unwrap());
        assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(), Some(&cal), Some(AdjustRule::Nearest)), NaiveDate::from_ymd_opt(2023, 12,22).unwrap());
    }

    #[test]
    fn adjust_unadjusted_test() {
        let setup: Setup = Setup::new();
        let  mut cal: c::Calendar = setup.cal;  
        let new_hol = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        cal.add_holidays(&[new_hol].into_iter().collect());
        let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // This is a Saturday
        assert_eq!(a::adjust(&new_hol, Some(&cal), Some(AdjustRule::Unadjusted)), NaiveDate::from_ymd_opt(2023, 2,15).unwrap());
        assert_eq!(a::adjust(&mom, Some(&cal), Some(AdjustRule::Unadjusted)), NaiveDate::from_ymd_opt(2023, 1,14).unwrap());
        assert_eq!(a::adjust(&setup.test_holiday, Some(&cal), Some(AdjustRule::Unadjusted)), NaiveDate::from_ymd_opt(2023, 12,25).unwrap());
        assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(), Some(&cal), Some(AdjustRule::Unadjusted)), NaiveDate::from_ymd_opt(2023, 12,24).unwrap());
    }

    // Business Day schedule test
    #[test]
    fn schedule_test() {
        let mut test_schedule: Vec<NaiveDate> = [].to_vec();
        let hol = NaiveDate::from_ymd_opt(2023,9 , 22).unwrap();
        // Create test vector with all the dates
        for i in 2 .. 31 {
            let dt = NaiveDate::from_ymd_opt(2023,9,i).unwrap();
            // Exclude weekends
            if dt.weekday() == Weekday::Sat || dt.weekday() == Weekday::Sun {} 
            // Include a Holiday
            else if dt == hol {}
            else {
                test_schedule.push(dt)
            }            
        }
    let setup: Setup = Setup::new();
    let mut cal: c::Calendar = setup.cal;
    cal.add_holidays(&[hol].into_iter().collect());
    let start_date: NaiveDate = NaiveDate::from_ymd_opt(2023,9,2).unwrap();
    let end_date: NaiveDate = NaiveDate::from_ymd_opt(2023,9,30).unwrap();
    let res: Vec<NaiveDate> = bus_day_schedule(&start_date, &end_date, &cal, Some(AdjustRule::ModFollowing));
    assert_eq!(test_schedule, res);
    }

    // Business Day count test
    #[test]
    fn bus_days_between_test() {
        let mut test_schedule: Vec<NaiveDate> = [].to_vec();
        let hol: NaiveDate = NaiveDate::from_ymd_opt(2023,9 , 22).unwrap();
        // Create test vector with all the dates
        for i in 1 .. 29 {
            let dt = NaiveDate::from_ymd_opt(2023,9,i).unwrap();
            // Exclude weekends
            if dt.weekday() == Weekday::Sat || dt.weekday() == Weekday::Sun {} 
            // Include a Holiday
            else if dt == hol {}
            else {
                test_schedule.push(dt)
            }            
        }
    let setup: Setup = Setup::new();
    let  mut cal: c::Calendar = setup.cal;  
    cal.add_holidays(&[hol].into_iter().collect());
    let start_date: NaiveDate = NaiveDate::from_ymd_opt(2023,9,1).unwrap();
    let end_date: NaiveDate = NaiveDate::from_ymd_opt(2023,9,29).unwrap();
    let res: u64 = business_days_between(&start_date, &end_date, &cal, Some(AdjustRule::Preceding));

    assert_eq!(test_schedule.len() as u64, res);
    
    }

    // Day count Fraction tests
    #[test]
    fn dcf_act360_test() {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;  
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        let expected: f64 = 0.6305556;
        let res: f64 = day_count_fraction(&start, &end
                                        , DayCount::Act360, None, None);
        // No calendar
        assert_eq!(round_decimals(res), round_decimals(expected) );    
        // With Calendar
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap(); // Adjusted to 02 Oct
        let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(); // Adjusted to 27 Dec
        let expected: f64 = 0.2388889;
        let res: f64 = day_count_fraction(&start, &end
                                        , DayCount::Act360, Some(&cal), Some(AdjustRule::Following));
        assert_eq!(round_decimals(res), round_decimals(expected));
    }

    #[test]
    fn dcf_act365_test() {
        let setup: Setup = Setup::new();
        let  cal: c::Calendar = setup.cal;  
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        let expected: f64 = 0.62191781;
        let res: f64 = day_count_fraction(&start, &end
                                        , DayCount::Act365, None, None);
        // No calendar
        assert_eq!(round_decimals(res), round_decimals(expected) );    
        // With Calendar
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap(); // Adjusted to 02 Oct
        let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(); // Adjusted to 27 Dec
        let expected: f64 = 0.23561644;
        let res: f64 = day_count_fraction(&start, &end
                                        , DayCount::Act365, Some(&cal), Some(AdjustRule::Following));
        assert_eq!(round_decimals(res), round_decimals(expected));
    }

    #[test]
    fn dcf_actactisda_test () {
        // The relevant test cases for this convention are when either or
        // both start date and end dates fall within a leap year.

        // Both dates within a leap year
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let start: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 5, 27).unwrap(); // This is a Monday within a Leap year
        let expected: f64 = 0.27868852;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::ActActISDA, Some(&cal), None);
        assert_eq!( round_decimals(res), round_decimals(expected) );

        // Both dates within a non-leap year
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 5, 27).unwrap(); // This will get adjusted to 29May2023
        let expected: f64 = 0.28219178;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::ActActISDA, Some(&cal), None);
        assert_eq!( round_decimals(res), round_decimals(expected) );

        // End date only within a leap year
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023, 2, 15).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 5, 27).unwrap(); // This is a Monday in a Leap Year
        let expected: f64 = 1.27835167;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::ActActISDA, Some(&cal), None);
        assert_eq!( round_decimals(res), round_decimals(expected) );

        // Start date and end dates within a leap year
        let start: NaiveDate = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap(); // This is a Saturday, will get adjusted to 2nd of March
        let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 5, 27).unwrap(); // This is a Monday in a Leap Year
        let expected: f64 = 4.23497268;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::ActActISDA, Some(&cal), None);
        assert_eq!( round_decimals(res), round_decimals(expected) );

    }

    #[test]
    fn dcf_d30360euro_test () {
        // Start date on the 31st
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023,1,31).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(); // This is a Monday within a Leap year
        let expected: f64 = 1.04166667;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::D30360Euro, Some(&cal), None);
        assert_eq!( round_decimals(res), round_decimals(expected) );
        // End date on the 31st
        let start: NaiveDate = NaiveDate::from_ymd_opt(2024,3,31).unwrap(); // Although this is a 31st, it is a Sunday so will get adjusted to Following first,
                                                                                             // since we are passing a calendar.
        let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 10, 31).unwrap(); 
        let expected: f64 = 0.5805556;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::D30360Euro, Some(&cal), None);
        assert_eq!( round_decimals(res), round_decimals(expected) );
        // Same dates but passing no calendar, i.e. no adjustment:
        let expected: f64 = 0.583333;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::D30360Euro, None, None);
        assert_eq!( round_decimals(res), round_decimals(expected) );

    }

    #[test]
    fn dcf_d30365_test () {
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023,1,24).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(); // This is a Monday within a Leap year
        let expected: f64 = 1.04383562;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::D30365, Some(&cal), None);
        assert_eq!( round_decimals(res), round_decimals(expected) );
    }

    #[test]
    fn dcf_bd252_test() {
        // For a Business Day Calendar, the relevant test cases should
        // of course take into account Holidays and check if the business
        // days are being properly counted.
        let setup: Setup = Setup::new();
        let cal: c::Calendar = setup.cal;
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023,1,24).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(); 
        let expected: f64 = 1.09126984;
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::Bd252, Some(&cal), None);
        assert_eq!( round_decimals(res), round_decimals(expected) );
        // Test case with an adjustment on the end date
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023,1,24).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 23).unwrap(); // This will get adjusted to the 27th of Dec
        let end2: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 27).unwrap(); // This is a business day so won't be adjusted        
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::Bd252, Some(&cal), None);
        let res2: f64 = day_count_fraction(&start, &end2
                , DayCount::Bd252, Some(&cal), None);
        // Business day count for both end dates above should be the same
        assert_eq!( round_decimals(res), round_decimals(res2) );
        // But if we pass a Preceding adjustment they should differ
        let res: f64 = day_count_fraction(&start, &end
            , DayCount::Bd252, Some(&cal), Some(AdjustRule::Preceding));
        let res2: f64 = day_count_fraction(&start, &end2
                , DayCount::Bd252, Some(&cal), Some(AdjustRule::Preceding));
        assert_ne!( round_decimals(res), round_decimals(res2) );
        let expected: f64 = 0.94444444;
        assert_eq!(round_decimals(res), round_decimals(expected));
    }

    #[test]
    #[should_panic]
    fn dcf_bd252_panic_test() {
        // A panic should occur since Bd252 is passed without a calendar
        let start: NaiveDate = NaiveDate::from_ymd_opt(2023,1,24).unwrap();
        let end: NaiveDate = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(); 
        let _res: f64 = day_count_fraction(&start, &end
            , DayCount::Bd252, None, None); 
    }



}
