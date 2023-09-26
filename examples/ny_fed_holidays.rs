use std::collections::HashSet;
use std::iter::Successors;

/// An example program used to first create a calendar with
/// the NY Federal reserve holidays and then using that calendar
/// to generate a payment schedule for a bond in USD.
/// 
/// The NY Federal Reserve Holidays as of 19th of September 2023
/// can be found on: https://www.newyorkfed.org/aboutthefed/holiday_schedule
/// and guidance on when Holidays that fall on a weekend will be observed can
/// be found on: https://www.federalpay.org/holidays
/// 

// Findates imports
use findates::algebra;
use findates::calendar::{Calendar};
use findates::conventions::*;
use findates::schedule::{self, Schedule};
// Chrono imports
use chrono::{Days,Months,Weekday,NaiveDate, Datelike};
use itertools::*;



fn main() {

// ========================================================================================================================
// In the U.S. there are Fixed date Holidays and Floating date Holidays.
// Both types can be created utilizing Schedule generators and then simply
// performing a calendar union so the full set of U.S. Holidays can be contained
// in a single calendar. The Federal legal holiday dates are https://www.law.cornell.edu/uscode/text/5/6103:

//     Fixed Date Holidays:

//     New Year's Day: January 1st
//     Juneteenth National Independence Day, June 19.
//     Independence Day: July 4th
//     Veterans Day: November 11th 
//     Christmas Day: December 25th

//     Floating Holidays:

//     Thanksgiving Day: The fourth Thursday in November
//     Labor Day: The first Monday in September
//     Columbus Day: The second Monday in October
//     Martin Luther King Jr. Day: The third Monday in January
//     Washington's Birthday: The third Monday in February
//     Memorial Day: The last Monday in May

// ========================================================================================================================


// Let's start by creating an empty calendar.
// We declare it mutable as we'll build it in parts.
let mut ny_fed_calendar: Calendar = Calendar::new();

// Saturdays and Sundays are non-business days there, so let's add those
// to the weekend field of our calendar:
let weekend: HashSet<Weekday> = [Weekday::Sat, Weekday::Sun].into_iter().collect();
ny_fed_calendar.add_weekends(&weekend);
println!("{:?}", ny_fed_calendar.weekend);

// Starting with the Fixed date holidays, let's create a schedule and an iterator for the NY's day:
let new_year_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
let new_year_schedule: Schedule = Schedule::new(Frequency::Annual, None, None);
let new_year_iterator = new_year_schedule.iter(new_year_day);

// We can now print out the next 5 New year dates:
println!("List of 5 NY dates: {:?}", new_year_iterator.take(4).collect::<Vec<NaiveDate>>());

// Great! But we don't need to explicitly create an iterator
// we just need the NY dates for the next 5 years, so we can just use the
// generate method.
let new_years: Vec<NaiveDate> = new_year_schedule.generate(&new_year_day, 
                                                    &new_year_day.checked_add_months(Months::new(48)).unwrap())
                                                    .expect("This should work");
println!("New Year days: {:?}", &new_years);

// Ok, but according to https://www.federalpay.org/holidays 
// Holidays that fall on a weekend should be observed on a Friday if they
// fall on a Saturday and on a Monday if they fall on a Sunday.
// To achieve that, we can make use of the Nearest Adjustment Rule:
let new_year_schedule: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let real_new_years: Vec<NaiveDate> = new_year_schedule.generate(&new_year_day, 
    &new_year_day.checked_add_months(Months::new(49)).unwrap())
    .expect("This should work");
println!("The actual observed days: {:?}", &real_new_years);

// Lets add these to our calendar:
// ny_fed_calendar.add_holidays(&real_new_years.into_iter().collect());


// It turns out that for the next four years NY's days will fall
// on Weekdays, so no adjustment was needed! Let's repeat this procedure
// for the rest of the fixed date holidays:
let independence_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 7,4).unwrap();
let independence_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let indep_days = independence_day_sch.generate(
                                                    &independence_day,
                                                       &independence_day.checked_add_months(Months::new(50)).unwrap()).unwrap();
println!("4th of july dates: {:?}", &indep_days);

let i_day: NaiveDate = NaiveDate::from_ymd_opt(2027, 7,4).unwrap();
let tes = algebra::adjust(&i_day, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
println!("************* tes: {:?}", &tes);
// ny_fed_calendar.add_holidays(&indep_days.into_iter().collect());

// Christmas day now.
let christmas_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 12,25).unwrap();
let christmas_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let christmas_days = christmas_day_sch.generate(
                                                    &christmas_day,
                                                       &christmas_day.checked_add_months(Months::new(48)).unwrap()).unwrap();
println!("Christmas dates: {:?}", &christmas_days);
// ny_fed_calendar.add_holidays(&christmas_days.into_iter().collect());

// println!("Holidays so far: {:?}", &ny_fed_calendar.get_holidays());

// And Veterans day.
let veterans_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 11,11).unwrap();
let veterans_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let veterans_days = veterans_day_sch.generate(
                                                    &veterans_day,
                                                       &veterans_day.checked_add_months(Months::new(50)).unwrap()).unwrap();
println!("Veteran days dates: {:?}", &veterans_days);

// And Juneteenth.
let juneteenth_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 06,19).unwrap();
let juneteenth_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let juneteenth_days = juneteenth_day_sch.generate(
                                                    &juneteenth_day,
                                                       &juneteenth_day.checked_add_months(Months::new(50)).unwrap()).unwrap();
println!("Juneteenth dates: {:?}", &juneteenth_days);

// Creating the floating holidays for a particular year
// is made easy using the functionalities from chrono and the use of closures.
// Let's start
let years = 2024 ..= 2027;
let thanksgiving_days: Vec<NaiveDate>;
thanksgiving_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 11, Weekday::Thu, 4).unwrap())
                                     .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                                     .collect();
println!("Thanksgiving dates: {:?}", &thanksgiving_days);

let labor_days: Vec<NaiveDate>;
labor_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 9, Weekday::Mon, 1).unwrap())
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("Labor Day dates: {:?}", &labor_days);

let columbus_days: Vec<NaiveDate>;
columbus_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 10, Weekday::Mon, 2).unwrap())
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("Columbus Day dates: {:?}", &columbus_days);

let mlkjr_days: Vec<NaiveDate>;
mlkjr_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 1, Weekday::Mon, 3).unwrap())
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("MLK Jr Day dates: {:?}", &mlkjr_days);

let washington_days: Vec<NaiveDate>;
washington_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 2, Weekday::Mon, 3).unwrap())
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("Washington's birthdate dates: {:?}", &washington_days);

// For memorial day we use a little trick with Weekdays
fn last_monday_of_may (year: i32) -> NaiveDate {
    let may_31st = NaiveDate::from_ymd_opt(year, 5, 31).unwrap();
    let delta = may_31st.weekday().num_days_from_monday() ;
    return may_31st - chrono::Duration::days(delta as i64);
}

let memorial_days: Vec<NaiveDate>;
memorial_days = years.clone().map(|x| last_monday_of_may(x))
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("Memorial Day dates: {:?}", &memorial_days);

// Now let's add all of those dates to our calendar.
let mut all_holidays = itertools::concat([real_new_years
                                                    ,christmas_days
                                                    ,indep_days
                                                    ,veterans_days
                                                    ,juneteenth_days
                                                    ,thanksgiving_days
                                                    ,labor_days
                                                    ,columbus_days
                                                    ,mlkjr_days
                                                    ,washington_days
                                                    ,memorial_days]);
all_holidays.sort();
println!("Holiday Calendar: {:?}", & all_holidays);

ny_fed_calendar.add_holidays(&all_holidays.into_iter().collect());

// Now let's see how our calendar looks:
println!("NY Federal Reserve Holiday Calendar: {:?}", & ny_fed_calendar);



}