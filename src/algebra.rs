use std::ops::Add;


use chrono::{NaiveDate, Datelike, Days};

use crate::calendar::Calendar;
use crate::calendar as c;
use crate::conventions as conv;


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

impl Date {

}
/// Adjust a date to a business day according to a Calendar and a AdjustRule
pub fn adjust (date: &NaiveDate, calendar: &Calendar, adjust_rule: Option<conv::AdjustRule>) -> NaiveDate {
    match adjust_rule {
        None                                => return *date,
        Some(conv::AdjustRule::Unadjusted)  => return *date,
        Some(conv::AdjustRule::Following)   => {
            if is_business_day(date, calendar) {
                return *date;
            } else {
                let mut adj_date: NaiveDate = date.checked_add_days(Days::new(1)).unwrap(); // add_days function does not modify the original date
                loop {
                    if is_business_day(&adj_date, calendar) {
                        break;
                    } else {
                        adj_date = date.checked_add_days(Days::new(1)).unwrap();
                    }
                }
                return adj_date;
            }
        },

        Some(conv::AdjustRule::ModFollowing)  => return *date, // !!! Stub
        Some(conv::AdjustRule::Preceding)  => return *date, // !!! Stub
        Some(conv::AdjustRule::ModPreceding)  => return *date, // !!! Stub
        Some(conv::AdjustRule::HalfMonthModFollowing)  => return *date, // !!! Stub
        Some(conv::AdjustRule::Nearest)  => return *date, // !!! Stub

    } 


}


/// Tests
#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use chrono::Weekday;
    use crate::calendar as c;
    use crate::algebra as a;

    // Is business day function test.
    #[test]
    fn is_business_day_test() {
        let basic_cal = c::basic_calendar();
        let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Sun);
        assert_eq!(false, a::is_business_day(&my_date.unwrap(), &basic_cal));
        let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Mon);
        assert_eq!(true, a::is_business_day(&my_date.unwrap(), &basic_cal));
        
    }


}
