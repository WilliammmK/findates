//! Holiday Calendar objects. Essentially a list of dates that are not
//! "business days". These can be National or Local holidays usually,
//! but any other day there might be no settlement or trading.

use std::collections::HashSet;
// use std::hash::Hasher;
// use std::hash::Hash;
// use std::iter::Map;


use chrono::Datelike;
use chrono::Weekday;
use chrono::NaiveDate;

// use itertools::Itertools;

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

/// Union between a list of calendars
pub fn calendar_unions (calendars: &Vec<Calendar>) -> Calendar {
    let mut result_cal = Calendar::new();
    calendars.iter().for_each(|cal| {
        result_cal.union(cal)
    });

    return result_cal;
}

impl Calendar {
    /// Construct a new empty calendar
    pub fn new() -> Self {
        Self { weekend: HashSet::new(), holidays: HashSet::new() }
    }

    pub fn get_holidays(self) -> HashSet<NaiveDate> {
        return self.holidays;
    }

    /// Add Holidays to the calendar
    pub fn add_holidays (&mut self, holidays: &HashSet<NaiveDate>) {       
        self.holidays = self.holidays.union(holidays).cloned().collect();

    }

    /// Add Weekends to the calendar
    pub fn add_weekends (&mut self, weekends: &HashSet<Weekday>) {
        self.weekend = self.weekend.union(weekends).cloned().collect();
    }
    
    /// Calendar Union
    pub fn union (&mut self, calendar: &Calendar) {
        self.holidays = self.holidays.union(&calendar.holidays).cloned().collect();
        self.weekend = self.weekend.union(&calendar.weekend).cloned().collect();

    }

    /// Calendar Intersection
    pub fn intersection (&mut self, calendar: &Calendar) {
        self.holidays = self.holidays.intersection(&calendar.holidays).cloned().collect();
        self.weekend = self.weekend.intersection(&calendar.weekend).cloned().collect();

    }


    /// Given a calendar, propagate the current Holiday list until the given target date.
    /// The function will simply take the Day and month of each holiday in the current list
    /// and replace the year for every year until the target date.
    /// Useful if only a list of holidays for the current year is available.
    /// If the target date is before the earliest holiday date, it back propagate the calendar.
    pub fn holiday_propagate (&mut self, target_date: NaiveDate) {
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

        
        
        self.holidays = HashSet::new();        


    return;
}


    
}





/// Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use chrono::{Weekday, NaiveDate};
    use itertools::Itertools;
    use crate::calendar::{self as c, Calendar};
    
    struct Setup {
        empty_calendar: c::Calendar,
        basic_calendar: c::Calendar,

    }

    impl Setup {
    // A simple calendar with only Saturdays and Sundays as non-business days.
        fn  new() -> Self {

            Self {  empty_calendar: c::Calendar::new() ,
                    basic_calendar : c::basic_calendar()
            }
        }
    }

    // Constructing a Basic UK calendar

    // add_holidays function test
    #[test]
    fn add_holidays_test() {
        let mut cal: c::Calendar = c::basic_calendar();
        let christmas_day = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023,12,26).unwrap();
        let new_holidays: HashSet<NaiveDate> =  [christmas_day, boxing_day].into_iter().collect();
        cal.add_holidays(&new_holidays);

        println!("{:?}",cal.holidays);
        assert_eq!(cal.holidays, new_holidays);
    }

    // add_weekends function test
    #[test]
    fn add_weekends_test() {
        let mut cal: c::Calendar = c::Calendar::new();
        let new_weekend: HashSet<Weekday> = vec![Weekday::Mon].into_iter().collect();
        cal.add_weekends(&new_weekend);
        println!("{:?}", cal.weekend);
        assert_eq!(cal.weekend, new_weekend);
    }

    // Calendar union function test
    #[test]
    fn calendar_union_test() {
        let christmas_day = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023,12,26).unwrap();
        let mut cal1: Calendar = c::Calendar {weekend: vec![Weekday::Sat].into_iter().collect()
                                        , holidays: [christmas_day].into_iter().collect()};
        let cal2: Calendar = c::Calendar {weekend: vec![Weekday::Sun].into_iter().collect()
                                        , holidays: [boxing_day].into_iter().collect()};

        let mut cal: c::Calendar = c::basic_calendar();
        let new_holidays: HashSet<NaiveDate> =  [christmas_day, boxing_day].into_iter().collect();
        cal.add_holidays(&new_holidays);


        cal1.union(&cal2);
        println!("{:?}", cal1);
        assert_eq!(cal1, cal);
    }

    // Calendar intersection function test
    #[test]
    fn calendar_intersection_test() {
        let christmas_day = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023,12,26).unwrap();
        let mut cal1: Calendar = c::Calendar {weekend: vec![Weekday::Sun].into_iter().collect()
                                        , holidays: [christmas_day].into_iter().collect()};
        let cal2: Calendar = c::Calendar {weekend: vec![Weekday::Sun].into_iter().collect()
                                        , holidays: [christmas_day,boxing_day].into_iter().collect()};

        let mut cal: c::Calendar = Calendar::new();
        let new_holidays: HashSet<NaiveDate> =  [christmas_day].into_iter().collect();
        cal.add_weekends(&[Weekday::Sun].into_iter().collect());
        cal.add_holidays(&new_holidays);


        cal1.intersection(&cal2);
        println!("{:?}", cal1);
        assert_eq!(cal1, cal);
    }





    


}

