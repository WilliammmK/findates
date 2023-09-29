// Integration test recreating the U.S. Federal Holiday calendar.
// National Holidays: https://www.law.cornell.edu/uscode/text/5/6103
// NY Federal Reserve published calendar: https://www.frbservices.org/about/holiday-schedules.

use std::collections::HashSet;

use chrono::{NaiveDate, Weekday, Datelike};
use findates::algebra;
use findates::calendar::Calendar;
use findates::schedule::Schedule;
use findates::conventions::{Frequency, AdjustRule, DayCount};

// The setup function ouputs the calendar and schedule of dates
// that will later be used in the different test functions.
pub fn calendar_setup () -> Calendar {

    // Empty Calendar
    let mut ny_fed_calendar: Calendar = Calendar::new();

    // Adding weekends
    let weekend: HashSet<Weekday> = [Weekday::Sat, Weekday::Sun].into_iter().collect();
    ny_fed_calendar.add_weekends(&weekend);

    // Calculated all holiday dates for the next ten years and add them to the calendar
    // New Years
    let new_year_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let new_year_schedule: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
    let new_years: Vec<NaiveDate> = new_year_schedule.generate(&new_year_day, 
                                                              &algebra::checked_add_years(&new_year_day, 10).unwrap()).unwrap();
    
    // 4th of July
    let independence_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 7,4).unwrap();
    let independence_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
    let indep_days = independence_day_sch.generate(
                                                        &independence_day,
                                                        &algebra::checked_add_years(&independence_day, 10).unwrap()).unwrap();

    // Christmas 
    let christmas_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 12,25).unwrap();
    let christmas_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
    let christmas_days = christmas_day_sch.generate(
                                                        &christmas_day,
                                                        &algebra::checked_add_years(&christmas_day, 10).unwrap()).unwrap();

    // Veterans day
    let veterans_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 11,11).unwrap();
    let veterans_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
    let veterans_days = veterans_day_sch.generate(
                                                        &veterans_day,
                                                        &algebra::checked_add_years(&veterans_day, 10).unwrap()).unwrap();

    // Juneteenth
    let juneteenth_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 06,19).unwrap();
    let juneteenth_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
    let juneteenth_days = juneteenth_day_sch.generate(
                                                        &juneteenth_day,
                                                        &algebra::checked_add_years(&juneteenth_day, 10).unwrap()).unwrap();

    // Thanksgiving
    let years = 2023 ..= 2033;
    let thanksgiving_days: Vec<NaiveDate>;
    thanksgiving_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 11, Weekday::Thu, 4).unwrap())
                                        .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                                        .collect();
    
    // Labor day
    let labor_days: Vec<NaiveDate>;
    labor_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 9, Weekday::Mon, 1).unwrap())
                                .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                                .collect();
    
    // Columbus day
    let columbus_days: Vec<NaiveDate>;
    columbus_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 10, Weekday::Mon, 2).unwrap())
                                .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                                .collect();

    // Martin Luther King day
    let mlkjr_days: Vec<NaiveDate>;
    mlkjr_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 1, Weekday::Mon, 3).unwrap())
                                .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                                .collect();

    // Washington day
    let washington_days: Vec<NaiveDate>;
    washington_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 2, Weekday::Mon, 3).unwrap())
                                .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                                .collect();

    // Memorial day
    fn last_monday_of_may (year: i32) -> NaiveDate {
        let may_31st = NaiveDate::from_ymd_opt(year, 5, 31).unwrap();
        let delta = may_31st.weekday().num_days_from_monday() ;
        return may_31st - chrono::Duration::days(delta as i64);
    }
    
    let memorial_days: Vec<NaiveDate>;
    memorial_days = years.clone().map(|x| last_monday_of_may(x))
                                   .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                                   .collect();

    // Adding all Holidays to the calendar
    // Now let's add all of those dates to our calendar.
    let all_holidays:HashSet<NaiveDate> = itertools::concat([ new_years
                                                                    ,christmas_days
                                                                    ,indep_days
                                                                    ,veterans_days
                                                                    ,juneteenth_days
                                                                    ,thanksgiving_days
                                                                    ,labor_days
                                                                    ,columbus_days
                                                                    ,mlkjr_days
                                                                    ,washington_days
                                                                    ,memorial_days]).into_iter().collect();


    ny_fed_calendar.add_holidays(&all_holidays);

    


    return ny_fed_calendar;

}


// Payment dates for the 10 year U.S. Treasury Note
// https://www.treasurydirect.gov/instit/annceresult/press/preanre/2023/A_20230802_2.pdf
pub fn payment_schedule_setup (calendar: &Calendar) -> (Vec<NaiveDate>, Vec<f64>, Vec<NaiveDate>) {

    // Issue and maturity date
    let issue_date = NaiveDate::from_ymd_opt(2023,8,15).unwrap();
    let maturity_date = NaiveDate::from_ymd_opt(2033, 8, 15).unwrap();

    // Coupon dates
    let coupon_schedule = Schedule::new(Frequency::Semiannual, None, None);
    let coupon_dates = coupon_schedule.generate(&issue_date, &maturity_date);
    let coupon_dates_list: Vec<NaiveDate> = coupon_dates.unwrap().into_iter().collect();
    
    // Calculate day count fractions
    let mut dcfs: Vec<f64> = vec![  ];
    for i in 0 .. (coupon_dates_list.len() - 1) {
        let dcf = algebra::day_count_fraction(coupon_dates_list.get(i).unwrap(),
                                                coupon_dates_list.get(i + 1).unwrap(), DayCount::D30360Euro, None, None);
        dcfs.push(dcf);
    }

    // Adjust for actual settlement dates
    let settlement_dates: Vec<NaiveDate> = coupon_dates_list.clone().into_iter()
                                                        .map(|x| algebra::adjust(&x, Some(&calendar), Some(AdjustRule::Following)))
                                                        .collect();


    
    return (coupon_dates_list, dcfs, settlement_dates);
}


