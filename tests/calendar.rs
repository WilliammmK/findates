

// #[cfg(test)]
// mod calendar_tests {
//     use chrono::{Weekday, NaiveDate};
//     use findates::calendar;


//     struct Setup {
//         basic_calendar: calendar::Calendar,

//     }

//     impl Setup {
//     // A simple calendar with only Saturdays and Sundays as non-business days.
//         fn  new() -> Self {
//             Self { basic_calendar : calendar::Calendar { weekend: vec![ Weekday::Sat
//                                                                       , Weekday::Sun ]
//                                                        , holidays: vec![] }
//             }
//         }
//     }
    
    
//     #[test]
//     fn check_date() {
//         let basic_cal = Setup::new().basic_calendar;
//         let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Sun);
//         assert_eq!(false, calendar::is_business_day(my_date.unwrap(), &basic_cal));
//         let my_date: Option<NaiveDate> = NaiveDate::from_isoywd_opt(2015, 10, Weekday::Mon);
//         assert_eq!(true, calendar::is_business_day(my_date.unwrap(), &basic_cal));
//     }



// }