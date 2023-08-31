use chrono::{NaiveDate, Datelike};

use crate::calendar::Calendar;
use crate::calendar as c;





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
