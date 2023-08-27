//! Holiday Calendar objects. Essentially a list of dates that are not
//! "business days". These can be National or Local holidays usually,
//! but any other day there might be no settlement or trading.

use chrono::Datelike;
use chrono::Weekday;
use chrono::NaiveDate;

/// A basic calendar with Saturday and Sunday as non-working days.
/// 
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Calendar {
    pub weekend:   Vec<Weekday>,          // Which weekdays are not good working days
    pub holidays:  Vec<NaiveDate>,        // Which days of the year are not good working days
    pub propagate: Option<NaiveDate>      // Use the given holidays dates to propagate until the given date
}

/// Creating a basic calendar with Saturdays and Sundays as weekend.
pub fn basic_calendar() -> Calendar {
    let res: Calendar = Calendar { weekend: vec![ Weekday::Sat
                                                , Weekday::Sun ]
                                 , holidays: vec![]
                                 , propagate: None };
    return res;
}

/// Given a calendar, propagate the current Holiday list until the given target date.
/// The function will simply take the Day and month of each holiday in the current list
/// and replace the year for every year until the target date.
/// Useful if only a list of holidays for the current year is available.
/// If the target date is before the earliest holiday date, it back propagate the calendar.
pub fn holiday_propagate (cal: Calendar, anchor_date: NaiveDate, target_date: NaiveDate) -> Calendar {

    return basic_calendar();
}

/// Calendar Union
pub fn calendar_union (cal1: Calendar, cal2: Calendar) -> Calendar {
    //let cal1
    return basic_calendar();
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

