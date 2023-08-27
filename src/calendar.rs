//! Holiday Calendar objects. Essentially a list of dates that are not
//! "business days". These can be National or Local holidays usually,
//! but any other day there might be no settlement or trading.

use std::collections::HashSet;
use std::hash::Hasher;
use std::hash::Hash;
use std::iter::Map;


use chrono::Datelike;
use chrono::Weekday;
use chrono::NaiveDate;

use itertools::Itertools;

/// A Calendar representation.
/// Essentially a list of dates that are not
/// "business days". These can be National or Local holidays usually,
/// but any other day there might be no settlement or trading.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Calendar {
    pub weekend:   HashSet<Weekday>,          // Which weekdays are not good working days
    pub holidays:  HashSet<NaiveDate>,        // Which days of the year are not good working days
    
}


/// Creating a basic calendar with Saturdays and Sundays as weekend.
pub fn basic_calendar() -> Calendar {
    let res: Calendar = Calendar { weekend: vec![ Weekday::Sat
                                                , Weekday::Sun ].into_iter().collect()
                                 , holidays: vec![].into_iter().collect() };
    return res;
}

impl Calendar {
    /// Add Holidays to a calendar
    pub fn add_holidays (mut self, holidays: &HashSet<NaiveDate>) {       
        self.holidays.union(holidays);

    }
    
    /// Calendar Union
    pub fn calendar_union (self, calendar: &Calendar) {
        self.holidays.union(&calendar.holidays);
        self.weekend.union(&calendar.weekend);

    }
    


    /// Given a calendar, propagate the current Holiday list until the given target date.
    /// The function will simply take the Day and month of each holiday in the current list
    /// and replace the year for every year until the target date.
    /// Useful if only a list of holidays for the current year is available.
    /// If the target date is before the earliest holiday date, it back propagate the calendar.
    pub fn holiday_propagate (mut self, target_date: NaiveDate) {
        // If no holidays in the calendar just return the calendar itself.
        if self.holidays.is_empty() { return }
        
        else {
            let target_year = target_date.year();
            let hols_years = self.holidays.iter().map(|x: &NaiveDate| {x.year()});
            let mut days_and_months =self.holidays.iter()
                                                                    .map(|d: &NaiveDate| {(d.day(),d.month())});
            let min_year: i32;
            min_year = hols_years.min().unwrap_or(target_year);

            // New holiday vector
            let new_holidays: Vec<NaiveDate>;
            
            // Ok to just unwrap here as empty list has been already checked in the first if.
            // Back propagate
            if min_year > target_year {
                let years: std::ops::Range<i32> = target_year .. min_year;
                


            } 
            // Propagate
            else if min_year == target_year{
    
            }

        }

        
        
        self.holidays = [].to_vec();        


    return;
}


    
}





/// Check if a date is a good business day in a given calendar.
pub fn is_business_day (date: NaiveDate, calendar: &Calendar) -> bool {
    if calendar.weekend.contains(&date.weekday()) {
        return false;
    } else if calendar.holidays.contains(&date) {
        return false;
    } else {
        return true;    
    }
}



/// Tests
#[cfg(test)]
mod tests {
    use chrono::{Weekday, NaiveDate};
    use crate::calendar;
    
    struct Setup {
        basic_calendar: calendar::Calendar,

    }

    impl Setup {
    // A simple calendar with only Saturdays and Sundays as non-business days.
        fn  new() -> Self {
            Self { basic_calendar : calendar::Calendar { weekend: vec![ Weekday::Sat
                                                                      , Weekday::Sun ]
                                                       , holidays: vec![] }
            }
        }
    }

    // Constructing a Basic UK calendar


    
    
    #[test]
    fn is_business_day_test() {
        let basic_cal = Setup::new().basic_calendar;
        let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Sun);
        assert_eq!(false, calendar::is_business_day(my_date.unwrap(), &basic_cal));
        let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Mon);
        assert_eq!(true, calendar::is_business_day(my_date.unwrap(), &basic_cal));
        
    }


    


}

