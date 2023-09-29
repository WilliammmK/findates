//! Holiday Calendar objects.
//! Essentially, a list of dates that are not "business days". 
//! These can be National or Local holidays, but any other day there might be no settlement or trading. 
//! A work week can also be defined, with different weekdays as non-working days.

use std::collections::HashSet;
use chrono::Weekday;
use chrono::NaiveDate;


/// A Calendar representation.
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
    /// Get the holidays in the Calendar
    pub fn get_holidays(&self) -> &HashSet<NaiveDate> {
        return &self.holidays;
    }

    /// Get the weekend in the Calendar
    pub fn get_weekend(&self) -> &HashSet<Weekday> {
        return &self.weekend;
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

    
}





/// Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use chrono::{Weekday, NaiveDate};
    use crate::calendar::{self as c, Calendar};
    

    // Constructing a Basic UK calendar

    // add_holidays function test
    #[test]
    fn add_holidays_test() {
        let mut cal: c::Calendar = c::basic_calendar();
        let christmas_day = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023,12,26).unwrap();
        let new_holidays: HashSet<NaiveDate> =  [christmas_day, boxing_day].into_iter().collect();
        cal.add_holidays(&new_holidays);
        assert_eq!(cal.holidays, new_holidays);
    }

    // add_weekends function test
    #[test]
    fn add_weekends_test() {
        let mut cal: c::Calendar = c::Calendar::new();
        let new_weekend: HashSet<Weekday> = vec![Weekday::Mon].into_iter().collect();
        cal.add_weekends(&new_weekend);
        assert_eq!(cal.weekend, new_weekend);
    }

    // get_holidays function test
    #[test]
    fn get_holidays_test () {
        let mut cal: c::Calendar = c::basic_calendar();
        let christmas_day = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023,12,26).unwrap();
        let new_holidays: HashSet<NaiveDate> =  [christmas_day, boxing_day].into_iter().collect();
        cal.add_holidays(&new_holidays);
        let res = cal.get_holidays();
        assert_eq!(res, &new_holidays);
    }

    // get_weekend function test
    #[test]
    fn get_weekend_test () {
        let mut cal: c::Calendar = c::Calendar::new();
        let new_weekend: HashSet<Weekday> = vec![Weekday::Mon].into_iter().collect();
        cal.add_weekends(&new_weekend);
        let res = cal.get_weekend();
        assert_eq!(res, &new_weekend);

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
        assert_eq!(cal1, cal);
    }

    // Calendar intersection function test
    #[test]
    fn calendar_intersection_test() {
        let christmas_day: NaiveDate = NaiveDate::from_ymd_opt(2023,12,25).unwrap();
        let boxing_day: NaiveDate = NaiveDate::from_ymd_opt(2023,12,26).unwrap();
        let mut cal1: Calendar = c::Calendar {weekend: vec![Weekday::Sun].into_iter().collect()
                                        , holidays: [christmas_day].into_iter().collect()};
        let cal2: Calendar = c::Calendar {weekend: vec![Weekday::Sun].into_iter().collect()
                                        , holidays: [christmas_day,boxing_day].into_iter().collect()};

        let mut cal: c::Calendar = Calendar::new();
        let new_holidays: HashSet<NaiveDate> =  [christmas_day].into_iter().collect();
        cal.add_weekends(&[Weekday::Sun].into_iter().collect());
        cal.add_holidays(&new_holidays);

        cal1.intersection(&cal2);
        assert_eq!(cal1, cal);
    }





    


}

