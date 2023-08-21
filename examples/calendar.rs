
use findates::calendar::Calendar;
use chrono::Weekday;
use chrono::NaiveDate;
use findates::calendar::is_business_day;


fn main() {

    // A simple calendar with only Saturdays and Sundays as non-business days.
    let basic_calendar: Calendar = Calendar { weekend: vec![ Weekday::Sat
                                                                     , Weekday::Sun ]
                                                      , holidays: vec![] };

    let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Sun);
    dbg!("{}", is_business_day(my_date.unwrap(), &basic_calendar));    
    
}

