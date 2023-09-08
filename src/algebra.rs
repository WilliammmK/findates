use std::ops::Add;
use std::ops::Sub;


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
/// This function returns a new NaiveDate without modifying the input.
pub fn adjust (date: &NaiveDate, calendar: &Calendar, adjust_rule: Option<conv::AdjustRule>) -> NaiveDate {
    // If it is a good day, just return it.
    if is_business_day(date, calendar) {
        return date.clone();
    } else {
        let adj_date: NaiveDate;
        match adjust_rule {
            None                                => return date.clone(),

            Some(conv::AdjustRule::Unadjusted)  => return date.clone(),

            Some(conv::AdjustRule::Following)   => {
                return add_adjust(date, calendar);
            },
    
            Some(conv::AdjustRule::ModFollowing)  => {
                adj_date = add_adjust(date, calendar);
                if adj_date.month() != date.month() {
                    return sub_adjust(date, calendar);
                } else {
                    return adj_date;
                }
            },
    
            Some(conv::AdjustRule::Preceding)  => {
                return sub_adjust(date, calendar);
            }
            
            Some(conv::AdjustRule::ModPreceding)  => {
                adj_date = sub_adjust(date, calendar);
                if adj_date.month() != date.month() {
                    return add_adjust(date, calendar);
                } else {
                    return adj_date;
                }
            }

            Some(conv::AdjustRule::HalfMonthModFollowing)  => {
                adj_date = add_adjust(date, calendar);
                if adj_date.month() != date.month() {
                    return sub_adjust(date, calendar);
                } else if date.day() <= 15 && adj_date.day() > 15  {
                    return sub_adjust(date, calendar);
                } else {
                    return adj_date;
                }
            }

            Some(conv::AdjustRule::Nearest)  => {
                let follow_date: NaiveDate = add_adjust(date, calendar);
                let prec_date: NaiveDate   = sub_adjust(date, calendar);
                if (follow_date - *date) <= (prec_date - *date) {
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


/// Tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::str::FromStr;
    use chrono::Datelike;
    use chrono::NaiveDate;
    use chrono::Weekday;
    use chrono::Days;
    use crate::calendar as c;
    use crate::algebra as a;
    use crate::conventions::AdjustRule;
    
    

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



    #[test]
    fn adjust_following_test() {
        // !!! implement = 
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
        cal.add_holidays(&[NaiveDate::from_ymd_opt(2023, 4, 14).unwrap()].into_iter().collect());
        let eom: NaiveDate = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
        let mom: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 14).unwrap(); // This is a Saturday
        //let boy: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(a::adjust(&setup.test_weekend, &cal, Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 9,4).unwrap());
        //assert_eq!(a::adjust(&mom, &cal, Some(AdjustRule::HalfMonthModFollowing)), NaiveDate::from_ymd_opt(2023, 9,1).unwrap());
        //assert_eq!(a::adjust(&boy, &cal, Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 1,2).unwrap());
        //assert_eq!(a::adjust(&NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(), &cal, Some(AdjustRule::ModPreceding)), NaiveDate::from_ymd_opt(2023, 2,2).unwrap());
    }



}
